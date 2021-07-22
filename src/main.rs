use reqwest;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    println!("Hello, world!");
    let body = reqwest::get("https://www.baidu.com/").await?.text().await?;
    println!("{}", body);
    Ok(())
}
