//! Healthpoint CLI entrypoint.

use std::{fs::File, path::{Path, PathBuf}};

use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand, ValueEnum};
use healthpoint_client::{
    parse_auth_scheme, parse_geo_search_mode, ClientConfig, HealthpointClient,
};
use healthpoint_core::{
    AccessPolicy, Code, DirectoryProvider, GeoPoint, HealthpointResourceUri, LocationRecord,
    OrganizationRecord, Page, QueryLimit, ServiceQuery, ServiceRecord,
};
use healthpoint_export::{ExportManifest, ServiceExportFormat};
use healthpoint_testkit::FixtureDirectoryProvider;
use schemars::schema_for;
use url::Url;

#[derive(Debug, Parser)]
#[command(name = "healthpoint", version, about = "Rust CLI for the Healthpoint HL7 FHIR API")]
struct Cli {
    #[arg(long, env = "HEALTHPOINT_BASE_URL", default_value = "https://www.healthpointapi.com/")]
    base_url: String,

    #[arg(long, env = "HEALTHPOINT_API_KEY", hide_env_values = true)]
    api_key: Option<String>,

    #[arg(long, env = "HEALTHPOINT_AUTH_SCHEME", default_value = "bearer")]
    auth_scheme: String,

    #[arg(long, env = "HEALTHPOINT_GEO_SEARCH_MODE", default_value = "healthpoint-lat-lon")]
    geo_search_mode: String,

    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Show redacted configuration and environment readiness.
    Doctor,
    /// Search Healthpoint resources.
    Search {
        #[command(subcommand)]
        command: SearchCommand,
    },
    /// Get a single Healthpoint resource.
    Get {
        #[command(subcommand)]
        command: GetCommand,
    },
    /// Inspect generated requests without sending them.
    Inspect {
        #[command(subcommand)]
        command: InspectCommand,
    },
    /// Export local data and manifests.
    Export {
        #[command(subcommand)]
        command: ExportCommand,
    },
    /// Work with synthetic fixtures; never touches the live API.
    Fixture {
        #[command(subcommand)]
        command: FixtureCommand,
    },
    /// Print JSON Schemas for integration contracts.
    Schema {
        /// Schema target.
        #[arg(value_enum)]
        target: SchemaTarget,
    },
    /// Show access/export policy metadata.
    Policy {
        #[command(subcommand)]
        command: PolicyCommand,
    },
    /// Explain how to launch the MCP server binary.
    Mcp,
}

#[derive(Debug, Subcommand)]
enum SearchCommand {
    /// Search FHIR HealthcareService resources.
    Services(ServiceSearchArgs),
}

#[derive(Debug, Args)]
struct ServiceSearchArgs {
    #[command(flatten)]
    query: ServiceQueryArgs,

    /// Output format.
    #[arg(long, value_enum, default_value = "human")]
    format: OutputFormat,
}

#[derive(Debug, Clone, Args)]
struct ServiceQueryArgs {
    /// Full-text search term.
    #[arg(long)]
    text: Option<String>,

    /// FHIR category token. Repeatable.
    #[arg(long = "category")]
    categories: Vec<String>,

    /// FHIR type token, e.g. `http://snomed.info/sct|171149006`. Repeatable.
    #[arg(long = "type")]
    service_types: Vec<String>,

    /// Convenience SNOMED CT service-type code. Repeatable.
    #[arg(long = "snomed")]
    snomed_types: Vec<String>,

    /// FHIR specialty token. Repeatable.
    #[arg(long = "specialty")]
    specialties: Vec<String>,

    /// Latitude for nearby search.
    #[arg(long)]
    lat: Option<f64>,

    /// Longitude for nearby search.
    #[arg(long)]
    lon: Option<f64>,

    /// Radius in kilometres for nearby search.
    #[arg(long)]
    radius_km: Option<f32>,

    /// Pagination cursor from the previous page's `next_cursor`.
    #[arg(long)]
    cursor: Option<String>,

    /// Maximum records to return. Clamped to 1..100.
    #[arg(long, default_value_t = 25)]
    limit: u16,
}

#[derive(Debug, Subcommand)]
enum GetCommand {
    /// Get a HealthcareService by id.
    Service(GetArgs),
    /// Get an Organization by id.
    Organization(GetArgs),
    /// Get a Location by id.
    Location(GetArgs),
    /// Get any supported resource through a healthpoint:// URI.
    Uri(UriGetArgs),
}

