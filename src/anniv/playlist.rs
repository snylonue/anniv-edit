use anni_repo::prelude::TrackType;
use serde::{Deserialize, Serialize};

use crate::anniv::types::{AlbumIdentifier, DiscIdentifier, TrackIdentifier, TrackInfo};

pub type PlaylistCover = Option<DiscIdentifier>;
pub type PlaylistPatchItem = PlaylistItem;
pub type PlaylistPatchItemWithId = PlaylistItemWithId;

#[derive(Debug, Clone, Deserialize)]
pub struct PlaylistInfo {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub owner: String,
    pub is_public: bool,
    pub cover: PlaylistCover,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BasePlaylistItemWithId<Info> {
    #[serde(rename = "type")]
    pub kind: String,
    pub description: Option<String>,
    pub id: String,
    pub info: Info,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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
    PlaylistItemPlainTrack(BasePlaylistItem<TrackIdentifier>),
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PatchPlaylistCommand {
    Info,
    Append,
    Remove,
    Reorder,
    Replace,
}

#[derive(Debug, Clone, Serialize)]
pub struct PatchedPlaylistInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<PlaylistCover>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BasePatchPlaylistBody<T> {
    pub id: String,
    pub command: PatchPlaylistCommand,
    pub payload: T,
}

impl BasePatchPlaylistBody<PatchedPlaylistInfo> {
    pub fn new_info(id: String, payload: PatchedPlaylistInfo) -> Self {
        Self {
            id,
            command: PatchPlaylistCommand::Info,
            payload,
        }
    }
}

impl BasePatchPlaylistBody<Vec<PlaylistPatchItem>> {
    pub fn new_append(id: String, payload: Vec<PlaylistPatchItem>) -> Self {
        Self {
            id,
            command: PatchPlaylistCommand::Append,
            payload,
        }
    }
}

impl BasePatchPlaylistBody<Vec<String>> {
    pub fn new_remove(id: String, payload: Vec<String>) -> Self {
        Self {
            id,
            command: PatchPlaylistCommand::Remove,
            payload,
        }
    }

    pub fn new_reorder(id: String, payload: Vec<String>) -> Self {
        Self {
            id,
            command: PatchPlaylistCommand::Reorder,
            payload,
        }
    }
}

impl BasePatchPlaylistBody<Vec<PlaylistPatchItemWithId>> {
    pub fn new_replace(id: String, payload: Vec<PlaylistPatchItemWithId>) -> Self {
        Self {
            id,
            command: PatchPlaylistCommand::Replace,
            payload,
        }
    }
}
