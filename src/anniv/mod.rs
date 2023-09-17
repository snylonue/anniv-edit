pub mod playlist;
pub mod types;

pub use playlist::BasePatchPlaylistBody;

use serde::{Deserialize, Serialize};
use snafu::Snafu;

use playlist::{PlaylistCover, PlaylistItemWithId, PlaylistPatchItem};

#[derive(Debug, Snafu)]
#[snafu(display("Error {status}: {message}"))]
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
    pub id: String,
    pub description: String,
    pub owner: String,
    pub is_public: bool,
    pub items: Vec<PlaylistItemWithId>,
    pub cover: PlaylistCover,
    pub last_modified: u64,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreatePlaylistBody {
    pub name: String,
    pub description: Option<String>,
    pub is_public: bool,
    pub cover: PlaylistCover,
    pub items: Vec<PlaylistPatchItem>,
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
