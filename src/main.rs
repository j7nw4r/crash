#[tokio::main]
async fn main() {
    let resp = reqwest::get("https://www.example.org").await.unwrap();

    println!(
        "Got HTTP {}, with headers {:#?}",
        resp.status(),
        resp.headers()
    );

    let body = resp.text().await.unwrap();
    let num_lines = 10;
    println!("Print the first {num_lines} of the body");
    for line in body.lines().take(num_lines) {
        println!("{line}");
    }
}
