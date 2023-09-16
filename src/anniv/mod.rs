pub mod playlist;
pub mod types;

use serde::Deserialize;
use snafu::Snafu;

use playlist::PlaylistItemWithId;

#[derive(Debug, Snafu)]
#[snafu(display("Error code {status}: {message}"))]
pub struct Error {
    status: i32,
    message: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct UserInfo {
    pub user_id: String,
    pub email: String,
    pub nickname: String,
    pub avatar: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Playlist {
    pub items: Vec<PlaylistItemWithId>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AnnivResponse<T> {
    pub status: i32,
    pub message: Option<String>,
    pub data: Option<T>,
}

impl<T> AnnivResponse<T> {
    pub fn into_result(self) -> Result<T, Error> {
        match self.status {
            0 => Ok(self
                .data
                .expect("`data` should not be `None` when `status` is 0")),
            status => Err(Error {
                status,
                message: self.message.unwrap_or_default(),
            }),
        }
    }
}

impl<T> Into<Result<T, Error>> for AnnivResponse<T> {
    fn into(self) -> Result<T, Error> {
        self.into_result()
    }
}
