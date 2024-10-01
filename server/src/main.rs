use reqwest::get;

#[tokio::main]
async fn main() {
    let resp = get("https://www.youtube.com/watch?v=HTAQxUXq674").await;
    println!("{}", resp.unwrap().text().await.unwrap());
}
