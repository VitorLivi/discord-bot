use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct ResponseNewsContent {
    pub title: String,
    pub body: String,
    pub owner_username: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ResponseNews {
    pub slug: String,
    pub title: String,
    pub owner_username: String,
}

pub struct News {
    pub title: String,
    pub body: String,
    pub user: String,
    pub url: String,
}

pub struct TabNews {
    reqwest: reqwest::Client,
}

impl TabNews {
    pub fn new() -> Self {
        Self {
            reqwest: reqwest::Client::new(),
        }
    }

    pub async fn get_relevants(&self) -> Result<Vec<News>, reqwest::Error> {
        let response = self
            .reqwest
            .get("https://www.tabnews.com.br/api/v1/contents?page=1&per_page=8&strategy=relevant")
            .send()
            .await?
            .json::<Vec<ResponseNews>>()
            .await?;

        let mut news: Vec<News> = Vec::new();
        for article in response {
            let article_content: ResponseNewsContent =
                self.get_article_content(article.clone()).await?;

            news.push(News {
                title: article_content.title,
                body: article_content.body,
                user: article_content.owner_username,
                url: format!(
                    "https://www.tabnews.com.br/{}/{}",
                    article.owner_username, article.slug
                ),
            });
        }

        Ok(news)
    }

    async fn get_article_content(
        &self,
        data: ResponseNews,
    ) -> Result<ResponseNewsContent, reqwest::Error> {
        let url = format!(
            "https://www.tabnews.com.br/api/v1/contents/{}/{}",
            data.owner_username, data.slug
        );

        let response = self
            .reqwest
            .get(&url)
            .send()
            .await?
            .json::<ResponseNewsContent>()
            .await?;

        Ok(response)
    }
}
