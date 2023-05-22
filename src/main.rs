use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Error;
use serde::Deserialize;
use std::io::{stdin, stdout, Write};

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Results {
    results: Vec<Ingredients>,
    #[serde(flatten)]
    pagination: Pagination,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Ingredients {
    id: u32,
    name: String,
    image: String,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Pagination {
    offset: u32,
    number: u32,
    #[serde(rename = "totalResults")]
    total_results: u32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut term = String::new();
    print!("{}", "Please enter the ingredients to search for: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut term).expect("failed to read line");

    let query =
        format!("food/ingredients/search?query={term}&metaInformation=false&offset=0&number=10");
    let request_url =
        format!("https://spoonacular-recipe-food-nutrition-v1.p.rapidapi.com/{query}");

    let mut headers = HeaderMap::new();
    headers.insert(
        "X-Rapidapi-Key",
        HeaderValue::from_static("bw9akQ59yyjwvLyUHSkW2QK98k8GoucQ"),
    );
    headers.insert(
        "X-Rapidapi-Host",
        HeaderValue::from_static("spoonacular-recipe-food-nutrition-v1.p.rapidapi.com"),
    );
    headers.insert(
        "Host",
        HeaderValue::from_static("spoonacular-recipe-food-nutrition-v1.p.rapidapi.com"),
    );

    let client = reqwest::Client::new()
        .get(&request_url)
        .headers(headers)
        .send()
        .await?;

    let data: Results = client.json().await?;
    let ingredients = data.results;
    for v in ingredients {
        println!("{:?}", v.name);

    }
    Ok(())
}