#[derive(Debug, Args)]
struct GetArgs {
    /// Resource id.
    id: String,

    /// Output format.
    #[arg(long, value_enum, default_value = "json")]
    format: OutputFormat,
}

#[derive(Debug, Args)]
struct UriGetArgs {
    /// Resource URI, e.g. healthpoint://service/<id>.
    uri: String,

    /// Output format.
    #[arg(long, value_enum, default_value = "json")]
    format: OutputFormat,
}

#[derive(Debug, Subcommand)]
enum InspectCommand {
    /// Build a HealthcareService search URL without sending it.
    SearchUrl(ServiceQueryArgs),
    /// Build a resource URL without sending it.
    ResourceUrl {
        /// FHIR resource type, e.g. HealthcareService, Location, Organization.
        resource_type: String,
        /// FHIR id.
        id: String,
    },
}

#[derive(Debug, Subcommand)]
enum ExportCommand {
    /// Write an export manifest only; does not query Healthpoint.
    Manifest {
        /// Output path.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Query services and write a local export plus manifest sidecar.
    Services(ExportServicesArgs),
}

#[derive(Debug, Args)]
struct ExportServicesArgs {
    #[command(flatten)]
    query: ServiceQueryArgs,

    /// Output data path. Extension does not control format; use --format explicitly.
    #[arg(long)]
    output: PathBuf,

    /// Optional manifest path. Defaults to `<output>.manifest.json`.
    #[arg(long)]
    manifest: Option<PathBuf>,

