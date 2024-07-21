use std::collections::HashMap;

use crate::{
    newsdata_io::{NewsdataIO, Requests},
    ApiResult, Json,
};

/// Trait for the Latest News API.
pub trait LatestNews {
    /// Get the latest news articles.
    ///
    /// # Arguments
    ///
    /// * `params`: The parameters for the request.
    ///
    /// # Returns
    ///
    /// An `ApiResult` containing the JSON response from the API.
    fn get_latest(&self, params: &GetLatestNewsParams) -> ApiResult<Json>;
}

impl LatestNews for NewsdataIO {
    fn get_latest(&self, params: &GetLatestNewsParams) -> ApiResult<Json> {
        let mut query_params = HashMap::new();

        if let Some(id) = &params.id {
            query_params.insert("id".to_string(), id.join(","));
        }
        if let Some(q) = &params.q {
            query_params.insert("q".to_string(), q.clone());
        }
        if let Some(q_in_title) = &params.q_in_title {
            query_params.insert("qInTitle".to_string(), q_in_title.clone());
        }
        if let Some(q_in_meta) = &params.q_in_meta {
            query_params.insert("qInMeta".to_string(), q_in_meta.clone());
        }
        if let Some(timeframe) = &params.timeframe {
            query_params.insert("timeframe".to_string(), timeframe.clone());
        }
        if let Some(country) = &params.country {
            query_params.insert("country".to_string(), country.join(","));
        }
        if let Some(category) = &params.category {
            query_params.insert("category".to_string(), category.join(","));
        }
        if let Some(exclude_category) = &params.exclude_category {
            query_params.insert("excludecategory".to_string(), exclude_category.join(","));
        }
        if let Some(language) = &params.language {
            query_params.insert("language".to_string(), language.clone());
        }
        if let Some(tag) = &params.tag {
            query_params.insert("tag".to_string(), tag.clone());
        }
        if let Some(sentiment) = &params.sentiment {
            query_params.insert("sentiment".to_string(), sentiment.clone());
        }
        if let Some(region) = &params.region {
            query_params.insert("region".to_string(), region.clone());
        }
        if let Some(domain) = &params.domain {
            query_params.insert("domain".to_string(), domain.clone());
        }
        if let Some(exclude_domain) = &params.exclude_domain {
            query_params.insert("excludedomain".to_string(), exclude_domain.clone());
        }
        if let Some(domain_url) = &params.domain_url {
            query_params.insert("domainurl".to_string(), domain_url.clone());
        }
        if let Some(exclude_field) = &params.exclude_field {
            query_params.insert("excludefield".to_string(), exclude_field.clone());
        }
        if let Some(priority_domain) = &params.priority_domain {
            query_params.insert("prioritydomain".to_string(), priority_domain.clone());
        }
        if let Some(timezone) = &params.timezone {
            query_params.insert("timezone".to_string(), timezone.clone());
        }
        if let Some(full_content) = &params.full_content {
            println!("full_content: {}", full_content.value().to_string());
            query_params.insert("full_content".to_string(), full_content.value().to_string());
        }
        if let Some(image) = &params.image {
            query_params.insert("image".to_string(), image.value().to_string());
        }
        if let Some(video) = &params.video {
            query_params.insert("video".to_string(), video.value().to_string());
        }
        if let Some(size) = &params.size {
            query_params.insert("size".to_string(), size.to_string());
        }
        if let Some(page) = &params.page {
            query_params.insert("page".to_string(), page.to_string());
        }

        self.get("latest", Some(query_params))
    }
}

/// Enum for representing boolean values as strings.
#[derive(Debug)]
pub enum Flag {
    /// False value.
    False,
    /// True value.
    True,
}

impl Flag {
    /// Returns the string representation of the flag.
    fn value(&self) -> &str {
        match self {
            Flag::True => "1",
            Flag::False => "0",
        }
    }
}

