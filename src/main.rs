// use std::collections::HashMap;
use scraper::{Html, Selector};
extern crate select;

// fn main() {
//     hacker_news("https://news.ycombinator.com");
// }

#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;

    // println!("body = {:?}", body);

    let html = body;

    // println!("body = {:?}", html);

    let fragment = Html::parse_fragment(&html);
    let selector = Selector::parse("li").unwrap();

    for element in fragment.select(&selector) {
        assert_eq!("li", element.value().name());
        println!("test {:?}", element.html());
    }

    Ok(())
}
