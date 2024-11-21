# DAV-Save-Tool
CLI tool to read and manipulate BioWare's save files used in Dragon Age: The Veilguard.
![](https://i.imgur.com/NcP1iin.png)
![](https://i.imgur.com/McIA6HY.png)    
[Pre-compiled binaries](https://github.com/Sorrow446/DAV-Save-Tool/releases)

## Commands  
|Name|Description|Example|Input|Output|
| --- | --- | --- | --- | --- |
|dump-blocks/db|Parses and decompresses all blocks, then writes them locally.|`davst.exe db -i "0-440065 Kalais-Save 5 #82.csav"`|Path of save file.|Path of an output folder or none for current dir.|
|dump-metadata/dm|Parses the metadata block and writes it to a JSON file.|`davst.exe dm -i "0-440065 Kalais-Save 5 #82.csav"`|Path of save file.|Path of an output folder or none for current dir.|
|inject-appearance/ia|Extracts the appearance data from the source save file and injects it into the destination save. Everything like story progression, inventory etc. will be retained in the source save, **but currently The Inquisitor's appearance is also carried over.** This isn't how I want it, but it is how  it is. **Genders and races must match.**|`davst.exe ia -i "0-439076 decision0.csav" -o "0-440065 Kalais-Save 5 #82.csav"`|Path of source save file with the apperance data you want to inject.|Path of the destination save file to be injected into.|

## Goal
~~Goal is to be able to inject others' appeances into other saves. The game can only do this at the start of a new save.~~    
Done :).

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

# Disclaimer   
- I won't be responsible for the rare chance of your saves getting corrupted. Absolutely back them up.
- DAVST has no partnership, sponsorship or endorsement with BioWare.
