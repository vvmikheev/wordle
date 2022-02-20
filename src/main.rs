use reqwest::Client;
use std::error::Error;
use std::fs;
use url::Url;
use once_cell::sync::OnceCell;

static URL: OnceCell<Url> = OnceCell::new();

fn initialize_url() -> Result<Url, Box<dyn Error>> {
    let mut url: Url = "https://dictionary.yandex.net/api/v1/dicservice.json/lookup".parse()?;
    let key = fs::read_to_string("apikey.txt")?;
    url.query_pairs_mut().append_pair("key", &key);
    url.query_pairs_mut().append_pair("lang", "en-ru");
    Ok(url)
}


#[tokio::main]
async fn main() {
    let url = initialize_url().expect("Something bad has happened");
    
    println!("{}", url.as_str());
    URL.set(url).unwrap();
    let client = Client::default();
    is_a_word("texfsdft", &client ).await.unwrap();
}

async fn is_a_word(word: &str, client: &Client) -> Result<bool, Box<dyn Error>> {
    let mut url: Url = URL.get().ok_or("URL wasn't initialized")?.clone();
    url.query_pairs_mut().append_pair("text", word);

    let response = client.get(url.as_str()).send().await?;
    println!("{}", response.text().await?);
    
    Ok(true)
}