    /// Data export format.
    #[arg(long, value_enum, default_value = "jsonl")]
    format: DataExportFormat,
}

#[derive(Debug, Subcommand)]
enum FixtureCommand {
    /// Print the synthetic HealthcareService fixture after typed mapping.
    Services {
        /// Output format.
        #[arg(long, value_enum, default_value = "json")]
        format: OutputFormat,
    },
    /// Print the synthetic Location fixture after typed mapping.
    Location,
    /// Print the synthetic Organization fixture after typed mapping.
    Organization,
}

#[derive(Debug, Subcommand)]
enum PolicyCommand {
    /// Show the default conservative access policy.
    Show,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum OutputFormat {
    Human,
    Json,
    Jsonl,
    Csv,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum DataExportFormat {
    Json,
    Jsonl,
    Csv,
}

impl From<DataExportFormat> for ServiceExportFormat {
    fn from(value: DataExportFormat) -> Self {
        match value {
            DataExportFormat::Json => Self::Json,
            DataExportFormat::Jsonl => Self::Jsonl,
            DataExportFormat::Csv => Self::Csv,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum SchemaTarget {
    AccessPolicy,
    ExportManifest,
    ServiceQuery,
    ServiceRecord,
    ServicePage,
    LocationRecord,
    OrganizationRecord,
    ResourceUri,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    let cli = Cli::parse();
    let client = build_client(&cli)?;

    match cli.command {
        Command::Doctor => {
            println!("{}", serde_json::to_string_pretty(&client.diagnostic_status())?);
        }
        Command::Search {
            command: SearchCommand::Services(args),
        } => {
            let query = args.query.to_query()?;
            let page = client.search_services(query).await?;
            print_service_page(&page, args.format)?;
        }
        Command::Get {
            command: GetCommand::Service(args),
        } => {
            let service = client.get_service(&args.id).await?;
            print_services(&[service], args.format)?;
        }
        Command::Get {
            command: GetCommand::Organization(args),
        } => {
            let org = client.get_organization(&args.id).await?;
            print_organization(&org, args.format)?;
        }
        Command::Get {
            command: GetCommand::Location(args),
        } => {
            let location = client.get_location(&args.id).await?;
            print_location(&location, args.format)?;
        }
        Command::Get {
            command: GetCommand::Uri(args),
        } => {
            match HealthpointResourceUri::parse(&args.uri)? {
                HealthpointResourceUri::Service(id) => {
                    let service = client.get_service(&id).await?;
                    print_services(&[service], args.format)?;
                }
                HealthpointResourceUri::Location(id) => {
                    let location = client.get_location(&id).await?;
                    print_location(&location, args.format)?;
                }
                HealthpointResourceUri::Organization(id) => {
                    let organization = client.get_organization(&id).await?;
                    print_organization(&organization, args.format)?;
                }
            }
        }
        Command::Inspect { command } => match command {
            InspectCommand::SearchUrl(args) => {
                let query = args.to_query()?;
                println!("{}", client.service_search_url(&query)?);
            }
            InspectCommand::ResourceUrl { resource_type, id } => {
                println!("{}", client.inspect_resource_url(&resource_type, &id)?);
            }
        },
        Command::Export {
            command: ExportCommand::Manifest { output },
        } => {
            let manifest = ExportManifest::new(
                healthpoint_core::SourceProvenance::healthpoint(cli.base_url),
                false,
            );
            write_manifest(&manifest, output.as_deref())?;
        }
        Command::Export {
            command: ExportCommand::Services(args),
        } => {
            let query = args.query.to_query()?;
            let page = client.search_services(query).await?;
            create_parent_dir(&args.output)?;
            let file = File::create(&args.output)
                .with_context(|| format!("failed to create {}", args.output.display()))?;
            healthpoint_export::write_services(&page.items, args.format.into(), file)?;
            let manifest_path = args
                .manifest
                .clone()
                .unwrap_or_else(|| sidecar_manifest_path(&args.output));
            let manifest = ExportManifest::new(page.provenance, true);
            write_manifest(&manifest, Some(manifest_path.as_path()))?;
            eprintln!("wrote {} records to {}", page.items.len(), args.output.display());
            eprintln!("wrote manifest to {}", manifest_path.display());
        }
        Command::Fixture { command } => {
            let provider = FixtureDirectoryProvider::new();
            match command {
                FixtureCommand::Services { format } => {
                    let page = provider.search_services(ServiceQuery::default()).await?;
                    print_service_page(&page, format)?;
                }
                FixtureCommand::Location => {
                    let location = provider.get_location("loc-auckland-clinic-1").await?;
                    println!("{}", serde_json::to_string_pretty(&location)?);
                }
                FixtureCommand::Organization => {
                    let organization = provider.get_organization("org-example-provider-1").await?;
                    println!("{}", serde_json::to_string_pretty(&organization)?);
                }
            }
        }
        Command::Schema { target } => print_schema(target)?,
        Command::Policy {
            command: PolicyCommand::Show,
        } => {
            println!("{}", serde_json::to_string_pretty(&AccessPolicy::default())?);
        }
        Command::Mcp => {
            println!("Run the read-only MCP server with: cargo run -p healthpoint-mcp");
            println!("For installed binaries: healthpoint-mcp");
        }
    }

    Ok(())
}

fn build_client(cli: &Cli) -> Result<HealthpointClient> {
    let base_url = Url::parse(&cli.base_url).context("invalid --base-url")?;
    let auth_scheme = parse_auth_scheme(&cli.auth_scheme)?;
    let mut config = ClientConfig::new(base_url, cli.api_key.clone(), auth_scheme);
    config.geo_search_mode = parse_geo_search_mode(&cli.geo_search_mode)?;
    Ok(HealthpointClient::new(config))
}

impl ServiceQueryArgs {
    fn to_query(&self) -> Result<ServiceQuery> {
        let nearby = match (self.lat, self.lon) {
            (Some(lat), Some(lon)) => Some(GeoPoint { lat, lon }),
            (None, None) => None,
            _ => anyhow::bail!("both --lat and --lon are required for nearby search"),
        };
        let mut service_types = self
            .service_types
            .iter()
            .map(|raw| Code::from_token(raw))
            .collect::<Vec<_>>();
        service_types.extend(self.snomed_types.iter().map(|raw| Code::snomed(raw.clone())));
        let query = ServiceQuery {
            text: self.text.clone(),
            categories: self.categories.iter().map(|raw| Code::from_token(raw)).collect(),
            service_types,
            specialties: self.specialties.iter().map(|raw| Code::from_token(raw)).collect(),
            nearby,
            radius_km: self.radius_km,
            limit: QueryLimit(self.limit),
            cursor: self.cursor.clone(),
        };
        query.validate()?;
        Ok(query)
    }
}

fn print_service_page(page: &Page<ServiceRecord>, format: OutputFormat) -> Result<()> {
    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(page)?),
        OutputFormat::Jsonl => healthpoint_export::write_services_jsonl(&page.items, std::io::stdout())?,
        OutputFormat::Csv => healthpoint_export::write_services_csv(&page.items, std::io::stdout())?,
        OutputFormat::Human => print_services_human(&page.items),
    }
    Ok(())
}

fn print_services(records: &[ServiceRecord], format: OutputFormat) -> Result<()> {
    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(records)?),
        OutputFormat::Jsonl => healthpoint_export::write_services_jsonl(records, std::io::stdout())?,
        OutputFormat::Csv => healthpoint_export::write_services_csv(records, std::io::stdout())?,
        OutputFormat::Human => print_services_human(records),
    }
    Ok(())
}

fn print_organization(record: &OrganizationRecord, format: OutputFormat) -> Result<()> {
    match format {
        OutputFormat::Human => {
            println!("{}\t{}", record.id, record.name.as_deref().unwrap_or("<unnamed>"));
        }
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(record)?),
        OutputFormat::Jsonl => println!("{}", serde_json::to_string(record)?),
        OutputFormat::Csv => anyhow::bail!("CSV output is not implemented for single Organization records"),
    }
    Ok(())
}

fn print_location(record: &LocationRecord, format: OutputFormat) -> Result<()> {
    match format {
        OutputFormat::Human => {
            println!("{}\t{}", record.id, record.name.as_deref().unwrap_or("<unnamed>"));
            if let Some(address) = &record.address {
                println!("  address: {}", address.text.as_deref().unwrap_or("<unspecified>"));
            }
        }
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(record)?),
        OutputFormat::Jsonl => println!("{}", serde_json::to_string(record)?),
        OutputFormat::Csv => anyhow::bail!("CSV output is not implemented for single Location records"),
    }
    Ok(())
}

