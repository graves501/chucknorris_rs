use anyhow::Result;
use serde::Deserialize;
use strum_macros::Display;

#[derive(Deserialize)]
struct ChuckNorrisJoke {
    value: String,
}

#[derive(Display)]
enum JokeCategory {
    Animal,
    Career,
    Celebrity,
    Dev,
    Explicit,
    Fashion,
    Food,
    History,
    Money,
    Movie,
    Music,
    Political,
    Religion,
    Science,
    Sport,
    Travel,
}

//TODO free textsearch https://api.chucknorris.io/jokes/search?query={query}
//TODO list all categories https://api.chucknorris.io/jokes/categories
//TODO get joke from category https://api.chucknorris.io/jokes/random?category={category}

#[tokio::main]
async fn main() -> Result<()> {
    let joke = get_random_joke().await?.value;

    println!("{}", joke);
    println!(
        "{}",
        get_random_joke_by_category(JokeCategory::Animal)
            .await?
            .value
    );
    // println!("{}", search_for_joke(String::from("texas")).await?.value);

    Ok(())
}

async fn get_random_joke() -> Result<ChuckNorrisJoke> {
    let response = reqwest::get("https://api.chucknorris.io/jokes/random")
        .await?
        .json::<ChuckNorrisJoke>()
        .await?;

    Ok(response)
}

async fn search_for_joke(search_string: String) -> Result<ChuckNorrisJoke> {
    let response = reqwest::get(format!(
        "https://api.chucknorris.io/jokes/search?query={}",
        search_string
    ))
    .await?
    .json::<ChuckNorrisJoke>()
    .await?;

    Ok(response)
}

async fn get_random_joke_by_category(category: JokeCategory) -> Result<ChuckNorrisJoke> {
    let response = reqwest::get(format!(
        "https://api.chucknorris.io/jokes/random?category={}",
        joke_category_to_string(category)
    ))
    .await?
    .json::<ChuckNorrisJoke>()
    .await?;

    Ok(response)
}

fn joke_category_to_string(joke_category: JokeCategory) -> String {
    joke_category.to_string().to_lowercase()
}
