# Norris

[![Crate Status](https://img.shields.io/crates/v/norris.svg)](https://crates.io/crates/norris)
[![Docs Status](https://docs.rs/norris/badge.svg)](https://docs.rs/crate/norris/)

Simple async wrapper for the [Chuck Norris Joke API](https://api.chucknorris.io/).

## Usage

Example for retrieving a random joke:

```rust
use::std::error::Error;
use::norris::get_random_joke;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let joke = get_random_joke().await?.value;
    println!("{}", joke);
    Ok(())
}
```

## Documentation

Please refer to [docs.rs](https://docs.rs/norris/).
