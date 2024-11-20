use std::error::Error;
use std::io::{self, Cursor, Read, Seek, SeekFrom};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::bw_save_game_metadata_reader::structs::*;
use crate::bw_save_game_metadata_reader::enums::*;

impl BWSaveGameMetadataReader {
    pub(crate) fn new(data: Vec<u8>) -> Self {
        Self {
            c: Cursor::new(data),
            buffer: vec![0; 1024 * 1024],
            metadata: BWSaveGameMetadata::default()
        }
    }

    pub(crate) fn read_bytes(&mut self, count: usize) -> io::Result<&[u8]> {
        self.c.read_exact(&mut self.buffer[..count])?;
        Ok(&self.buffer[..count])
    }

    pub(crate) fn read_string_of_len(&mut self, n : usize) -> Result<String, Box<dyn Error>> {
        let buf = self.read_bytes(n)?;
        let s = String::from_utf8(buf.to_vec())?;
        Ok(s)
    }

    fn read_guid(&mut self) -> Result<Uuid, Box<dyn Error>> {
        let guid_bytes = self.read_bytes(16)?;
        let guid = Uuid::from_bytes_le(guid_bytes.try_into()?);
        Ok(guid)
    }

    pub(crate) fn seek_from_start(&mut self, n: u64) -> io::Result<u64> {
        self.c.seek(SeekFrom::Start(n))
    }

    fn seek_from_current(&mut self, n: i64) -> io::Result<u64> {
        self.c.seek(SeekFrom::Current(n))
    }

    fn read_date_time(&mut self) -> Result<DateTime<Utc>, Box<dyn Error>> {
        // 0x15
        self.seek_from_current(1)?;

        let date_time_string = self.read_null_terminated_string()?;

        let timestamp = DateTime::parse_from_rfc3339(&date_time_string)?
            .with_timezone(&Utc);

        Ok(timestamp)
    }

    pub(crate) fn read_u16_le(&mut self) -> Result<u16, Box<dyn Error>> {
        let buf = self.read_bytes(2)?;
        let arr: [u8; 2] = buf.try_into()?;
        Ok(u16::from_le_bytes(arr))
    }

    pub(crate) fn read_i32_le(&mut self) -> Result<i32, Box<dyn Error>> {
        let buf = self.read_bytes(4)?;
        let arr: [u8; 4] = buf.try_into()?;
        Ok(i32::from_le_bytes(arr))
    }

    pub(crate) fn read_u32_le(&mut self) -> Result<u32, Box<dyn Error>> {
        let buf = self.read_bytes(4)?;
        let arr: [u8; 4] = buf.try_into()?;
        Ok(u32::from_le_bytes(arr))
    }

    pub(crate) fn read_i64_le(&mut self) -> Result<i64, Box<dyn Error>> {
        let buf = self.read_bytes(8)?;
        let arr: [u8; 8] = buf.try_into()?;

        Ok(i64::from_le_bytes(arr))
    }

