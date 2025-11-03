use reqwest::Client;
use std::time::Instant;

async fn fetch_url_data(client: &Client, ids: Vec<u32>) -> Vec<reqwest::Result<reqwest::Response>> {
    let tasks = ids.into_iter().map(|id| {
        let client = client.clone();
        async move {
            client
                .get(format!(
                    "https://jsonplaceholder.typicode.com/posts?id={}",
                    &id
                ))
                .send()
                .await
        }
    });
    futures::future::join_all(tasks).await
}

#[tokio::main]
async fn main() {
    let client = Client::new();
    let ids: Vec<u32> = (1..=100).collect();

    let start = Instant::now();
    let responses = fetch_url_data(&client, ids).await;

    for (i, response) in responses.iter().enumerate() {
        println!("Request number ==== {:3} ====", i + 1);
        match response {
            Ok(res) => {
                if res.status().is_success() {
                    println!("Response: {:?}", &res);
                } else {
                    println!("Response {} failed with status: {}", i + 1, res.status());
                }
            }
            Err(e) => println!("Response {} error: {}", i + 1, e),
        }
    }

    let elapsed = start.elapsed();
    println!("Elapsed time: {:?}", elapsed);
}

// result
// Elapsed time: 11.875µs
// Elapsed time: 341.256084ms
