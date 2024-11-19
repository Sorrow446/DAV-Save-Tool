use std::io::Cursor;
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;
use crate::bw_save_game_metadata_reader::enums::*;

pub struct BWSaveGameMetadataReader {
    pub(crate) c: Cursor<Vec<u8>>,
    pub(crate) buffer: Vec<u8>,
    pub(crate) metadata: BWSaveGameMetadata,
}

#[derive(Debug, Default, Serialize)]
pub struct BWSaveGameMetadata {
    pub faction: CharacterFaction,
    pub lineage: CharacterLineage,
    pub arche_type: CharacterArchetype,
    pub character_name: String,
    pub quest_id: u32,
    pub request_id: i64,
    pub active_career: u32,
    pub key_binding_profile: KeyBindingProfile,
    pub after_point_of_no_return: bool,
    pub character_level: u32,
    pub difficulty: Difficulty,
    pub voice_tone: CharacterVoiceTone,
    pub voice: CharacterVoice,
    pub pronouns: CharacterPronouns,
    pub gender: CharacterGender,
    pub transition_point_name: String,
    pub version_two: u32,
    pub project_data: u16,
    pub post_streaming_install: bool,
    pub cdur: u32,
    pub playtime: u32,
    pub save_type: SaveType,
    pub description: String,
    pub nexus_session_id: u16,
    pub session_id: String,
    pub buildcl: u32,
    pub unix_timestamp: i64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub date_time: DateTime<Utc>,
    pub expansion: [u8; 9],
    pub save_file_version: i32,
    pub project: u32,
    pub licensee_version: String,
    pub version: u32,
    #[serde(with = "uuid::serde::braced")]
    pub guid: Uuid,
    pub checkpoint_id: u32,
}
