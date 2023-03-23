use std::collections::HashMap;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let params = [("foo", "bar"), ("baz", "quux")];
    let client = reqwest::Client::new();
    let res = client.post("http://httpbin.org/post")
        .form(&params)
        .send()
        .await?;
    println!("{:#?}", res);
    Ok(())
}