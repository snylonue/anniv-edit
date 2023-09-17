use anni_repo::prelude::TrackType;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AlbumIdentifier(pub Uuid); // uuid

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct TrackIdentifier {
    pub album_id: Uuid,
    pub disc_id: u32,
    pub track_id: u32,
}

impl Into<anni_repo::prelude::TrackIdentifier> for TrackIdentifier {
    fn into(self) -> anni_repo::prelude::TrackIdentifier {
        anni_repo::prelude::TrackIdentifier {
            album_id: self.album_id,
            disc_id: self.disc_id,
            track_id: self.track_id,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DiscIdentifier {
    pub album_id: AlbumIdentifier,
    pub disc_id: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TrackInfo {
    pub title: String,
    pub artist: String,
    #[serde(rename = "type")]
    pub kind: TrackType,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TrackInfoWithAlbum {
    #[serde(flatten)]
    pub identifier: TrackIdentifier,
    #[serde(flatten)]
    pub track_info: TrackInfo,
    pub album_title: String,
}
