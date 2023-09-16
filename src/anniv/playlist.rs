use anni_repo::prelude::TrackType;
use serde::Deserialize;

use crate::anniv::types::{AlbumIdentifier, DiscIdentifier, TrackInfo};

pub type PlaylistCover = Option<DiscIdentifier>;

#[derive(Debug, Clone, Deserialize)]
pub struct PlaylistInfo {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub owner: String,
    pub is_public: bool,
    pub cover: PlaylistCover,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BasePlaylistItemWithId<Info> {
    #[serde(rename = "type")]
    pub kind: String,
    pub description: Option<String>,
    pub id: String,
    pub info: Info,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum PlaylistItemWithId {
    PlaylistItemDummyTrack(BasePlaylistItemWithId<TrackType>),
    PlaylistItemTrack(BasePlaylistItemWithId<TrackInfo>),
    PlaylistItemAlbum(BasePlaylistItemWithId<AlbumIdentifier>),
}
