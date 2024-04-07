#[tokio::main]
async fn main() {
    let r1 = reqwest::get("https://httpbin.org/ip").await;
    let r2 = reqwest::get("https://google.com").await;
    println!("{}", r1.unwrap().status());
    println!("{}", r2.unwrap().status());
}
