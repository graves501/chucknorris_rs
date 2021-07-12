use serde::{Deserialize};

#[derive(Deserialize)]
struct ChuckNorrisJoke {
    value: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get("https://api.chucknorris.io/jokes/random")
        .await?
        .json::<ChuckNorrisJoke>()
        .await?;
    println!("{}", response.value);
    Ok(())
}
