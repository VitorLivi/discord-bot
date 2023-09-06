use scraper::element_ref::Select;

use crate::web_scraper::web_scraper::WebScraper;

pub struct Article {
    pub title: String,
    pub description: String,
    pub date: String,
    pub link: String,
}

impl Article {
    pub fn new(title: String, description: String, date: String, link: String) -> Article {
        Article {
            title,
            description,
            date,
            link,
        }
    }
}

unsafe impl Send for InvestingNews {}

pub struct InvestingNews {
    web_scraper: WebScraper,
}

unsafe impl Sync for InvestingNews {}

impl InvestingNews {
    pub async fn new() -> InvestingNews {
        let new_web_scraper: WebScraper =
            WebScraper::new("https://www.infomoney.com.br/ultimas-noticias/".to_string()).await;

        InvestingNews {
            web_scraper: new_web_scraper,
        }
    }

    pub fn get_news(&self) -> Vec<Article> {
        let scraper_html: &scraper::Html = &self.web_scraper.scraper_html;
        let item_selector = scraper::Selector::parse(".item").unwrap();
        let description_selector = scraper::Selector::parse(".hl-title").unwrap();
        let title_selector = scraper::Selector::parse(".hl-hat").unwrap();
        let date_selector = scraper::Selector::parse(".posted-diff").unwrap();
        let description_a_selector = scraper::Selector::parse("a").unwrap();

        let mut articles = scraper_html.select(&item_selector);

        let mut result: Vec<Article> = Vec::new();
        while let Some(article) = articles.next() {
            let description = article.select(&description_selector).next().unwrap();
            let description_a = description.select(&description_a_selector).nth(0).unwrap();

            let link = description_a.value().attr("href").unwrap();
            let description = description_a.inner_html();

            let title = article.select(&title_selector).next().unwrap().inner_html();
            let date = article.select(&date_selector).next().unwrap().inner_html();

            result.push(Article::new(title, description, date, link.to_string()));
        }

        return result;
    }
}
