# DAV-Save-Tool
Tool to read and manipulate BioWare's CSAV save files used in Dragon Age: Veilguard.
![](https://i.imgur.com/g1fyuB4.png)    
[Pre-compiled binaries](https://github.com/Sorrow446/DAV-Save-Tool/releases)

## Commands  
|Name|Description|Example|
| --- | --- | --- |
|dump-blocks|Parses and decompresses all blocks, then writes them locally.|`davst.exe dump-blocks -i "0-440065 Kalais-Save 5 #82.csav"`

## Goal
Goal is to be able to inject others' appeances into other saves. The game can only do this at the start of a new save.

## Specs
|Name|Description|
| --- | --- |
|Magic|`<!--DASC`|
|Save version|u32|
|unk_001|u32|
|Block two decomp size|u64|
|Block two comp size|u64|
|Block one decomp size|u64|
|Block one comp size|u64|
|unk_002|u64|
|Block one comp data|gzip, default comp level with header|
|Block two comp data|^|

## Some notes
- I'm able to modify blocks and rebuild saves and have the game load them fine. Body proportions are stored as f32s, but the game clamps them to 1 so setting them out of range in your save won't work.
- The character's appearance and inventory are stored in blocks inside the second block. They start with `.....name...RPGPlayerExtent`?
- There's two unknown lots of data in the header. If you know what they store, let me know.
- The version u32 may be a block count and not a version, though, I'm sure it's just a version.
