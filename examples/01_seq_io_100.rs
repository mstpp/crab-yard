use anyhow::Result;
use reqwest::blocking::get;
use std::time::Instant;

fn one_100() -> Result<()> {
    for i in 1..101 {
        let url = format!("https://jsonplaceholder.typicode.com/posts?id={}", &i);
        let res = get(&url)?;
        println!("{:3}: {:?}", &i, res);
    }
    Ok(())
}

fn main() {
    let start = Instant::now();
    let _ = one_100();
    let elapsed = start.elapsed();
    println!("\nTotal duration: {:?}", elapsed);
}

// result
// Total duration: 3.205771083s
//
// looks like some caching is used, first time it took even 28s (maybe compilation overhead few seconds)
// Total duration: 28.217084916s