fn print_schema(target: SchemaTarget) -> Result<()> {
    let schema = match target {
        SchemaTarget::AccessPolicy => serde_json::to_value(schema_for!(AccessPolicy))?,
        SchemaTarget::ExportManifest => serde_json::to_value(schema_for!(ExportManifest))?,
        SchemaTarget::ServiceQuery => serde_json::to_value(schema_for!(ServiceQuery))?,
        SchemaTarget::ServiceRecord => serde_json::to_value(schema_for!(ServiceRecord))?,
        SchemaTarget::ServicePage => serde_json::to_value(schema_for!(Page<ServiceRecord>))?,
        SchemaTarget::LocationRecord => serde_json::to_value(schema_for!(LocationRecord))?,
        SchemaTarget::OrganizationRecord => serde_json::to_value(schema_for!(OrganizationRecord))?,
        SchemaTarget::ResourceUri => serde_json::to_value(schema_for!(HealthpointResourceUri))?,
    };
    println!("{}", serde_json::to_string_pretty(&schema)?);
    Ok(())
}

fn write_manifest(manifest: &ExportManifest, output: Option<&Path>) -> Result<()> {
    match output {
        Some(path) => {
            create_parent_dir(path)?;
            let file = File::create(path).with_context(|| format!("failed to create {}", path.display()))?;
            serde_json::to_writer_pretty(file, manifest)?;
        }
        None => println!("{}", serde_json::to_string_pretty(manifest)?),
    }
    Ok(())
}

fn sidecar_manifest_path(output: &Path) -> PathBuf {
    let mut path = output.as_os_str().to_os_string();
    path.push(".manifest.json");
    PathBuf::from(path)
}

fn create_parent_dir(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent().filter(|parent| !parent.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    Ok(())
}

fn print_services_human(records: &[ServiceRecord]) {
    for record in records {
        println!("{}\t{}", record.id, record.name.as_deref().unwrap_or("<unnamed>"));
        if let Some(provider) = &record.provided_by {
            println!("  provider: {}", provider.display.as_deref().unwrap_or(&provider.reference));
        }
        if !record.service_types.is_empty() {
            println!(
                "  types: {}",
                record
                    .service_types
                    .iter()
                    .map(|code| code.display.as_deref().unwrap_or(&code.code))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        if !record.locations.is_empty() {
            println!(
                "  locations: {}",
                record
                    .locations
                    .iter()
                    .map(|location| location.display.as_deref().unwrap_or(&location.reference))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        if let Some(comment) = &record.comment {
            println!("  comment: {comment}");
        }
    }
}
