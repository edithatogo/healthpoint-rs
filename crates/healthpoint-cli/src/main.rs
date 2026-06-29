//! Healthpoint CLI entrypoint.

use std::{fs::File, path::PathBuf};

use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand, ValueEnum};
use healthpoint_client::{
    parse_auth_scheme, parse_geo_search_mode, ClientConfig, HealthpointClient,
};
use healthpoint_core::{Code, DirectoryProvider, GeoPoint, QueryLimit, ServiceQuery};
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
    /// Export local data and manifests.
    Export {
        #[command(subcommand)]
        command: ExportCommand,
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

    /// Output format.
    #[arg(long, value_enum, default_value_t = OutputFormat::Human)]
    format: OutputFormat,
}

#[derive(Debug, Subcommand)]
enum GetCommand {
    /// Get a HealthcareService by id.
    Service(GetArgs),
    /// Get an Organization by id.
    Organization(GetArgs),
    /// Get a Location by id.
    Location(GetArgs),
}

#[derive(Debug, Args)]
struct GetArgs {
    /// Resource id.
    id: String,

    /// Output format.
    #[arg(long, value_enum, default_value_t = OutputFormat::Json)]
    format: OutputFormat,
}

#[derive(Debug, Subcommand)]
enum ExportCommand {
    /// Write an export manifest only; does not query Healthpoint.
    Manifest {
        /// Output path.
        #[arg(long)]
        output: Option<PathBuf>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum OutputFormat {
    Human,
    Json,
    Jsonl,
    Csv,
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
            let query = args.to_query()?;
            let page = client.search_services(query).await?;
            match args.format {
                OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&page)?),
                OutputFormat::Jsonl => healthpoint_export::write_services_jsonl(&page.items, std::io::stdout())?,
                OutputFormat::Csv => healthpoint_export::write_services_csv(&page.items, std::io::stdout())?,
                OutputFormat::Human => print_services_human(&page.items),
            }
        }
        Command::Get {
            command: GetCommand::Service(args),
        } => {
            let service = client.get_service(&args.id).await?;
            match args.format {
                OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&service)?),
                OutputFormat::Jsonl => healthpoint_export::write_services_jsonl(&[service], std::io::stdout())?,
                OutputFormat::Csv => healthpoint_export::write_services_csv(&[service], std::io::stdout())?,
                OutputFormat::Human => print_services_human(&[service]),
            }
        }
        Command::Get {
            command: GetCommand::Organization(args),
        } => {
            let org = client.get_organization(&args.id).await?;
            println!("{}", serde_json::to_string_pretty(&org)?);
        }
        Command::Get {
            command: GetCommand::Location(args),
        } => {
            let location = client.get_location(&args.id).await?;
            println!("{}", serde_json::to_string_pretty(&location)?);
        }
        Command::Export {
            command: ExportCommand::Manifest { output },
        } => {
            let manifest = healthpoint_export::ExportManifest::new(
                healthpoint_core::SourceProvenance::healthpoint(cli.base_url),
                false,
            );
            match output {
                Some(path) => {
                    let file = File::create(&path)
                        .with_context(|| format!("failed to create {}", path.display()))?;
                    serde_json::to_writer_pretty(file, &manifest)?;
                }
                None => println!("{}", serde_json::to_string_pretty(&manifest)?),
            }
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

impl ServiceSearchArgs {
    fn to_query(&self) -> Result<ServiceQuery> {
        let nearby = match (self.lat, self.lon) {
            (Some(lat), Some(lon)) => Some(GeoPoint { lat, lon }),
            (None, None) => None,
            _ => anyhow::bail!("both --lat and --lon are required for nearby search"),
        };
        let mut service_types = self.service_types.iter().map(|raw| Code::from_token(raw)).collect::<Vec<_>>();
        service_types.extend(self.snomed_types.iter().map(|raw| Code::snomed(raw.clone())));
        Ok(ServiceQuery {
            text: self.text.clone(),
            categories: self.categories.iter().map(|raw| Code::from_token(raw)).collect(),
            service_types,
            specialties: self.specialties.iter().map(|raw| Code::from_token(raw)).collect(),
            nearby,
            radius_km: self.radius_km,
            limit: QueryLimit(self.limit),
            cursor: self.cursor.clone(),
        })
    }
}

fn print_services_human(records: &[healthpoint_core::ServiceRecord]) {
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
    }
}
