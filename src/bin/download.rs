use std::{
    env,
    fs,
};

use anyhow::{
    Context,
    Result,
};
use dotenv::dotenv;
use reqwest::Client;

fn make_url(year: u32, day: u32) -> String {
    format!("https://adventofcode.com/{}/day/{}/input", year, day)
}

fn make_filename(year: u32, day: u32) -> String {
    format!("input/year{}/day{:02}.txt", year, day)
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let session_cookie = env::var("SESSION_COOKIE").context("SESSION_COOKIE is not set")?;

    let client = Client::new();

    for year in 2015..=2024 {
        let year_dir = format!("input/year{}", year);
        fs::create_dir_all(&year_dir)
            .with_context(|| format!("Failed to create directory: {}", year_dir))?;

        for day in 1..=25 {
            let url = make_url(year, day);
            let filename = make_filename(year, day);

            let response = client
                .get(&url)
                .header("Cookie", format!("session={}", session_cookie))
                .send()
                .await
                .with_context(|| format!("Failed to request: {}", url))?;

            if !response.status().is_success() {
                return Err(anyhow::anyhow!(
                    "Request failed with status: {} for {}",
                    response.status(),
                    url
                ));
            }

            let content = response
                .text()
                .await
                .with_context(|| format!("Failed to read response body for: {}", url))?;

            fs::write(&filename, &content)
                .with_context(|| format!("Failed to write file: {}", filename))?;

            println!("Downloaded {}", filename);
        }
    }

    Ok(())
}
