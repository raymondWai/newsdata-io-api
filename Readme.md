# newsdata-io-api

This crate provides a Rust client for the Newsdata.io API.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
newsdata-io-api = "0.9.0"
```
Usage
1. Get your API key
You can get a free API key from Newsdata.io.

2. Create a NewsdataIO instance
```rust
use newsdata_io_api::{NewsdataIO, Auth};

// Replace "YOUR_API_KEY" with your actual API key
let newsdata_io = NewsdataIO::new(Auth::new("YOUR_API_KEY".to_string()));
```
3. Make API requests
   The NewsdataIO instance provides methods for making various API requests which include following:
   1. Latest news
   2. Crypto news
   3. News archive
   4. News sources

# Examples
## Get Latest News
```rust
use newsdata_io_api::{NewsdataIO, Auth, LatestNews, GetLatestNewsParams, Flag};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Replace "YOUR_API_KEY" with your actual API key
    let newsdata_io = NewsdataIO::new(Auth::new("YOUR_API_KEY".to_string()));

    // Get the latest news articles for the US in the business category
    let response = newsdata_io
        .get_latest(
            &GetLatestNewsParams {
                country: Some(vec!["us".to_string()]),
                category: Some(vec!["business".to_string()]),
                size: Some(10), // Get the top 10 articles
                full_content: Some(Flag::True), // Include full content
                ..Default::default()
            },
        ).unwrap();
    println!("{}", response);
    Ok(())
}

```
## Get Crypto News
```rust
use newsdata_io_api::{NewsdataIO, Auth, CryptoNews, GetCryptoNewsParams, Flag};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Replace "YOUR_API_KEY" with your actual API key
    let newsdata_io = NewsdataIO::new(Auth::new("YOUR_API_KEY".to_string()));

    // Get the latest news articles for the US in the business category
    let response = newsdata_io
        .get_crypto_news(
            &GetCryptoNewsParams {
                coin: Some(vec!["BTC".to_string()]),
                size: Some(10), // Get the top 10 articles
                full_content: Some(Flag::True), // Include full content
                ..Default::default()
            },
        ).unwrap();
    println!("{}", response);
    Ok(())
}

```
## Get News Archive
```rust
use newsdata_io_api::{NewsdataIO, Auth, NewsArchive, GetNewsArchiveParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Replace "YOUR_API_KEY" with your actual API key
    let newsdata_io = NewsdataIO::new(Auth::new("YOUR_API_KEY".to_string()));

    // Get the latest news articles for the US in the business category
    let response = newsdata_io
        .get_news_archive(
            &GetNewsArchiveParams {
                q: "business".to_string(),
                ..Default::default()
            },
        ).unwrap();
    println!("{}", response);
    Ok(())
}

```
## Get News Source
```rust
use newsdata_io_api::{NewsdataIO, Auth, NewsSources, GetNewsSourcesParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Replace "YOUR_API_KEY" with your actual API key
    let newsdata_io = NewsdataIO::new(Auth::new("YOUR_API_KEY".to_string()));

    // Get the latest news articles for the US in the business category
    let response = newsdata_io
        .get_news_sources(
            &GetNewsSourcesParams {
                country: Some(vec!["us".to_string()]),
                ..Default::default()
            },
        ).unwrap();
    println!("{}", response);
    Ok(())
}

```