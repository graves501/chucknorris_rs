#![doc = include_str!("../README.md")]

use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use strum_macros::Display;

/// API response when searching by search terms - contains multiple jokes and the total amount of
/// received jokes
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JokeSearchResult {
    pub total: i64,
    pub result: Vec<ChuckNorrisJoke>,
}

/// API response when requesting a joke - reduced to the joke string only
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChuckNorrisJoke {
    pub value: String,
}

/// Available joke categories to search by
#[derive(Display)]
pub enum JokeCategory {
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

pub async fn get_random_joke() -> Result<ChuckNorrisJoke> {
    let response = reqwest::get("https://api.chucknorris.io/jokes/random")
        .await?
        .json::<ChuckNorrisJoke>()
        .await?;

    Ok(response)
}

pub async fn get_random_joke_by_category(category: JokeCategory) -> Result<ChuckNorrisJoke> {
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

/// Searches for a joke that contain the `search_string`
pub async fn search_for_one_joke(search_string: String) -> Result<ChuckNorrisJoke> {
    let response = reqwest::get(format!(
        "https://api.chucknorris.io/jokes/search?query={}",
        search_string
    ))
    .await?
    .json::<JokeSearchResult>()
    .await?;

    Ok(response.result.get(0).unwrap().to_owned())
}

/// Searches for all jokes that contain the `search_string`
pub async fn search_for_all_available_jokes(search_string: String) -> Result<JokeSearchResult> {
    let response = reqwest::get(format!(
        "https://api.chucknorris.io/jokes/search?query={}",
        search_string
    ))
    .await?
    .json::<JokeSearchResult>()
    .await?;

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_random_joke() -> Result<()> {
        let response = get_random_joke().await?;
        let joke = response.value;

        assert!(!joke.is_empty());
        assert!(joke.contains("Chuck"));
        assert!(joke.contains("Norris"));

        Ok(())
    }

    #[tokio::test]
    async fn test_get_random_joke_by_category() -> Result<()> {
        let response = get_random_joke_by_category(JokeCategory::Food).await?;
        let joke = response.value;

        assert!(!joke.is_empty());
        assert!(joke.contains("Chuck"));
        assert!(joke.contains("Norris"));

        Ok(())
    }

    #[tokio::test]
    async fn test_search_for_one_joke() -> Result<()> {
        let response = search_for_one_joke(String::from("moon")).await?;
        let joke = response.value;

        assert!(!joke.is_empty());
        assert!(joke.contains("Chuck"));
        assert!(joke.contains("Norris"));

        Ok(())
    }

    #[tokio::test]
    async fn test_search_for_all_available_jokes() -> Result<()> {
        let response = search_for_all_available_jokes(String::from("texas")).await?;

        assert!(response.total > 0);
        assert!(response.result.len() > 0);

        Ok(())
    }
}
