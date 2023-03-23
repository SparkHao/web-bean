
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let body = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;

    println!("body = {:?}", body);


    let client = reqwest::Client::new();
    let res = client.post("http://httpbin.org/post")
        .body("the exact body that is sent")
        .send()
        .await?;
    println!("resp: {:?}", res);

    Ok(())
}