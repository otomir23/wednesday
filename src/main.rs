use clap::Parser;
use std::thread::sleep;
use std::time::Duration;
use reqwest::blocking::Client;
use serde::{Serialize, Deserialize};

/// A simple command line tool to watch for a new minecraft snapshot
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The target version to watch for. If not specified, snapshot code will be automatically determined
    #[arg(short, long)]
    target: Option<String>,

    /// The interval in seconds between checks
    #[arg(short, long, default_value_t = 30)]
    interval: u32,

    /// Continue watching even if a connection error occurs
    #[arg(short, long)]
    suppress: bool,

    /// Log all connection attempts
    #[arg(short, long)]
    verbose: bool,

    /// Custom launcher meta API URL
    #[arg(short, long)]
    url: Option<String>,
}

/// Representation of the latest version object
#[derive(Serialize, Deserialize, Debug)]
struct Latest {
    release: String,
    snapshot: String,
}

/// Representation of a minecraft version data
#[derive(Serialize, Deserialize, Debug)]
struct Version {
    id: String,
    #[serde(rename = "type")]
    version_type: String,
    url: String,
    time: String,
    #[serde(rename = "releaseTime")]
    release_time: String,
}

/// Representation of a response from Mojang's launcher meta API
#[derive(Serialize, Deserialize, Debug)]
struct LauncherMetaResponse {
    latest: Latest,
    versions: Vec<Version>,
}

/// Entry point of the program
fn main() {
    let args = Args::parse();
    let client = Client::new();

    let target = args.target.unwrap_or_else(|| {
        // Generate target version
        // Template: [two digit year]w[week number]a

        let now = chrono::Local::now();
        let year = now.format("%y");
        let week = now.format("%V");
        format!("{}w{}a", year, week)
    });
    let url = args.url.unwrap_or("https://launchermeta.mojang.com/mc/game/version_manifest.json".to_string());

    loop {
        let result = check(&url, &target, &client);
        match result {
            Ok(Some(release_time)) => {
                if args.verbose {
                    println!("{} was released at {}", target, release_time);
                } else {
                    println!("{}", release_time);
                }
                break;
            },
            Ok(None) => {
                if args.verbose {
                    println!("{} was not found. Retrying...", target);
                }
            },
            Err(e) => {
                eprintln!("Error happened while checking for {}: {}", target, e);
                if !args.suppress {
                    break;
                }
            }
        }

        sleep(Duration::from_secs(args.interval as u64));
    }
}

/// Checks if the target version is released
///
/// # Returns
/// the release time if the version is found, None if not found, and an error if an error occurs
///
/// # Arguments
/// * `url` - The URL to the launcher meta API
/// * `target` - The target version to check for
/// * `client` - The reqwest blocking client to use
///
/// # Example
/// ```
/// let client = reqwest::blocking::Client::new();
/// let result = check("https://launchermeta.mojang.com/mc/game/version_manifest.json", "21w06a", &client);
/// ```
fn check(url: &str, target: &str, client: &Client) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let resp = client.get(url).send()?.json::<LauncherMetaResponse>()?;

    resp.versions.iter()
        .find(|v| v.id == target)
        .map(|v| Ok(Some(v.release_time.clone())))
        .unwrap_or(Ok(None))
}