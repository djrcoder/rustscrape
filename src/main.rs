use scraper::{Html, Selector};
use reqwest;
#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::get("https://news.ycombinator.com/")
        .await?
        .text()
        .await?;

    let html = body;
    let fragment = Html::parse_fragment(&html);
    let stories = Selector::parse(".storylink").unwrap();

   // loop the elements matching the css selector
   for story in fragment.select(&stories) {
        // grab the headline text and place into a vector
        let story_txt = story.text().collect::<Vec<_>>();
        println!("{:?}", story_txt);
    }

    Ok(())
}
