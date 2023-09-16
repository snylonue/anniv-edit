pub mod anniv;
pub mod error;

use reqwest::{Client, ClientBuilder, Url};
use ring::digest::{digest, SHA256};
use serde_json::json;
use snafu::{ResultExt, Snafu};

use anniv::{playlist::PlaylistInfo, AnnivResponse, Playlist, UserInfo};

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

    pub async fn playlist(&self, id: &str) -> Result<Playlist, Error> {
        let mut url = self.url.join("api/playlist").context(UrlSnafu)?;
        url.query_pairs_mut().clear().append_pair("id", id);

        self.client
            .get(url)
            .send()
            .await
            .context(NetworkSnafu {
                endpoint: "/api/playlist",
            })?
            .json::<AnnivResponse<Playlist>>()
            .await
            .context(NetworkSnafu {
                endpoint: "/api/playlist",
            })?
            .into_result()
            .context(AnnivSnafu)
    }

    pub async fn playlists(&self, user_id: Option<&str>) -> Result<Vec<PlaylistInfo>, Error> {
        let mut url = self.url.join("api/playlists").context(UrlSnafu)?;
        match user_id {
            Some(user_id) => {
                url.query_pairs_mut()
                    .clear()
                    .append_pair("user_id", user_id);
            }
            _ => {}
        }

        self.client
            .get(url)
            .send()
            .await
            .context(NetworkSnafu {
                endpoint: "/api/playlists",
            })?
            .json::<AnnivResponse<Vec<PlaylistInfo>>>()
            .await
            .context(NetworkSnafu {
                endpoint: "/api/playlists",
            })?
            .into_result()
            .context(AnnivSnafu)
    }

    // pub async fn playlist(&self) -> Result<>
}
