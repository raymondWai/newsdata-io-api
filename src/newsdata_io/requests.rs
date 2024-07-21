use std::collections::HashMap;

use super::NewsdataIO;
use crate::Error;
use crate::{ApiResult, Json};

use log::{debug, error};

pub trait Requests {
    fn get(&self, sub_url: &str, query_params: Option<HashMap<String, String>>) -> ApiResult<Json>;
}

impl Requests for NewsdataIO {
    fn get(&self, sub_url: &str, query_params: Option<HashMap<String, String>>) -> ApiResult<Json> {
        let mut request = self
            .agent
            .get(&(format!("https://newsdata.io/api/1/{}", sub_url)))
            .query("apikey", self.auth.get_api_key().as_str());
        match query_params {
            Some(params) => {
                for (key, value) in params {
                    request = request.query(key.as_str(), value.as_str());
                }
            }
            None => {}
        };
        let response = request.set("Content-Type", "application/json").call();
        deal_response(response, sub_url)
    }
}

fn deal_response(response: Result<ureq::Response, ureq::Error>, sub_url: &str) -> ApiResult<Json> {
    match response {
        Ok(response) => {
            let json = response.into_json::<Json>().unwrap();
            debug!("<== ✔️\n\tDone api: {sub_url}, resp: {json}");
            Ok(json)
        }
        Err(err) => match err {
            ureq::Error::Status(status, response) => {
                let err_msg = response.into_json::<Json>().unwrap();
                error!("<== ❌\n\tError api: {sub_url}, status: {status}, error: {err_msg}");
                return Err(Error::ApiError(format!("{err_msg}")));
            }
            ureq::Error::Transport(e) => {
                error!("<== ❌\n\tError api: {sub_url}, error: {:?}", e.to_string());
                return Err(Error::RequestError(e.to_string()));
            }
        },
    }
}
