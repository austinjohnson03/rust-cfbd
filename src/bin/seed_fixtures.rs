use reqwest::Client;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use tokio::time::{Duration, sleep};

#[derive(Debug, Deserialize)]
struct FixtureRow {
    fixture_name: String, // Example: coaches_no_params, coaches_all_params
    stub_fn_name: String, // Stub function name: mount_coaches_no_params_stub
    url: String,          // API Url to hit to collect JSON response
}

#[derive(Debug, Deserialize)]
struct QueryInfo {
    endpoint: String,
    params: Option<HashMap<String, String>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("CFBD_API_KEY").expect("CFBD_API_KEY not set in .env");

    let csv_path = std::env::args()
        .nth(1)
        .expect("Usage: seed_fixtures <path/to/fixtures.csv>");

    let fixture_dir = PathBuf::from("tests/common/fixtures");
    let stub_dir = PathBuf::from("tests/common/stubs");
    fs::create_dir_all(&fixture_dir)?;
    fs::create_dir_all(&stub_dir)?;

    let mut reader = csv::Reader::from_path(&csv_path)?;
    let client = Client::new();
    let mut stub_paths_to_format: std::collections::HashSet<PathBuf> = HashSet::new();

    for r in reader.deserialize() {
        let row: FixtureRow = r?;
        let query_info = parse_query_info(&row.url)?;

        println!("Fetching {} -> {}.json", row.url, row.fixture_name);

        let resp = client.get(&row.url).bearer_auth(&api_key).send().await?;
        let status = resp.status();
        if !status.is_success() {
            eprint!(
                " WARN: {} returned {} - Skipping row.",
                row.fixture_name,
                status.as_u16()
            );
            sleep(Duration::from_secs(2)).await;
            continue;
        }

        let body = resp.text().await?;
        let parsed: serde_json::Value = serde_json::from_str(&body)?;
        let pretty = serde_json::to_string_pretty(&parsed)?;

        let output_path = fixture_dir.join(format!("{}.json", row.fixture_name));
        fs::write(&output_path, pretty)?;
        println!(" Successfully wrote contents {}", output_path.display());

        let stub_fn = generate_stub_fn(
            &row.stub_fn_name,
            &query_info.endpoint,
            &query_info.params,
            &row.fixture_name,
        );
        let stub_path = stub_dir.join(format!("{}.rs", query_info.endpoint));

        let existing_content = get_or_create_stub_file(&stub_path)?;
        let fn_signature = format!("pub async fn {}", row.stub_fn_name);
        if existing_content.contains(&fn_signature) {
            eprintln!(
                "ERROR: {} already exists. Skipping stub creation",
                row.stub_fn_name
            );
            sleep(Duration::from_secs(2)).await;
            continue;
        }

        let mut stub_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&stub_path)?;
        stub_file.write_all(stub_fn.as_bytes())?;

        // sleep between reqs
        stub_paths_to_format.insert(stub_path);
        sleep(Duration::from_secs(2)).await;
    }

    for path in stub_paths_to_format {
        std::process::Command::new("rustfmt")
            .arg("--edition")
            .arg("2021")
            .arg(&path)
            .status()
            .ok();

        println!("Formatted {}", path.display());
    }

    println!("\nDone");
    Ok(())
}

fn parse_query_info(url: &str) -> anyhow::Result<QueryInfo> {
    let after_host = url
        .split(".com/")
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("Invalid url: {}", url))?;

    let (endpoint, params) = match after_host.split_once('?') {
        Some((ep, query_string)) => {
            let map = query_string
                .split('&')
                .filter_map(|pair| {
                    let (k, v) = pair.split_once('=')?;
                    Some((k.to_string(), v.to_string()))
                })
                .collect::<HashMap<String, String>>();

            let cleaned_ep = ep.trim_start_matches('/');
            let top_level_ep = cleaned_ep.split('/').next().unwrap_or(cleaned_ep);
            (top_level_ep.to_string(), Some(map))
        }
        None => {
            let cleaned_ep = after_host.trim_start_matches('/');
            let top_level_ep = cleaned_ep.split('/').next().unwrap_or(cleaned_ep);
            (top_level_ep.to_string(), None)
        }
    };

    Ok(QueryInfo { endpoint, params })
}

fn generate_stub_fn(
    fn_name: &str,
    endpoint: &str,
    params: &Option<HashMap<String, String>>,
    fixture_name: &str,
) -> String {
    let param_matchers = params.as_ref().map(|map| {
        map.iter()
            .map(|(k, v)| format!("\t.and(query_param(\"{}\", \"{}\"))", k, v))
            .collect::<Vec<_>>()
            .join("\n")
    });

    let matchers = match param_matchers {
        Some(ref s) => format!("\t.and(path(\"/{}\"))\n{}", endpoint, s),
        None => format!("\t.and(path(\"/{}\"))", endpoint),
    };

    format!(
        r#"pub async fn {fn_name}(server: &MockServer) {{
            Mock::given(method("GET"))
                {matchers}
                .respond_with(
                    ResponseTemplate::new(200).set_body_raw(
                        include_str!("../fixtures/{fixture_name}.json"),
                        "application/json"
                    ),
                )
                .mount(server)
                .await;
    }}

        "#,
        fn_name = fn_name,
        matchers = matchers,
        fixture_name = fixture_name,
    )
}

fn get_or_create_stub_file(stub_path: &PathBuf) -> anyhow::Result<String> {
    if stub_path.exists() {
        Ok(fs::read_to_string(stub_path)?)
    } else {
        let header = "// @generated by seed_fixtures\n\
                            use wiremock::{Mock, MockServer, ResponseTemplate};\n\
                            use wiremock::matchers::{method, path, query_param};\n\n";
        fs::write(stub_path, header)?;
        Ok(header.to_string())
    }
}
