pub mod anniv;
pub mod error;

use reqwest::{Client, ClientBuilder, Url};
use ring::digest::{digest, SHA256};
use serde_json::json;
use snafu::{ResultExt, Snafu};

use anniv::{AnnivResponse, UserInfo};

#[derive(Debug, Snafu)]
pub enum Error {
    NetworkError {
        endpoint: Option<&'static str>,
        source: reqwest::Error,
    },
    UrlError {
        source: url::ParseError,
    },
    AnnivError {
        source: anniv::Error,
    },
}

pub struct AnnivClient {
    client: Client,
    url: Url,
}

impl AnnivClient {
    pub fn new(url: Url) -> Self {
        Self {
            client: ClientBuilder::new()
                .cookie_store(true)
                .no_proxy()
                .build()
                .unwrap(),
            url,
        }
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<UserInfo, Error> {
        let hash = digest(&SHA256, password.as_bytes());
        let hash = hex::encode(hash);

        self.client
            .post(self.url.join("api/user/login").context(UrlSnafu)?)
            .json(&json!({
                "email": username,
                "password": hash
            }))
            .send()
            .await
            .context(NetworkSnafu {
                endpoint: "/user/login",
            })?
            .json::<AnnivResponse<UserInfo>>()
            .await
            .context(NetworkSnafu {
                endpoint: "/user/login",
            })?
            .into_result()
            .context(AnnivSnafu)
    }

    // pub async fn playlist(&self) -> Result<>
}
