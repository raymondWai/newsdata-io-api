use std::collections::HashMap;

use crate::{
    newsdata_io::{NewsdataIO, Requests},
    ApiResult, Json,
};

/// Trait for the News Sources API.
pub trait NewsSources {
    /// Get news sources.
    ///
    /// # Arguments
    ///
    /// * `params`: The parameters for the request.
    ///
    /// # Returns
    ///
    /// An `ApiResult` containing the JSON response from the API.
    fn get_news_sources(&self, params: &GetNewsSourcesParams) -> ApiResult<Json>;
}

impl NewsSources for NewsdataIO {
    fn get_news_sources(&self, params: &GetNewsSourcesParams) -> ApiResult<Json> {
        let mut query_params = HashMap::new();

        // Add id parameter to query params
        if let Some(id) = &params.id {
            query_params.insert("id".to_string(), id.join(","));
        }

        // Add country parameter to query params
        if let Some(country) = &params.country {
            query_params.insert("country".to_string(), country.join(","));
        }

        // Add category parameter to query params
        if let Some(category) = &params.category {
            query_params.insert("category".to_string(), category.join(","));
        }

        // Add exclude_category parameter to query params
        if let Some(exclude_category) = &params.exclude_category {
            query_params.insert("excludecategory".to_string(), exclude_category.join(","));
        }

        // Add language parameter to query params
        if let Some(language) = &params.language {
            query_params.insert("language".to_string(), language.clone());
        }

        // Add priority_domain parameter to query params
        if let Some(priority_domain) = &params.priority_domain {
            query_params.insert("prioritydomain".to_string(), priority_domain.clone());
        }

        // Make the GET request to the sources endpoint
        self.get("sources", Some(query_params))
    }
}

/// Parameters for the `get_news_sources` method.
#[derive(Debug, Default)]
pub struct GetNewsSourcesParams {
    /// Unique identifier of the news source.\
    /// Max no. of id could be added: 50
    pub id: Option<Vec<String>>,
    /// Country code for the news sources.\
    /// Max no. of country could be added: 5.\
    /// Examples: "hk", "us", "wo"
    pub country: Option<Vec<String>>,
    /// Category for the news sources.\
    /// Max no. of category could be added: 5.\
    /// Possible values: "business", "crime", "domestic", "education", "entertainment", "environment", "food", "health", "lifestyle", "other", "politics", "science", "sports", "technology", "top", "tourism", "world"
    pub category: Option<Vec<String>>,
    /// Exclude category for the news sources.\
    /// Max no. of category could be added: 5.\
    /// Exclusive with category.\
    /// Possible values: "business", "crime", "domestic", "education", "entertainment", "environment", "food", "health", "lifestyle", "other", "politics", "science", "sports", "technology", "top", "tourism", "world"
    pub exclude_category: Option<Vec<String>>,
    /// Language code for the news sources.\
    /// Max no. of language could be added: 5.
    pub language: Option<String>,
    /// Priority domain for the news articles.\
    /// Top: Fetches news articles from the top 10% of the news domains\
    /// Medium: Fetches news articles from the top 30% of the news domains. It means it already includes all the news articles of "top" priority.\
    /// Low: Fetches news articles from the top 50% of the news domains. It means it already includes all the news articles of "top" and "medium" priorities.
    pub priority_domain: Option<String>,
}

impl GetNewsSourcesParams {
    /// Creates a new `GetNewsSourcesParams` with default values.
    ///
    /// This method sets the default values for all parameters, which are:
    ///
    /// * `id`: `None`
    /// * `country`: `None`
    /// * `category`: `None`
    /// * `exclude_category`: `None`
    /// * `language`: `None`
    /// * `priority_domain`: `None`
    ///
    /// This allows you to easily create a `GetNewsSourcesParams` object without having to specify all the parameters manually.
    pub fn default() -> Self {
        GetNewsSourcesParams {
            id: None,
            country: None,
            category: None,
            exclude_category: None,
            language: None,
            priority_domain: None,
        }
    }
}