    pub(crate) fn read_u8(&mut self) -> io::Result<u8> {
        let mut buf = [0; 1];
        self.c.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    pub(crate) fn read_null_terminated_string(&mut self) -> Result<String, Box<dyn Error>> {
        let mut buffer: Vec<u8> = Vec::new();
        let mut byte = [0u8; 1];

        loop {
            match self.c.read(&mut byte) {
                Ok(0) => break,
                Ok(_) if byte[0] == 0 => break,
                Ok(_) => buffer.push(byte[0]),
                Err(e) => return Err(e.into()),
            }
        }

        let s = String::from_utf8(buffer)?;
        Ok(s)
    }

    fn read_name_string(&mut self) -> Result<String, Box<dyn Error>>  {
        // Prefix byte.
        self.seek_from_current(1)?;
        let name = self.read_null_terminated_string()?;
        Ok(name)
    }

    fn read_bool(&mut self) -> Result<bool, Box<dyn Error>> {
        let value = self.read_u8()?;
        let value_bool = match value {
            0x0 => false,
            0x1 => true,
            _ => return Err(format!("Invalid bool value: {:#X}", value).into()),
        };

        Ok(value_bool)
    }

    fn get_stream_len(&mut self) -> Result<u64, Box<dyn Error>> {
        let original_pos = self.c.stream_position()?;
        let len = self.c.seek(SeekFrom::End(0))?;
        self.seek_from_start(original_pos)?;
        Ok(len)
    }

    // The order can vary so we need to parse in a loop. The prefix bytes don't have anything to do with types.
    pub(crate) fn parse_metadata(&mut self) -> Result<(), Box<dyn Error>> {
        let mut meta = BWSaveGameMetadata::default();
        let eof_offset = self.get_stream_len()?;

        self.seek_from_start(3)?;

        loop {
            let pos = self.c.stream_position()?;

            if pos >= eof_offset-1 {
                // println!("Reached EOF. pos: {}, EOF: {} ", pos, eof_offset);
                break
            }

            let name = self.read_name_string()?;
            match name.as_str() {
                "checkpointid" => {
                    meta.checkpoint_id = self.read_u32_le()?;
                }

                "faction" => {
                    let v = self.read_u32_le()?;
                    let faction = CharacterFaction::from_u32(v)
                        .ok_or(format!("Unknown CharacterFaction: {}", v))?;
                    meta.faction = faction;
                }
                "lineage" => {
                    let v = self.read_u32_le()?;
                    let lineage = CharacterLineage::from_u32(v)
                        .ok_or(format!("Unknown CharacterLineage: {}", v))?;
                    meta.lineage = lineage;

                }
                "archetype" => {
                    let v = self.read_u32_le()?;
                    let arche_type = CharacterArchetype::from_u32(v)
                        .ok_or(format!("Unknown CharacterArchetype: {}", v))?;
                    meta.arche_type = arche_type;

                }
                "charname" => {
                    // 0x07 prefix
                    self.seek_from_current(1)?;
                    meta.character_name = self.read_null_terminated_string()?;
                }
                "questid" => {
                    meta.quest_id = self.read_u32_le()?;
                }
                // Telemetry?
                "activecareer" => {
                    meta.active_career = self.read_u32_le()?;
                }
                // Telemetry?
                "requestid" => {
                    meta.request_id = self.read_i64_le()?;
                }
                "keybindingprofile" => {
                    let v = self.read_u32_le()?;
                    let key_binding_profile = KeyBindingProfile::from_u32(v)
                        .ok_or(format!("Unknown KeyBindingProfile: {}", v))?;
                    meta.key_binding_profile = key_binding_profile;
                    self.seek_from_current(1)?;
                }
                "level" => {
                    meta.character_level = self.read_u32_le()?;
                }
                "difficulty" => {
                    let v = self.read_u32_le()?;

                    let difficulty = Difficulty::from_u32(v)
                        .ok_or(format!("Unknown Difficulty: {}", v))?;
                    meta.difficulty = difficulty;
                }
                "tone" => {
                    let v = self.read_u32_le()?;
                    let voice_tone = CharacterVoiceTone::from_u32(v)
                        .ok_or(format!("Unknown CharacterVoiceTone: {}", v))?;
                    meta.voice_tone = voice_tone;
                }
                "voice" => {
                    let v = self.read_u32_le()?;
                    let voice = CharacterVoice::from_u32(v)
                        .ok_or(format!("Unknown CharacterVoice: {}", v))?;
                    meta.voice = voice;
                }
                "pronoun" => {
                    let v = self.read_u32_le()?;
                    let pronouns = CharacterPronouns::from_u32(v)
                        .ok_or(format!("Unknown CharacterPronouns: {}", v))?;
                    meta.pronouns = pronouns;

                }
                "gender" => {
                    let v = self.read_u32_le()?;
                    let gender = CharacterGender::from_u32(v)
                        .ok_or(format!("Unknown CharacterGender: {}", v))?;
                    meta.gender = gender;

                }
                "transitionpointname" => {
                    // Length prefix.
                    self.seek_from_current(1)?;
                    meta.transition_point_name = self.read_null_terminated_string()?;
                }
                "projdata" => {
                    meta.project_data = self.read_u16_le()?;
                }
                "poststreaminginstall" => {
                    meta.post_streaming_install = self.read_bool()?;
                }
                "afterpointofnoreturn" => {
                    meta.after_point_of_no_return = self.read_bool()?;
                    let b = self.read_u8()?;

                    // Key binding profile present.
                    if b != 0 {
                        self.seek_from_current(-1)?;
                    }
                }
                "cdur" => {
                    meta.cdur = self.read_u32_le()?;
                }
                "playtime" => {
                    meta.playtime = self.read_u32_le()?;
                }
                "type" => {
                    // 0x17 prefix
                    self.seek_from_current(1)?;
                    let v = self.read_null_terminated_string()?;
                    let save_type = SaveType::from_str(&v)
                        .ok_or(format!("Unknown SaveType: {}", v))?;
                    meta.save_type = save_type;
                }
                "description" => {
                    // Length prefix.
                    self.seek_from_current(1)?;
                    meta.description = self.read_null_terminated_string()?
                }
                "nexussessionid" => {
                    meta.nexus_session_id = self.read_u16_le()?;
                }
                "sessionid" => {
                    // Length prefix, 0x19.
                    self.seek_from_current(1)?;
                    meta.session_id = self.read_null_terminated_string()?;
                }
                "buildcl" => {
                    meta.buildcl = self.read_u32_le()?
                }
                "unixtimeseconds" => {
                    meta.unix_timestamp = self.read_i64_le()?
                }
                "time" => {
                    meta.date_time = self.read_date_time()?;
                }
                "expansion" => {
                    meta.expansion = self.read_bytes(9)?.try_into()?;
                }
                "savefileversion" => {
                    meta.save_file_version = self.read_i32_le()?
                }
                "project" => {
                    meta.project = self.read_u32_le()?
                }
                "licenseeversion" => {
                    meta.licensee_version = self.read_string_of_len(1)?
                }
                "version" => {
                    let v = self.read_u32_le()?;
                    if meta.version == 0 {
                        meta.version = v;
                    } else {
                        meta.version_two = v;
                    }

                }
                "uid" => {
                    meta.guid = self.read_guid()?;
                }
                _ => {
                    return Err(format!("Unknown name: {}", name).into());
                }
            }

        }

        self.metadata = meta;

        Ok(())

    }

}
