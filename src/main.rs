// following https://www.scrapingbee.com/blog/web-scraping-rust/ for basic web scraping
fn main() {
    // http GET request to the url
    let response = reqwest::blocking::get(
        "https://www.imdb.com/search/title/?groups=top_100&sort=user_rating,desc&count=100",
    )
    .unwrap()
    .text()
    .unwrap();

    // load the document into the parser
    let document = scraper::Html::parse_document(&response);
    // create the selector for f3 titles
    let title_selector = scraper::Selector::parse("h3.lister-item-header>a").unwrap();
    // find all h3 titles
    let titles = document.select(&title_selector).map(|x| x.inner_html());

    for title in titles {
        println!("{title}");
    }
}
