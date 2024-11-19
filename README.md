# DAV-Save-Tool
CLI tool to read and manipulate BioWare's save files used in Dragon Age: Veilguard.
![](https://i.imgur.com/LFSuStJ.png)    
[Pre-compiled binaries](https://github.com/Sorrow446/DAV-Save-Tool/releases)

## Commands  
|Name|Description|Example|
| --- | --- | --- |
|dump-blocks|Parses and decompresses all blocks, then writes them locally.|`davst.exe dump-blocks -i "0-440065 Kalais-Save 5 #82.csav"`
|dump-metadata|Parses the metadata block and writes it to a JSON file.|`davst.exe dump-metadata -i "0-440065 Kalais-Save 5 #82.csav"`

## Goal
Goal is to be able to inject others' appeances into other saves. The game can only do this at the start of a new save.

## Example meta output
```json
{
  "faction": "ShadowDragons",
  "lineage": "Human",
  "arche_type": "Mage",
  "character_name": "Kalais",
  "quest_id": 2371269104,
  "request_id": 3169160734052889072,
  "active_career": 450626560,
  "key_binding_profile": "Mage",
  "after_point_of_no_return": false,
  "character_level": 6,
  "difficulty": "Adventurer",
  "voice_tone": "Medium",
  "voice": "FeminineOne",
  "pronouns": "SheHer",
  "gender": "Female",
  "transition_point_name": "Nev_1_1_330_ReachRelicHandout",
  "version_two": 24,
  "project_data": 508,
  "post_streaming_install": true,
  "cdur": 15989,
  "playtime": 17939,
  "save_type": "Manual",
  "description": "Save 10",
  "nexus_session_id": 1,
  "session_id": "6731e42db4c841697c3e31ed",
  "buildcl": 1394350,
  "unix_timestamp": 1731324158,
  "date_time": 1731324158,
  "expansion": [
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0
  ],
  "save_file_version": 3,
  "project": 4,
  "licensee_version": "6",
  "version": 30,
  "guid": "{4250c690-a01f-11ef-b0a8-ee561f50b1dd}",
  "checkpoint_id": 737877733
}
```

## Some notes
- Body proportions are stored as f32s, but the game clamps them to 1 so setting them out of range in your save won't work. Modifying the floats' boundaries in Frosty Editor doesn't seem to work either.
- The metadata's stored in the first block. It contains stuff like char name, level, playtime.
- The appearance data's stored in the second block. It also contains the player's inventory.
