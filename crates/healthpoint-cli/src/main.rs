//! Healthpoint CLI entrypoint.

use std::{fs::File, path::PathBuf};

use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand, ValueEnum};
use healthpoint_client::{parse_auth_scheme, ClientConfig, HealthpointClient};
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

    /// FHIR type token, e.g. a SNOMED CT code. Repeatable.
    #[arg(long = "type")]
    service_types: Vec<String>,

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
                OutputFormat::Human => print_services_human(&page.items),
            }
        }
        Command::Get {
            command: GetCommand::Service(args),
        } => {
            let service = client.get_service(&args.id).await?;
            match args.format {
                OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&service)?),
                OutputFormat::Human => print_services_human(&[service]),
            }
        }
        Command::Get {
            command: GetCommand::Organization(args),
        } => {
            let org = client.get_organization(&args.id).await?;
            println!("{}", serde_json::to_string_pretty(&org)?);
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
    Ok(HealthpointClient::new(ClientConfig::new(
        base_url,
        cli.api_key.clone(),
        auth_scheme,
    )))
}

impl ServiceSearchArgs {
    fn to_query(&self) -> Result<ServiceQuery> {
        let nearby = match (self.lat, self.lon) {
            (Some(lat), Some(lon)) => Some(GeoPoint { lat, lon }),
            (None, None) => None,
            _ => anyhow::bail!("both --lat and --lon are required for nearby search"),
        };
        Ok(ServiceQuery {
            text: self.text.clone(),
            categories: self.categories.iter().map(parse_code).collect(),
            service_types: self.service_types.iter().map(parse_code).collect(),
            specialties: self.specialties.iter().map(parse_code).collect(),
            nearby,
            radius_km: self.radius_km,
            limit: QueryLimit(self.limit),
            cursor: None,
        })
    }
}

fn parse_code(raw: &String) -> Code {
    if let Some((system, code)) = raw.split_once('|') {
        Code {
            system: Some(system.to_owned()),
            code: code.to_owned(),
            display: None,
        }
    } else {
        Code::bare(raw)
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
    }
}
