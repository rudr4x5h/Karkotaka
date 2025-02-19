use crate::core::utils::error::AppError;
use anyhow::Error;
use dotenvy::dotenv;
use reqwest::{
    header::{AUTHORIZATION, USER_AGENT},
    Client,
};
use serde::{Deserialize, Serialize};

pub async fn trending_topics(woeid: String) -> Result<TrendingTopics, AppError> {
    dotenv().expect("Cannot read env vars.");
    // let woeid = "20070458"; // Delhi (India)
    // let endpoint = format!("https://api.x.com/2/trends/by/woeid/{}", woeid);
    let endpoint = format!("https://api.x.com/1.1/trends/place.json?id={}", woeid);
    let ua = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:101.0) Gecko/20100101 Firefox/101.0";
    let auth_token = format!(
        "Bearer {}",
        dotenvy::var("X_TOKEN").expect("no env var found")
    );

    let response = Client::new()
        .get(endpoint)
        .header(USER_AGENT, ua)
        .header(AUTHORIZATION, auth_token)
        .send()
        .await
        .map_err(|e| AppError(Error::msg(e)))?;

    if response.status().is_success() {
        response.json().await.map_err(|e| AppError(Error::msg(e)))
    } else {
        // dbg!(&response);
        // Err(AppError(Error::msg(format!(
        //     "{} / Failed to fetch trending topics",
        //     response.status()
        // ))))
        let sample_response = r#"
            {
                "data": [
                    {
                        "trend_name": "Europe",
                        "tweet_count": 232408
                    },
                    {
                        "trend_name": "Isak",
                        "tweet_count": 2956
                    },
                    {
                        "trend_name": "RNLI",
                        "tweet_count": 2484
                    },
                    {
                        "trend_name": "Toon",
                        "tweet_count": 11447
                    },
                    {
                        "trend_name": "St James",
                        "tweet_count": 5565
                    },
                    {
                        "trend_name": "Manning",
                        "tweet_count": 10077
                    },
                    {
                        "trend_name": "Copenhagen",
                        "tweet_count": 35272
                    }
                ]
            }
            "#;
        let response: TrendingTopics = serde_json::from_str(sample_response)?;
        Ok(response)
    }
}

async fn resolve_topics_to_stories(trending_topics: TrendingTopics) {
    let topics = trending_topics.get_data().to_owned();
    let trends = topics
        .iter()
        .map(|topic| {
            let topic = topic.get_topic().to_owned();
            topic
        })
        .collect::<Vec<_>>();
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendingTopics {
    data: Vec<TrendingTopic>,
}

impl TrendingTopics {
    pub fn get_data(&self) -> &Vec<TrendingTopic> {
        &self.data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendingTopic {
    trend_name: String,
    tweet_count: u32,
}

impl TrendingTopic {
    pub fn get_topic(&self) -> &String {
        &self.trend_name
    }

    pub fn get_tweet_count(&self) -> u32 {
        self.tweet_count
    }
}
