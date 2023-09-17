pub mod anniv;
pub mod error;

use reqwest::{Client, ClientBuilder, RequestBuilder, Url};
use ring::digest::{digest, SHA256};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::json;
use snafu::{ResultExt, Snafu};

use anniv::{
    playlist::PlaylistInfo, AnnivResponse, BasePatchPlaylistBody, CreatePlaylistBody, Playlist,
    UserInfo,
};

#[derive(Debug, Snafu)]
pub enum Error {
    NetworkError {
        endpoint: Option<&'static str>,
        source: reqwest::Error,
    },
    UrlError {
        source: url::ParseError,
    },
    #[snafu(display("AnnivError: {source}"))]
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
                .user_agent(concat!("anniv-edit/", env!("CARGO_PKG_VERSION")))
                .build()
                .unwrap(),
            url,
        }
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<UserInfo, Error> {
        let hash = digest(&SHA256, password.as_bytes());
        let hash = hex::encode(hash);

        send_request(
            self.client
                .post(self.url.join("api/user/login").context(UrlSnafu)?)
                .json(&json!({
                    "email": username,
                    "password": hash
                })),
            "/api/user/login",
        )
        .await
    }

    pub async fn playlist(&self, id: &str) -> Result<Playlist, Error> {
        let mut url = self.url.join("api/playlist").context(UrlSnafu)?;
        url.query_pairs_mut().clear().append_pair("id", id);

        send_request(self.client.get(url), "/api/playlist").await
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

        send_request(self.client.get(url), "/api/playlists").await
    }

    pub async fn create_playlist(&self, payload: CreatePlaylistBody) -> Result<Playlist, Error> {
        send_request(
            self.client
                .put(self.url.join("api/playlist").context(UrlSnafu)?)
                .json(&payload),
            "/api/playlist",
        )
        .await
    }

    pub async fn patch_playlist<T: Serialize>(
        &self,
        body: BasePatchPlaylistBody<T>,
    ) -> Result<Playlist, Error> {
        send_request(
            self.client
                .patch(self.url.join("api/playlist").context(UrlSnafu)?)
                .json(&body),
            "/api/playlist",
        )
        .await
    }

    pub async fn remove_playlist(&self, id: &str) -> Result<(), Error> {
        let mut url = self.url.join("api/playlist").context(UrlSnafu)?;
        url.query_pairs_mut().append_pair("id", id);

        send_request(self.client.delete(url), "/api/playlist").await
    }
}

async fn send_request<T: DeserializeOwned>(
    req: RequestBuilder,
    endpoint: &'static str,
) -> Result<T, Error> {
    req.send()
        .await
        .context(NetworkSnafu { endpoint })?
        .json::<AnnivResponse<T>>()
        .await
        .context(NetworkSnafu { endpoint })?
        .into_result()
        .context(AnnivSnafu)
}