/// Parameters for the `get_latest` method.
#[derive(Debug, Default)]
pub struct GetLatestNewsParams {
    /// Unique identifier of the news article.\
    /// Max no. of id could be added: 50
    pub id: Option<Vec<String>>,
    /// Keywords to search for in the news articles.\
    /// Max characters: 512.\
    /// Exclusive with q_in_title and q_in_meta
    pub q: Option<String>,
    /// Keywords to search for in the title of the news articles.\
    /// Max characters: 512.\
    /// Exclusive with q and q_in_meta
    pub q_in_title: Option<String>,
    /// Keywords to search for in the meta description of the news articles.\
    /// Max characters: 512.\
    /// Exclusive with q and q_in_title
    pub q_in_meta: Option<String>,
    /// Timeframe for the news articles.\
    /// Only hours or minutes is permitted.\
    /// Examples: 6 for 6 hours, 15m for 15 min
    pub timeframe: Option<String>,
    /// Country code for the news articles.\
    /// Max no. of country could be added: 5.\
    /// Examples: "hk", "us", "wo"
    pub country: Option<Vec<String>>,
    /// Category for the news articles.\
    /// Max no. of category could be added: 5.\
    /// Exclusive with exclude_category.\
    /// Possible values: "business", "crime", "domestic", "education", "entertainment", "environment", "food", "health", "lifestyle", "other", "politics", "science", "sports", "technology", "top", "tourism", "world"
    pub category: Option<Vec<String>>,
    /// Category to exclude from the results.\
    /// Max no. of exclude_category could be added: 5.\
    /// Exclusive with category.\
    /// Possible values: "business", "crime", "domestic", "education", "entertainment", "environment", "food", "health", "lifestyle", "other", "politics", "science", "sports", "technology", "top", "tourism", "world"
    pub exclude_category: Option<Vec<String>>,
    /// Language code for the news articles.\
    /// Max no. of language could be added: 5.
    pub language: Option<String>,
    /// Tag for the news articles.\
    /// Max no. of tag could be added: 5.\
    /// **Available only for Professional and Corporate users**\
    /// Possible values in [here](https://newsdata.io/documentation/#latest-news)
    pub tag: Option<String>,
    /// Sentiment for the news articles.\
    /// Possible values: "positive", "negative", "neutral".\
    /// **Available only for Professional and Corporate users**
    pub sentiment: Option<String>,
    /// Region for the news articles.\
    /// Max no. of region could be added: 5.\
    /// **Available only for Corporate users**
    pub region: Option<String>,
    /// Domain for the news articles.\
    /// Max no. of domain could be added: 5.\
    /// Possible values in [here](https://newsdata.io/documentation/#latest-news)
    pub domain: Option<String>,
    /// Domain to exclude from the results.\
    /// Max no. of domain could be added: 5.\
    /// Possible values in [here](https://newsdata.io/documentation/#latest-news)
    pub exclude_domain: Option<String>,
    /// Domain URL for the news articles.\
    /// Max no. of domain could be added: 5.
    pub domain_url: Option<String>,
    /// Field to exclude from the results.\
    /// "article_id" is not excludable in response
    pub exclude_field: Option<String>,
    /// Priority domain for the news articles.\
    /// Top: Fetches news articles from the top 10% of the news domains\
    /// Medium: Fetches news articles from the top 30% of the news domains. It means it already includes all the news articles of "top" priority.\
    /// Low: Fetches news articles from the top 50% of the news domains. It means it already includes all the news articles of "top" and "medium" priorities.
    pub priority_domain: Option<String>,
    /// Timezone for the news articles.
    pub timezone: Option<String>,
    /// Whether to include full content in the results.
    pub full_content: Option<Flag>,
    /// Whether to include images in the results.
    pub image: Option<Flag>,
    /// Whether to include videos in the results.
    pub video: Option<Flag>,
    /// Number of results to return.\
    /// Could only be 1 to 50
    pub size: Option<i32>,
    /// page parameter from last result\
    /// [Detail](https://newsdata.io/documentation/#pagination)
    pub page: Option<String>,
}

impl GetLatestNewsParams {
    /// Creates a new `GetLatestNewsParams` with default values.
    ///
    /// This method sets the default values for all parameters, which are:
    ///
    /// * `id`: `None`
    /// * `q`: `None`
    /// * `q_in_title`: `None`
    /// * `q_in_meta`: `None`
    /// * `timeframe`: `None`
    /// * `country`: `None`
    /// * `category`: `None`
    /// * `exclude_category`: `None`
    /// * `language`: `None`
    /// * `tag`: `None`
    /// * `sentiment`: `None`
    /// * `region`: `None`
    /// * `domain`: `None`
    /// * `exclude_domain`: `None`
    /// * `domain_url`: `None`
    /// * `exclude_field`: `None`
    /// * `priority_domain`: `None`
    /// * `timezone`: `None`
    /// * `full_content`: `Some(Flag::False)`
    /// * `image`: `Some(Flag::False)`
    /// * `video`: `Some(Flag::False)`
    /// * `size`: `None`
    /// * `page`: `None`
    ///
    /// This allows you to easily create a `GetLatestNewsParams` object without having to specify all the parameters manually.
    pub fn default() -> Self {
        GetLatestNewsParams {
            id: None,
            q: None,
            q_in_title: None,
            q_in_meta: None,
            timeframe: None,
            country: None,
            category: None,
            exclude_category: None,
            language: None,
            tag: None,
            sentiment: None,
            region: None,
            domain: None,
            exclude_domain: None,
            domain_url: None,
            exclude_field: None,
            priority_domain: None,
            timezone: None,
            full_content: None,
            image: None,
            video: None,
            size: None,
            page: None,
        }
    }
}
