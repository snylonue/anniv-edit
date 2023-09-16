use anni_repo::prelude::TrackType;
use serde::{Deserialize, Serialize};

use crate::anniv::types::{AlbumIdentifier, DiscIdentifier, TrackInfo};

pub type PlaylistCover = Option<DiscIdentifier>;
pub type PlaylistPatchItem = PlaylistItem;

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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BasePlaylistItem<Info> {
    #[serde(rename = "type")]
    pub kind: String,
    pub description: Option<String>,
    pub info: Info,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PlaylistItem {
    PlaylistItemDummyTrack(BasePlaylistItem<TrackType>),
    PlaylistItemTrack(BasePlaylistItem<TrackInfo>),
    PlaylistItemAlbum(BasePlaylistItem<AlbumIdentifier>),
}
