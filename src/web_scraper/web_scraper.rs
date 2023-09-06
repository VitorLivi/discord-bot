use reqwest::Client;

#[derive(Clone)]
pub struct WebScraper {
    reqwest: reqwest::Client,
    pub scraper_html: scraper::Html,
    url: String,
}

unsafe impl Send for WebScraper {}

impl WebScraper {
    pub async fn new(url: String) -> WebScraper {
        let mut new_web_scrapper: WebScraper = WebScraper {
            reqwest: Client::new(),
            scraper_html: scraper::Html::new_document(),
            url: url.clone(),
        };

        new_web_scrapper.load().await;
        new_web_scrapper
    }

    pub async fn load(&mut self) {
        let response = self.reqwest.get(&self.url.to_string()).send().await;

        match response {
            Ok(response) => {
                self.scraper_html = scraper::Html::parse_fragment(&response.text().await.unwrap());
            }
            Err(error) => {
                println!("Error: {}", error);
            }
        }
    }

    pub fn update_url(&mut self, url: String) {
        self.url = url;
    }
}
