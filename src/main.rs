use serde::Deserialize;
use anyhow::Result;

#[derive(Deserialize)]
struct ChuckNorrisJoke {
    value: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let response = reqwest::get("https://api.chucknorris.io/jokes/random")
        .await?
        .json::<ChuckNorrisJoke>()
        .await?;

    println!("{}", response.value);
    Ok(())
}
