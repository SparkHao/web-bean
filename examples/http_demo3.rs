use std::collections::HashMap;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut map = HashMap::new();
    map.insert("lang", "rust");
    map.insert("body", "json");

    let client = reqwest::Client::new();
    let res = client.post("http://httpbin.org/post")
        .json(&map)
        .send()
        .await?;
    println!("{:#?}", res);
    Ok(())
}