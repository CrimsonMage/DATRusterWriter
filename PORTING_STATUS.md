# DatReaderWriter Rust Port Status

This document tracks what has been ported from `C:\Repo\NewAC_Client\vitaeum\ref\DatReaderWriter` into this crate and what remains.

## Rules

See `PORTING_RULES.md` for the tracking contract used during this port.

## Scope Note

- Primary target remains read functionality, but write parity is also in scope where it can be ported explicitly and safely.
- Write functionality is no longer just incidental: we are also porting explicit write-side parity where it meaningfully supports future DAT authoring and roundtrip verification.
- When read and write differ in priority, read support wins.

## Current Summary

- Reference library size: 338 C# files, about 938 KB of source
- Rust crate scaffold: created
- Progress tracker: created
- Rules document: created
- Read-first porting strategy: adopted
- `cargo test`: passing

## Port Map

| Source | Rust | Status | Notes |
|---|---|---|---|
| `DatReaderWriter/Generated/Enums/DatFileType.generated.cs` | `src/Generated/Enums/DatFileType.rs` | Verified | First generated enum port |
| `DatReaderWriter/Generated/Enums/DBObjType.generated.cs` | `src/Generated/Enums/DBObjType.rs` | Ported | Generated DB object type enum port |
| `DatReaderWriter/Generated/Enums/DBObjHeaderFlags.generated.cs` | `src/Generated/Enums/DBObjHeaderFlags.rs` | Ported | Header flag bitflags |
| `DatReaderWriter/Generated/Enums/TextureType.generated.cs` | `src/Generated/Enums/TextureType.rs` | Ported | Texture type wrapper |
| `DatReaderWriter/Generated/Enums/PixelFormat.generated.cs` | `src/Generated/Enums/PixelFormat.rs` | Ported | Pixel format wrapper with indexed texture helpers |
| `DatReaderWriter/Generated/Enums/MotionCommand.generated.cs` | `src/Generated/Enums/MotionCommand.rs` | Ported | Motion command wrapper |
| DatReaderWriter/Generated/Enums/MotionDataFlags.generated.cs | src/Generated/Enums/MotionDataFlags.rs | Ported | Motion data flag bitflags |
| DatReaderWriter/Generated/Enums/MotionStance.generated.cs | src/Generated/Enums/MotionStance.rs | Ported | Explicit named combat/motion stance constants for CombatTable reads |
| `DatReaderWriter/Generated/Enums/PartsMask.generated.cs` | `src/Generated/Enums/PartsMask.rs` | Ported | Region parts bitflags now used in typed Region reads |
| `DatReaderWriter/Generated/Enums/GfxObjFlags.generated.cs` | `src/Generated/Enums/GfxObjFlags.rs` | Ported | Read-side mesh flags |
| `DatReaderWriter/Generated/Enums/SurfaceType.generated.cs` | `src/Generated/Enums/SurfaceType.rs` | Ported | Surface/material bitflags |
| `DatReaderWriter/Generated/Enums/AnimationHookType.generated.cs` | `src/Generated/Enums/AnimationHookType.rs` | Ported | Hook discriminator constants |
| `DatReaderWriter/Generated/Enums/AnimationHookDir.generated.cs` | `src/Generated/Enums/AnimationHookDir.rs` | Ported | Hook direction constants |
| DatReaderWriter/Generated/Enums/AnimationFlags.generated.cs | src/Generated/Enums/AnimationFlags.rs | Ported | Read-side animation presence bitflags |
| DatReaderWriter/Generated/Enums/AttackHeight.generated.cs | src/Generated/Enums/AttackHeight.rs | Ported | Explicit named combat attack-height constants for CombatTable reads |
| DatReaderWriter/Generated/Enums/AttackType.generated.cs | src/Generated/Enums/AttackType.rs | Ported | Explicit named combat attack-type constants for CombatTable reads |
| `DatReaderWriter/Generated/Enums/ParentLocation.generated.cs` | `src/Generated/Enums/ParentLocation.rs` | Ported | Full named parent-location constants now mirrored into the Rust wrapper surface |
| `DatReaderWriter/Generated/Enums/Placement.generated.cs` | `src/Generated/Enums/Placement.rs` | Ported | Full named placement constants now mirrored into the Rust wrapper surface |
| `DatReaderWriter/Generated/Enums/PlayScript.generated.cs` | `src/Generated/Enums/PlayScript.rs` | Ported | Explicit named play-script constants now mirrored into the Rust wrapper surface |
| `DatReaderWriter/Generated/Enums/SetupFlags.generated.cs` | `src/Generated/Enums/SetupFlags.rs` | Ported | Setup optional-data bitflags |
| DatReaderWriter/Generated/Enums/SkillId.generated.cs | src/Generated/Enums/SkillId.rs | Ported | Full named skill-id constant surface now mirrored into the Rust wrapper |
| `DatReaderWriter/Generated/Enums/EmitterType.generated.cs` | `src/Generated/Enums/EmitterType.rs` | Ported | Full named particle-emitter constants now mirrored into the Rust wrapper surface |
| `DatReaderWriter/Generated/Enums/ParticleType.generated.cs` | `src/Generated/Enums/ParticleType.rs` | Ported | Full named particle-type constants now mirrored into the Rust wrapper surface |
| `DatReaderWriter/Generated/Enums/VertexType.generated.cs` | `src/Generated/Enums/VertexType.rs` | Ported | Full named vertex-type constants now mirrored into the Rust wrapper surface |
| `DatReaderWriter/Generated/Enums/CullMode.generated.cs` | `src/Generated/Enums/CullMode.rs` | Ported | Full named cull-mode constants now mirrored into the Rust wrapper surface while keeping the existing CLOCKWISE compatibility alias |
| `DatReaderWriter/Generated/Enums/StipplingType.generated.cs` | `src/Generated/Enums/StipplingType.rs` | Ported | Polygon stippling bitflags |
| `DatReaderWriter/Enums/BSPNodeType.cs` | `src/Generated/Enums/BSPNodeType.rs` | Ported | Full named BSP node constants now mirrored into the Rust wrapper surface |
| `DatReaderWriter/Generated/Enums/Sound.generated.cs` | `src/Generated/Enums/Sound.rs` | Ported | Explicit named sound constants now mirrored into the Rust wrapper surface |
| `DatReaderWriter/Generated/Enums/TerrainTextureType.generated.cs` | `src/Generated/Enums/TerrainTextureType.rs` | Ported | Full named terrain texture constant surface now mirrored into the Rust wrapper |
| `DatReaderWriter/Options/DatAccessType.cs` | `src/Options/DatAccessType.rs` | Verified | Read-first access enum |
| `DatReaderWriter/Options/FileCachingStrategy.cs` | `src/Options/FileCachingStrategy.rs` | Verified | File cache enum |
| `DatReaderWriter/Options/IndexCachingStrategy.cs` | `src/Options/IndexCachingStrategy.rs` | Verified | Index cache enum |
| `DatReaderWriter/Options/DatDatabaseOptions.cs` | `src/Options/DatDatabaseOptions.rs` | Verified | Defaulted Rust options struct |
| `DatReaderWriter/Options/DatCollectionOptions.cs` | `src/Options/DatCollectionOptions.rs` | Verified | Path and per-dat strategy overrides ported |
| `DatReaderWriter/Lib/IO/IPackable.cs` | `src/Lib/IO/IPackable.rs` | Ported | Trait scaffolded for crate shape |
| `DatReaderWriter/Lib/IO/IUnpackable.cs` | `src/Lib/IO/IUnpackable.rs` | Ported | Trait scaffolded for crate shape |
| `DatReaderWriter/Lib/IO/IDBObj.cs` | `src/Lib/IO/IDBObj.rs` | Partial | Expanded into typed DB object trait with metadata hooks |
| `DatReaderWriter/Lib/IO/ObjectFactory.cs` | `src/Lib/IO/ObjectFactory.rs` | Partial | Minimal generic constructor helper only |
| `DatReaderWriter/Lib/IO/DatBinReader.cs` | `src/Lib/IO/DatBinReader.rs` | Verified | Primitive reader, seeking, and remaining-byte helpers ported and tested |
| `DatReaderWriter/Lib/IO/DatBinWriter.cs` | `src/Lib/IO/DatBinWriter.rs` | Verified | Retained mainly for parity and self-tests |
| `DatReaderWriter/Lib/IO/DatHeader.cs` | `src/Lib/IO/DatHeader.rs` | Verified | Header model and pack/unpack tested |
| `DatReaderWriter/Lib/IO/BlockAllocators/IDatBlockAllocator.cs` | `src/Lib/IO/BlockAllocators/IDatBlockAllocator.rs` | Partial | Expanded to include explicit write-capable allocator operations alongside the existing read contract |
| `DatReaderWriter/Lib/IO/BlockAllocators/BaseBlockAllocator.cs` | `src/Lib/IO/BlockAllocators/BaseBlockAllocator.rs` | Scaffolded | Placeholder while read logic lives in concrete allocator |
| `DatReaderWriter/Lib/IO/BlockAllocators/MemoryMappedBlockAllocator.cs` | `src/Lib/IO/BlockAllocators/MemoryMappedBlockAllocator.rs` | Partial | Read-only memory-mapped implementation |
| `DatReaderWriter/Lib/IO/BlockAllocators/StreamBlockAllocator.cs` | `src/Lib/IO/BlockAllocators/StreamBlockAllocator.rs` | Partial | Concrete file-backed allocator now supports explicit synchronous header init/version updates, free-block reservation, chained block read/write, and root-block updates; async parity remains pending |
| `DatReaderWriter/Lib/IO/DatBTree/DatBTreeFileFlags.cs` | `src/Lib/IO/DatBTree/DatBTreeFileFlags.rs` | Ported | Bitflags port |
| `DatReaderWriter/Lib/IO/DatBTree/DatBTreeFile.cs` | `src/Lib/IO/DatBTree/DatBTreeFile.rs` | Ported | Entry unpack and basic pack parity |
| `DatReaderWriter/Lib/IO/DatBTree/DatBTreeNode.cs` | `src/Lib/IO/DatBTree/DatBTreeNode.rs` | Verified | Read-path node pack/unpack symmetry plus explicit array mutation helpers for write-side B-tree operations tested |
| `DatReaderWriter/Lib/IO/DatBTree/DatBTreeReaderWriter.cs` | `src/Lib/IO/DatBTree/DatBTreeReaderWriter.rs` | Partial | Read-path lookup, enumeration, caching, and range traversal remain tested, and explicit synchronous write-side insert/update/delete behavior is now ported; async mutation parity remains pending |
| `DatReaderWriter/Lib/Attributes/DBObjTypeAttribute.cs` | `src/Lib/Attributes/DBObjTypeAttribute.rs` | Ported | Rust metadata descriptor |
| `DatReaderWriter/Lib/DBObjAttributeCache.cs` | `src/Lib/DBObjAttributeCache.rs` | Partial | Ported objects now resolve through a shared Rust attribute list with tested singular/range lookup; broader generated coverage remains pending |
| `DatReaderWriter/Types/DBObj.cs` | `src/Types/DBObj.rs` | Partial | Rust DB object base abstraction |
| DatReaderWriter/Types/AC1LegacyPStringBase.cs | src/Types/AC1LegacyString.rs | Partial | Byte-string read path only |
| DatReaderWriter/Types/PStringBase.cs | src/Types/PStringBase.rs | Ported | Generic packed string primitive for byte and UTF-16 string payloads |
| `DatReaderWriter/Generated/Types/ColorARGB.generated.cs` | `src/Types/ColorARGB.rs` | Ported | Color primitive |
| `DatReaderWriter/Types/QualifiedDataId.cs` | `src/Types/QualifiedDataId.rs` | Ported | Generic data-id wrapper with collection-backed typed resolution helpers |
| `DatReaderWriter/Generated/Types/AnimData.generated.cs` | `src/Types/AnimData.rs` | Ported | Motion animation entry |
| `DatReaderWriter/Generated/Types/MotionData.generated.cs` | `src/Types/MotionData.rs` | Ported | Motion data payload |
| `DatReaderWriter/Generated/Types/MotionCommandData.generated.cs` | `src/Types/MotionCommandData.rs` | Ported | Motion command map payload |
| `DatReaderWriter/Generated/Types/LandDefs.generated.cs` | `src/Types/LandDefs.rs` | Ported | Region terrain layout primitive |
| `DatReaderWriter/Generated/Types/GameTime.generated.cs` | `src/Types/GameTime.rs` | Ported | Region time settings root |
| `DatReaderWriter/Generated/Types/TimeOfDay.generated.cs` | `src/Types/TimeOfDay.rs` | Ported | Game time child type |
| `DatReaderWriter/Generated/Types/Season.generated.cs` | `src/Types/Season.rs` | Ported | Game time child type |
| `DatReaderWriter/Generated/Types/AmbientSoundDesc.generated.cs` | `src/Types/AmbientSoundDesc.rs` | Ported | Region sound child type |
| `DatReaderWriter/Generated/Types/AmbientSTBDesc.generated.cs` | `src/Types/AmbientSTBDesc.rs` | Ported | Region sound child type |
| `DatReaderWriter/Generated/Types/SoundDesc.generated.cs` | `src/Types/SoundDesc.rs` | Ported | Region sound container |
| `DatReaderWriter/Generated/Types/SceneType.generated.cs` | `src/Types/SceneType.rs` | Ported | Region scene child type |
| `DatReaderWriter/Generated/Types/SceneDesc.generated.cs` | `src/Types/SceneDesc.rs` | Ported | Region scene container |
| `DatReaderWriter/Generated/Types/TerrainAlphaMap.generated.cs` | `src/Types/TerrainAlphaMap.rs` | Ported | Terrain merge helper |
| `DatReaderWriter/Generated/Types/RoadAlphaMap.generated.cs` | `src/Types/RoadAlphaMap.rs` | Ported | Terrain merge helper |
| `DatReaderWriter/Generated/Types/TerrainTex.generated.cs` | `src/Types/TerrainTex.rs` | Ported | Terrain merge helper |
| `DatReaderWriter/Generated/Types/TMTerrainDesc.generated.cs` | `src/Types/TMTerrainDesc.rs` | Ported | Terrain merge helper |
| `DatReaderWriter/Generated/Types/TexMerge.generated.cs` | `src/Types/TexMerge.rs` | Ported | Terrain merge root |
| `DatReaderWriter/Generated/Types/LandSurf.generated.cs` | `src/Types/LandSurf.rs` | Ported | Terrain surface container |
| `DatReaderWriter/Generated/Types/TerrainType.generated.cs` | `src/Types/TerrainType.rs` | Ported | Terrain metadata type |
| `DatReaderWriter/Generated/Types/TerrainDesc.generated.cs` | `src/Types/TerrainDesc.rs` | Ported | Region terrain container |
| `DatReaderWriter/Generated/Types/RegionMisc.generated.cs` | `src/Types/RegionMisc.rs` | Ported | Region misc tail block |
| `DatReaderWriter/Generated/Types/SkyObject.generated.cs` | `src/Types/SkyObject.rs` | Ported | Region sky child type |
| `DatReaderWriter/Generated/Types/SkyObjectReplace.generated.cs` | `src/Types/SkyObjectReplace.rs` | Ported | Region sky child type |
| `DatReaderWriter/Generated/Types/SkyTimeOfDay.generated.cs` | `src/Types/SkyTimeOfDay.rs` | Ported | Region sky child type |
| `DatReaderWriter/Generated/Types/DayGroup.generated.cs` | `src/Types/DayGroup.rs` | Ported | Region sky child type |
| `DatReaderWriter/Generated/Types/SkyDesc.generated.cs` | `src/Types/SkyDesc.rs` | Ported | Region sky container |
| `DatReaderWriter/Generated/Types/Frame.generated.cs` | `src/Types/Frame.rs` | Ported | Shared transform primitive |
| `DatReaderWriter/Generated/Types/ObjectDesc.generated.cs` | `src/Types/ObjectDesc.rs` | Ported | Scene object placement record |
| `DatReaderWriter/Generated/Types/Vec2Duv.generated.cs` | `src/Types/Vec2Duv.rs` | Ported | Mesh UV primitive |
| `DatReaderWriter/Generated/Types/SWVertex.generated.cs` | `src/Types/SWVertex.rs` | Ported | Mesh vertex record |
| `DatReaderWriter/Generated/Types/VertexArray.generated.cs` | `src/Types/VertexArray.rs` | Ported | Mesh vertex container |
| `DatReaderWriter/Generated/Types/Sphere.generated.cs` | `src/Types/Sphere.rs` | Ported | BSP bounding primitive |
| `DatReaderWriter/Types/PortalRef.cs` | `src/Types/PortalRef.rs` | Ported | Drawing BSP portal reference |
| `DatReaderWriter/Generated/Types/Polygon.generated.cs` | `src/Types/Polygon.rs` | Ported | Mesh polygon record |
| `DatReaderWriter/Types/BSPTree.cs`, `DatReaderWriter/Types/PhysicsBSPNode.cs`, `DatReaderWriter/Types/DrawingBSPNode.cs`, `DatReaderWriter/Types/CellBSPNode.cs` | `src/Types/BSPTrees.rs` | Verified | Read-side BSP tree/node port collapsed into one Rust module; drawing, physics, and cell BSP behavior now match the current reference slices and are covered by focused tests |
| `DatReaderWriter/Generated/Types/AttackCone.generated.cs` | `src/Types/AttackCone.rs` | Ported | Physics script hook payload |
| `DatReaderWriter/Generated/Types/AnimationFrame.generated.cs` | `src/Types/AnimationFrame.rs` | Ported | Read-side animation/setup frame payload with explicit part-count helper |
| `DatReaderWriter/Generated/Types/AnimationPartChange.generated.cs` | `src/Types/AnimationPartChange.rs` | Ported | Read-side explicit packed part swap payload |
| DatReaderWriter/Generated/Types/ObjDesc.generated.cs | src/Types/ObjDesc.rs | Ported | Explicit object-description payload with sub-palettes, texture swaps, and animation part changes |
| DatReaderWriter/Generated/Types/SubPalette.generated.cs | src/Types/ObjDesc.rs | Ported | Collapsed into the explicit ObjDesc Rust module |
| DatReaderWriter/Generated/Types/TextureMapChange.generated.cs | src/Types/ObjDesc.rs | Ported | Collapsed into the explicit ObjDesc Rust module |
| `DatReaderWriter/Generated/Types/CylSphere.generated.cs` | `src/Types/CylSphere.rs` | Ported | Setup collision primitive |
| `DatReaderWriter/Generated/Types/LightInfo.generated.cs` | `src/Types/LightInfo.rs` | Ported | Setup attached light payload |
| `DatReaderWriter/Generated/Types/LocationType.generated.cs` | `src/Types/LocationType.rs` | Ported | Setup attachment point payload |
| `DatReaderWriter/Generated/Types/PhysicsScriptTableData.generated.cs` | `src/Types/PhysicsScriptTableData.rs` | Ported | Physics script table entry list |
| `DatReaderWriter/Generated/Types/ScriptAndModData.generated.cs` | `src/Types/ScriptAndModData.rs` | Ported | Physics script id plus modifier payload |
| `DatReaderWriter/Generated/Types/SoundData.generated.cs` | `src/Types/SoundData.rs` | Ported | Sound-table sound entry list |
| `DatReaderWriter/Generated/Types/SoundEntry.generated.cs` | `src/Types/SoundEntry.rs` | Ported | Wave reference with sound weights |
| `DatReaderWriter/Generated/Types/SoundHashData.generated.cs` | `src/Types/SoundHashData.rs` | Ported | Hash-keyed sound weights |
| `DatReaderWriter/Types/PackedQualifiedDataId.cs` | `src/Types/PackedQualifiedDataId.rs` | Ported | Packed known-type id wrapper with collection resolution helper |
| DatReaderWriter/Types/HashTable.cs | src/Types/HashTable.rs | Partial | Explicit read/write hash-table wrapper now supports broader primitive and string key/value surfaces plus `QualifiedDataId<T>` keys, and is covered by focused roundtrip tests; full generic parity with every reference generic case still remains |
| DatReaderWriter/Types/PackableHashTable.cs | src/Types/PackableHashTable.rs | Ported | Explicit packable hash-table wrapper now covers the setup-keyed clothing and numeric sub-palette tables used on the read path |
| DatReaderWriter/Generated/Types/CloSubPaletteRange.generated.cs | src/Types/CloSubPaletteRange.rs | Ported | Clothing sub-palette range payload |
| DatReaderWriter/Generated/Types/CloSubPalette.generated.cs | src/Types/CloSubPalette.rs | Ported | Clothing sub-palette entry with PalSet reference |
| DatReaderWriter/Generated/Types/CloSubPalEffect.generated.cs | src/Types/CloSubPalEffect.rs | Ported | Clothing sub-palette effect payload |
| DatReaderWriter/Generated/Types/CloTextureEffect.generated.cs | src/Types/CloTextureEffect.rs | Ported | Clothing texture swap payload |
| DatReaderWriter/Generated/Types/CloObjectEffect.generated.cs | src/Types/CloObjectEffect.rs | Ported | Clothing object effect payload with GfxObj reference |
| DatReaderWriter/Generated/Types/ClothingBaseEffect.generated.cs | src/Types/ClothingBaseEffect.rs | Ported | Clothing setup-keyed effect payload |
| DatReaderWriter/Generated/Types/CombatManeuver.generated.cs | src/Types/CombatManeuver.rs | Ported | Combat-table maneuver payload |
| DatReaderWriter/Generated/Types/StringTableString.generated.cs | src/Types/StringTableString.rs | Ported | String-table entry payload with packed UTF-16 strings and variable references |
| DatReaderWriter/Generated/Types/StringTableData.generated.cs | src/Types/StringTableData.rs | Ported | String-table data payload with variable names, values, comments, and strings |
| DatReaderWriter/Generated/Types/Position.generated.cs | src/Types/Position.rs | Ported | Position payload for starting-area locations |
| DatReaderWriter/Generated/Types/StartingArea.generated.cs | src/Types/StartingArea.rs | Ported | Character-creation starting area payload |
| DatReaderWriter/Generated/Types/SkillCG.generated.cs | src/Types/SkillCG.rs | Ported | Character-generation skill cost payload |
| DatReaderWriter/Generated/Types/TemplateCG.generated.cs | src/Types/TemplateCG.rs | Ported | Character-generation template payload |
| DatReaderWriter/Generated/Types/HairStyleCG.generated.cs | src/Types/HairStyleCG.rs | Ported | Character-generation hair style payload |
| DatReaderWriter/Generated/Types/EyeStripCG.generated.cs | src/Types/EyeStripCG.rs | Ported | Character-generation eye strip payload |
| DatReaderWriter/Generated/Types/FaceStripCG.generated.cs | src/Types/FaceStripCG.rs | Ported | Character-generation face strip payload |
| DatReaderWriter/Generated/Types/GearCG.generated.cs | src/Types/GearCG.rs | Ported | Character-generation gear option payload |
| DatReaderWriter/Generated/Types/SexCG.generated.cs | src/Types/SexCG.rs | Verified | Character-generation gender payload now reads setup/sound/motion/combat references and appearance options |
| DatReaderWriter/Generated/Types/HeritageGroupCG.generated.cs | src/Types/HeritageGroupCG.rs | Verified | Character-generation heritage group payload now reads starting areas, templates, skills, and gender table |
| `DatReaderWriter/Generated/Types/AnimationHook.generated.cs` and hook variants | `src/Types/AnimationHook.rs` | Partial | Read-side hook family collapsed into one Rust enum; unknown hook payloads are not preserved |
| `DatReaderWriter/Generated/Types/PhysicsScriptData.generated.cs` | `src/Types/PhysicsScriptData.rs` | Ported | Physics script timing + hook record |
| `DatReaderWriter/DatDatabase.cs` | `src/DatDatabase.rs` | Partial | Raw file entry lookup, byte/decompression read support, typed `try_get<T>()`, typed id enumeration including masked and special-routed cell DBObjs, and allocator selection for both read-only and read-write access |
| `DatReaderWriter/DatCollection.cs` | `src/DatCollection.rs` | Partial | Typed `try_get<T>()`, portal/high-res fallback, typed id enumeration, and portal-backed master-property resolution for local property-driven reads are now ported for read use |
| `DatReaderWriter/CellDatabase.cs` | `src/CellDatabase.rs` | Verified | Read-first concrete wrapper with header validation and typed read delegation |
| `DatReaderWriter/PortalDatabase.cs` | `src/PortalDatabase.rs` | Verified | Read-first concrete wrapper with header validation and typed read delegation |
| `DatReaderWriter/LocalDatabase.cs` | `src/LocalDatabase.rs` | Verified | Read-first concrete wrapper with header validation and typed read delegation |
| `DatReaderWriter/DBObjs/Iteration.cs` | `src/DBObjs/Iteration.rs` | Verified | First typed DB object read end to end |
| `DatReaderWriter/Generated/DBObjs/Palette.generated.cs` | `src/DBObjs/Palette.rs` | Verified | Typed asset object |
| `DatReaderWriter/Generated/DBObjs/SurfaceTexture.generated.cs` | `src/DBObjs/SurfaceTexture.rs` | Verified | Typed asset object |
| `DatReaderWriter/Generated/DBObjs/RenderSurface.generated.cs` | `src/DBObjs/RenderSurface.rs` | Verified | Typed asset object |
| `DatReaderWriter/Generated/DBObjs/MotionTable.generated.cs` | `src/DBObjs/MotionTable.rs` | Verified | Typed asset object |
| `DatReaderWriter/Generated/DBObjs/Setup.generated.cs` | `src/DBObjs/Setup.rs` | Verified | Setup graph root now reads explicit part hierarchy, placements, collision primitives, lights, and default asset refs |
| `DatReaderWriter/Generated/DBObjs/Animation.generated.cs` | `src/DBObjs/Animation.rs` | Verified | Animation root now reads positional frames and per-part animation frames with hooks |
| `DatReaderWriter/Generated/DBObjs/Region.generated.cs` | `src/DBObjs/Region.rs` | Verified | Region now reads sound, scene, sky, terrain, and misc sections with nested typed coverage |
| `DatReaderWriter/Generated/DBObjs/Scene.generated.cs` | `src/DBObjs/Scene.rs` | Verified | Scene object list now reads as typed DBObj |
| `DatReaderWriter/Generated/DBObjs/Surface.generated.cs` | `src/DBObjs/Surface.rs` | Verified | Surface/material DBObj ported for mesh references |
| `DatReaderWriter/Generated/DBObjs/GfxObj.generated.cs` | `src/DBObjs/GfxObj.rs` | Verified | Mesh, surface, physics BSP, drawing BSP, and degrade-id read path covered by Rust asset tests |
| `DatReaderWriter/Generated/DBObjs/Wave.generated.cs` | `src/DBObjs/Wave.rs` | Verified | Audio sample container ported |
| DatReaderWriter/Generated/DBObjs/CharGen.generated.cs | src/DBObjs/CharGen.rs | Verified | Character-generation root now reads starting areas and heritage-group hash tables |
| DatReaderWriter/Generated/DBObjs/PalSet.generated.cs | src/DBObjs/PalSet.rs | Ported | Typed palette-set DBObj ported for CharGen references |
| DatReaderWriter/Generated/DBObjs/PaletteSet.generated.cs | src/DBObjs/PaletteSet.rs | Verified | Explicit generated `PaletteSet` surface now mirrors the raw `u32` palette-id list layout and is covered by focused and retail-DAT validation |
| DatReaderWriter/Generated/DBObjs/ClothingTable.generated.cs | src/DBObjs/ClothingTable.rs | Verified | Full clothing base-effect and sub-palette tables now read as typed hash maps and are covered by Rust asset tests |
| DatReaderWriter/Generated/DBObjs/Clothing.generated.cs | src/DBObjs/Clothing.rs | Verified | Explicit generated `Clothing` surface now mirrors the raw `u32`-keyed dictionary layout and is covered by focused and retail-DAT validation |
| DatReaderWriter/Generated/DBObjs/CombatTable.generated.cs | src/DBObjs/CombatTable.rs | Verified | Full combat maneuver list now reads as typed data and is covered by Rust asset tests |
| DatReaderWriter/Generated/DBObjs/StringTable.generated.cs | src/DBObjs/StringTable.rs | Verified | Local string-table DBObj now reads language plus hashed string entries and is covered by typed DBObj tests |
| DatReaderWriter/Generated/DBObjs/LanguageString.generated.cs | src/DBObjs/LanguageString.rs | Verified | Portal-language string DBObj now reads packed byte strings and is covered by typed DBObj tests |
| DatReaderWriter/Generated/DBObjs/ParticleEmitter.generated.cs | src/DBObjs/ParticleEmitter.rs | Ported | Core particle emitter data ported for script references |
| DatReaderWriter/Generated/DBObjs/ParticleEmitterInfo.generated.cs | src/DBObjs/ParticleEmitterInfo.rs | Verified | Explicit generated `ParticleEmitterInfo` surface now mirrors the emitter payload and is covered by focused and retail-DAT validation |
| `DatReaderWriter/DBObjs/LandBlock.cs` | `src/DBObjs/LandBlock.rs` | Verified | Cell land-block DBObj now reads packed terrain and height grids and is covered by focused tests plus retail DAT validation |
| `DatReaderWriter/Generated/DBObjs/PhysicsScript.generated.cs` | `src/DBObjs/PhysicsScript.rs` | Verified | Physics script list + hook decoding ported |
| `DatReaderWriter/Generated/DBObjs/SoundTable.generated.cs` | `src/DBObjs/SoundTable.rs` | Verified | Explicit read-side sound table port with hash and named sound maps |
| `DatReaderWriter/Generated/DBObjs/PhysicsScriptTable.generated.cs` | `src/DBObjs/PhysicsScriptTable.rs` | Verified | Explicit play-script to script-list table now ported |
| `DatReaderWriter.Tests/IO/DatBinReadWriteSelfTests.cs` | `tests/dat_bin_read_write_self_tests.rs` | Verified | Initial Rust equivalents passing |
| `DatReaderWriter.Tests/IO/DatBTree/DatBTreeReaderWriterTests.cs` | `tests/btree_tests.rs` | Partial | Mock-allocator coverage now includes read traversal plus write-side insert, replace, root-split, leaf delete, and root-collapse behavior |
| `DatReaderWriter.Tests/*` options-adjacent behavior | `tests/options_tests.rs` | Verified | Rust-specific coverage for options defaults and overrides |
| `DatReaderWriter` database/collection constructor behavior | `tests/collection_tests.rs` | Verified | Synthetic header-backed tests for wrapper validation, path resolution, high-res fallback, typed id enumeration, and qualified-id resolution |
| typed DB object read behavior | `tests/typed_dbobj_tests.rs` | Verified | `Iteration` id resolution, typed read, typed id enumeration, attribute-cache coverage, and portal/local string DBObj coverage |
| typed asset object read behavior | `tests/typed_asset_tests.rs` | Verified | Palette / SurfaceTexture / RenderSurface / MotionTable / Setup / Animation / SoundTable / PhysicsScriptTable / ObjDesc / CharGen / ClothingTable / CombatTable / Region / Scene / Surface / GfxObj / Wave / ParticleEmitter / PhysicsScript plus generated enum surface coverage |

## What Works Now

- Open the crate and compile successfully.
- Read and parse DAT headers.
- Use a memory-mapped allocator for read-only byte access.
- Traverse the B-tree in read mode.
- Resolve raw file entries by id.
- Read raw file bytes and auto-decompress compressed entries.
- Mutate the in-memory/viewed B-tree structure through explicit insert/update/delete operations when backed by a writable allocator contract.
- Use a concrete stream-backed allocator for synchronous DAT header initialization, version updates, free-block reservation, and chained block read/write operations on the write path.
- Configure database and collection options from Rust.
- Open the standard four DATs through `DatCollection`.
- Validate specialized database wrappers against the header's DAT type.
- Resolve typed objects through `DatCollection` with portal-to-high-res fallback.
- Enumerate ids by DB object type through both `DatDatabase` and `DatCollection`, including masked and special-routed cell DBObjs like `Environment`, `LandBlockInfo`, `LandBlock`, and `EnvCell`.
- Resolve `QualifiedDataId<T>` references through `DatCollection`.
- Resolve packed qualified ids for known-mask DBObj references.
- Resolve and read typed `Iteration`, `Palette`, `SurfaceTexture`, `RenderSurface`, `MotionTable`, `Region`, `Scene`, and `Surface` objects.
- Read the core `GfxObj` mesh layout including surfaces, vertex arrays, polygons, and drawing/physics BSP structures.
- Read `Wave`, `ParticleEmitter`, and `PhysicsScript` objects with typed animation hook decoding.
- Read `SoundTable` and `PhysicsScriptTable` objects with explicit typed map payloads.
- Read `Setup` and `Animation` objects with explicit part/frame structures and default asset references.
- Read `CharGen` objects with explicit heritage-group, gender, starting-area, and appearance payloads.
- Read nested `Region` sound, scene, sky, terrain, and misc payloads through typed helper structs.

## Latest Progress

- Live DAT validation against your local install remains intentionally external to the crate and is not hardcoded into the Rust port or its test suite.
- Finished the main read-side `Region` dependency tree needed to move beyond the earlier raw remainder fallback.
- Added Rust ports for `SoundDesc`, `SceneDesc`, `TerrainDesc`, `RegionMisc`, `SkyDesc`, and their immediate generated child types.
- Added read-first enum wrappers for `Sound` and `TerrainTextureType` so the typed region path can compile and deserialize now.
- Added minimal marker DBObj types for `Scene`, `GfxObj`, and `PhysicsScript` so typed data-id references can be preserved without pulling in those full DBObjs yet.
- Expanded `typed_asset_tests.rs` to verify a nested `Region` roundtrip covering sound, scene, terrain, and misc blocks.
- Added game-client-facing typed read helpers on `DatDatabase`, the specialized database wrappers, and `DatCollection`.
- Added typed id enumeration and verified portal/high-res asset fallback with synthetic DAT-backed tests.
- Added `QualifiedDataId<T>` collection resolution helpers and verified them against synthetic portal/high-res data.
- Replaced the `Scene` marker with a real typed DBObj port and added the `Surface` DBObj plus the core `GfxObj` geometry/BSP read structures.
- Expanded asset tests to cover `Scene`, `Surface`, and `GfxObj` roundtrips on the read path.
- Ported the `AnimationHook` family into a read-side Rust enum, added `PackedQualifiedDataId`, and wired `PhysicsScriptData` / `PhysicsScript` on top of it.
- Added real `Wave` and `ParticleEmitter` DBObjs so immediate hook references now resolve into typed data.
- Expanded asset tests again to cover Wave, ParticleEmitter, and mixed-hook PhysicsScript roundtrips.
- Ported the next explicit asset-graph slice around those hooks: `SoundTable`, `PhysicsScriptTable`, `Setup`, and `Animation`.
- Added the supporting read-side enums and payload types for setup placements, sound tables, physics script tables, and animation frames without hardcoding any external DAT paths.
- Replaced the thin read-first Sound and PlayScript wrappers with explicit named constant surfaces mirrored from the reference enums.
- Ported ObjDesc, SubPalette, and TextureMapChange as the next explicit setup/animation-adjacent payload slice.
- Expanded `typed_asset_tests.rs` to cover `SoundTable`, `PhysicsScriptTable`, `Animation`, `Setup`, `ObjDesc`, and `CharGen` roundtrips.
- Finished the immediate CharGen dependency chain for ClothingTable and CombatTable, including ClothingBaseEffect, CloObjectEffect, CloTextureEffect, CloSubPalEffect, CloSubPalette, CloSubPaletteRange, and CombatManeuver.
- Extended the explicit HashTable port to support QualifiedDataId<T> keys, which unblocks the setup-keyed clothing table read path.
- Expanded `typed_asset_tests.rs` again to verify nested `ClothingTable` and `CombatTable` roundtrips on the read path.
- Finished the remaining read-side GfxObj support gap by aligning the drawing/physics BSP node behavior with the reference and filling out the full named BSPNodeType surface.
- Expanded `typed_asset_tests.rs` to cover drawing portal BSP nodes, physics portal rejection, and `GfxObj` roundtrips with physics, drawing, and degrade data.
- Finished the remaining partial named enum surfaces for `SkillId`, `TerrainTextureType`, `ParentLocation`, `Placement`, `EmitterType`, `ParticleType`, `VertexType`, and `CullMode`, and verified the new constants inside the existing asset test binary.
- Ported the explicit B-tree node mutation helpers and synchronous write-side insert/update/delete flow, including empty-tree root creation, in-place replacement, full-root splitting, leaf deletion, and empty-root collapse through a writable allocator contract.
- Replaced the stream allocator placeholder with a concrete file-backed synchronous implementation and verified it with temp-file tests for header/version writes, chained block IO, and in-place block rewrites.
- Reworked DBObjAttributeCache so current Rust DBObj ports resolve through a shared attribute list instead of a large hand-maintained Portal match, and added typed tests for singular/range resolution.
- Added the string-resource read path with PStringBase, StringTableString, StringTableData, LanguageString, and StringTable, including local DAT typed reads for hashed string entries.
- Added the explicit enum-mapper family support with concrete `AutoGrowHashTable` / explicit map containers, `EnumMapperData`, `EnumMapper`, `EnumIDMap`, and `DualEnumIDMap`, and corrected the retail-matching `ReadString16LByte`/`WriteString16LByte` contract to use compressed-length single-byte strings instead of the earlier guessed UTF-16 path.
- Added the remaining generated DBObj file surfaces `PaletteSet`, `Clothing`, and `ParticleEmitterInfo`, and verified them against both focused roundtrip tests and the external retail DAT validation path.
- Added the explicit material/render asset slice with `RenderTexture`, `RenderMaterial`, `MaterialModifier`, `MaterialInstance`, `MaterialProperty`, and their immediate enum dependencies.
- Added the explicit degrade/filter/spell-component slice with `GfxObjDegradeInfo`, `QualityFilter`, `SpellComponentTable`, `GfxObjInfo`, `SpellComponentBase`, `ObfuscatedPStringBase`, and `ComponentType`.
- Added the explicit bad-data/contract/taboo slice with `BadDataTable`, `ContractTable`, `TabooTable`, `Contract`, `TabooTableEntry`, and the `AC1LegacyPStringBase` compatibility layer.
- Added the explicit `ChatPoseTable` slice with `ChatEmoteData` and legacy packed-string keyed table handling kept local to the DBObj instead of broadening the generic container surface.
- Added the explicit `ObjectHierarchy` slice with recursive `ObjHierarchyNode` support on top of the already-ported obfuscated string format.
- Added the explicit `MasterInputMap` slice with `DeviceType`, `DeviceKeyMapEntry`, `ControlSpecification`, `QualifiedControl`, and `CInputMap`.
- Re-ran external retail DAT validation against `C:\Turbine\Asheron's call\` after the new material slice, and the ignored real-DAT suite still passes without hardcoding that path into the crate.
- Re-ran external retail DAT validation against `C:\Turbine\Asheron's call\` after the new degrade/filter/spell-component slice, and the ignored real-DAT suite still passes without hardcoding that path into the crate.
- Re-ran external retail DAT validation against `C:\Turbine\Asheron's call\` after the new bad-data/contract/taboo slice, and the ignored real-DAT suite still passes without hardcoding that path into the crate.
- Re-ran external retail DAT validation against `C:\Turbine\Asheron's call\` after the new `ChatPoseTable` slice, and the ignored real-DAT suite still passes without hardcoding that path into the crate.
- Re-ran external retail DAT validation against `C:\Turbine\Asheron's call\` after the new `ObjectHierarchy` slice, and the ignored real-DAT suite still passes without hardcoding that path into the crate.
- Re-ran external retail DAT validation against `C:\Turbine\Asheron's call\` after the new `MasterInputMap` slice, and the ignored real-DAT suite still passes without hardcoding that path into the crate.

## Remaining Major Areas

- Remaining write-path parity, including async allocator methods and any additional database-level authoring helpers still needed
- DB object attribute cache and id-to-type resolution beyond the currently ported Rust DBObj set, including broader Local/Cell coverage
- Generated DBObjs beyond the current asset-focused subset
- Generated Types beyond the current asset-focused subset
- Generated database readers
- Richer cache behavior and higher-level object graph traversal beyond direct `QualifiedDataId<T>` lookups
- Write-path allocator and B-tree behavior if parity is still desired later
- Broader test suite migration with real DAT-backed fixtures that remain external to the crate























## Restored From Snapshot

- Restored from `PORTING_STATUS_snapshot_298f8bd.md`, sourced from commit `298f8bdad60cdc4842583958161e853d2537c749` in the `CrimsonMage/DATRusterWriter` repository.
- The original local `PORTING_STATUS.md` became unreadable due to null-padded contents and was rebuilt from this snapshot plus subsequent verified port work.
- The temporary `PORTING_STATUS_ADDENDUM.md` recovery notes have been merged back into this main tracker, and the addendum is no longer needed.

## Progress Since Snapshot Recovery

| Source | Rust | Status | Notes |
|---|---|---|---|
| `DatReaderWriter/Generated/Enums/AttributeId.generated.cs` | `src/Generated/Enums/AttributeId.rs` | Ported | Attribute ids required for stat table support |
| `DatReaderWriter/Generated/Enums/SkillCategory.generated.cs` | `src/Generated/Enums/SkillCategory.rs` | Ported | Skill categories required for `SkillTable` |
| `DatReaderWriter/Generated/DBObjs/VitalTable.generated.cs` | `src/DBObjs/VitalTable.rs` | Verified | Typed vital formulas now read and are covered by focused tests and retail DAT validation |
| `DatReaderWriter/Generated/DBObjs/SkillTable.generated.cs` | `src/DBObjs/SkillTable.rs` | Verified | Typed skill entries now read and are covered by focused tests and retail DAT validation |
| `DatReaderWriter/Generated/DBObjs/ExperienceTable.generated.cs` | `src/DBObjs/ExperienceTable.rs` | Verified | Typed progression arrays now read and are covered by focused tests and retail DAT validation |
| `DatReaderWriter/Generated/Types/SkillBase.generated.cs` | `src/Types/SkillBase.rs` | Ported | Supporting typed skill payload |
| `DatReaderWriter/Generated/Types/SkillFormula.generated.cs` | `src/Types/SkillFormula.rs` | Ported | Supporting typed formula payload |
| `DatReaderWriter/Generated/DBObjs/Font.generated.cs` | `src/DBObjs/Font.rs` | Verified | Glyph metrics and typed surface references are now ported and validated against retail DATs |
| `DatReaderWriter/Generated/DBObjs/LanguageInfo.generated.cs` | `src/DBObjs/LanguageInfo.rs` | Verified | Local language settings DBObj is now ported and validated against retail DATs |
| `DatReaderWriter/Generated/DBObjs/NameFilterTable.generated.cs` | `src/DBObjs/NameFilterTable.rs` | Verified | Name-filter rule table is now ported and validated against retail DATs |
| `DatReaderWriter/Generated/Types/FontCharDesc.generated.cs` | `src/Types/FontCharDesc.rs` | Ported | Supporting glyph description payload |
| `DatReaderWriter/Generated/Types/NameFilterLanguageData.generated.cs` | `src/Types/NameFilterLanguageData.rs` | Ported | Supporting language-specific naming-rule payload |
| `DatReaderWriter/Generated/Types/StringInfo.generated.cs` | `src/Types/StringInfo.rs` | Verified | String-table indirection payload is ported and roundtrip tested |
| `DatReaderWriter/Generated/Types/MediaDesc.generated.cs` and `DatReaderWriter/Generated/Types/MediaDesc*.generated.cs` | `src/Types/MediaDesc.rs`, `src/Types/MediaDesc*.rs` | Verified | Discriminated media descriptor family is ported and roundtrip tested for current client-facing variants |
| `DatReaderWriter/Generated/Enums/DrawModeType.generated.cs` | `src/Generated/Enums/DrawModeType.rs` | Ported | Media descriptor draw-mode enum |
| `DatReaderWriter/Generated/Enums/MediaType.generated.cs` | `src/Generated/Enums/MediaType.rs` | Ported | Media descriptor discriminant enum |
| `DatReaderWriter/Generated/Enums/StringInfoOverrideFlag.generated.cs` | `src/Generated/Enums/StringInfoOverrideFlag.rs` | Ported | String-info override flags |
| `DatReaderWriter/Generated/Enums/UIStateId.generated.cs` | `src/Generated/Enums/UIStateId.rs` | Verified | Full named UI state constant surface now mirrors the generated reference and is exercised in the enum-surface test |
| `DatReaderWriter/Generated/Enums/BasePropertyType.generated.cs` | `src/Generated/Enums/BasePropertyType.rs` | Ported | Base-property discriminant enum for the local property pipeline |
| `DatReaderWriter/Generated/Enums/VitalId.generated.cs` | `src/Generated/Enums/VitalId.rs` | Verified | Full named vital-id constant surface is now mirrored and exercised in the enum-surface test |
| `DatReaderWriter/Generated/Enums/PatchFlags.generated.cs` | `src/Generated/Enums/PatchFlags.rs` | Ported | Property descriptor patch-flag enum |
| `DatReaderWriter/Generated/Enums/PropertyCachingType.generated.cs` | `src/Generated/Enums/PropertyCachingType.rs` | Ported | Property descriptor caching enum |
| `DatReaderWriter/Generated/Enums/PropertyDatFileType.generated.cs` | `src/Generated/Enums/PropertyDatFileType.rs` | Ported | Property descriptor dat-file enum |
| `DatReaderWriter/Generated/Enums/PropertyGroupName.generated.cs` | `src/Generated/Enums/PropertyGroupName.rs` | Ported | Property descriptor grouping enum with the current named surface mirrored |
| `DatReaderWriter/Generated/Enums/PropertyInheritanceType.generated.cs` | `src/Generated/Enums/PropertyInheritanceType.rs` | Ported | Property descriptor inheritance enum |
| `DatReaderWriter/Generated/Enums/PropertyPropagationType.generated.cs` | `src/Generated/Enums/PropertyPropagationType.rs` | Ported | Property descriptor propagation enum |
| external retail DAT validation | `tests/real_dat_validation_tests.rs` | Verified | Live validation continues to pass against retail DATs in `C:\Turbine\Asheron's call\` via `DAT_READER_WRITER_REAL_DAT_DIR`, without hardcoding the path into the crate |
| tracker recovery | `PORTING_STATUS.md` | Verified | Main tracker restored from the last good snapshot and updated with post-snapshot work |
| `DatReaderWriter/Types/BaseProperty.cs` | `src/Types/BaseProperty.rs` | Partial | `MasterProperty`-driven generic unpack is now ported for current read paths, including array/struct and several raw property variants, but the full long-tail property surface still remains |
| `DatReaderWriter/Generated/Types/BoolBaseProperty.generated.cs` | `src/Types/BoolBaseProperty.rs` | Verified | Explicit scalar bool property wrapper now mirrors the generated file shape and is covered by focused roundtrip tests |
| `DatReaderWriter/Generated/Types/IntegerBaseProperty.generated.cs` | `src/Types/IntegerBaseProperty.rs` | Verified | Explicit scalar integer property wrapper now mirrors the generated file shape and is covered by focused roundtrip tests |
| `DatReaderWriter/Generated/Types/FloatBaseProperty.generated.cs` | `src/Types/FloatBaseProperty.rs` | Verified | Explicit scalar float property wrapper now mirrors the generated file shape and is covered by focused roundtrip tests |
| `DatReaderWriter/Generated/Types/VectorBaseProperty.generated.cs` | `src/Types/VectorBaseProperty.rs` | Verified | Explicit vector property wrapper now mirrors the generated file shape and is covered by focused roundtrip tests |
| `DatReaderWriter/Generated/Types/ColorBaseProperty.generated.cs` | `src/Types/ColorBaseProperty.rs` | Verified | Explicit color property wrapper now mirrors the generated file shape and is covered by focused roundtrip tests |
| `DatReaderWriter/Generated/Types/EnumBaseProperty.generated.cs` | `src/Types/EnumBaseProperty.rs` | Verified | Explicit enum property wrapper now mirrors the generated file shape and is covered by focused roundtrip tests |
| `DatReaderWriter/Generated/Types/DataIdBaseProperty.generated.cs` | `src/Types/DataIdBaseProperty.rs` | Verified | Explicit data-id property wrapper now mirrors the generated file shape and is covered by focused roundtrip tests |
| `DatReaderWriter/Generated/Types/InstanceIdBaseProperty.generated.cs` | `src/Types/InstanceIdBaseProperty.rs` | Verified | Explicit instance-id property wrapper now mirrors the generated file shape and is covered by focused roundtrip tests |
| `DatReaderWriter/Generated/Types/Bitfield32BaseProperty.generated.cs` | `src/Types/Bitfield32BaseProperty.rs` | Verified | Explicit 32-bit bitfield property wrapper now mirrors the generated file shape and is covered by focused roundtrip tests |
| `DatReaderWriter/Generated/Types/Bitfield64BaseProperty.generated.cs` | `src/Types/Bitfield64BaseProperty.rs` | Verified | Explicit 64-bit bitfield property wrapper now mirrors the generated file shape and is covered by focused roundtrip tests |
| `DatReaderWriter/Generated/Types/StringInfoBaseProperty.generated.cs` | `src/Types/StringInfoBaseProperty.rs` | Partial | Explicit `StringInfo` property wrapper is ported on top of the new base-property foundation |
| `DatReaderWriter/Types/BasePropertyDesc.cs` | `src/Types/BasePropertyDesc.rs` | Partial | Property descriptor records now unpack/pack current scalar-bound/default metadata, but broader `MasterProperty` consumers are still pending |
| `DatReaderWriter/Generated/Enums/RMDataType.generated.cs` | `src/Generated/Enums/RMDataType.rs` | Ported | Explicit render-material data-type enum mirrored from the reference |
| `DatReaderWriter/Generated/Enums/RenderPassType.generated.cs` | `src/Generated/Enums/RenderPassType.rs` | Verified | Full named render-pass constant surface now mirrors the generated reference and is exercised in the enum-surface test |
| `DatReaderWriter/Generated/Types/MaterialProperty.generated.cs` | `src/Types/MaterialProperty.rs` | Verified | Explicit material-property payload with aligned length fields now roundtrips in focused tests |
| `DatReaderWriter/Generated/DBObjs/RenderTexture.generated.cs` | `src/DBObjs/RenderTexture.rs` | Verified | Explicit render-texture DBObj now reads texture type and render-surface level refs and is validated against retail DATs |
| `DatReaderWriter/Generated/DBObjs/RenderMaterial.generated.cs` | `src/DBObjs/RenderMaterial.rs` | Verified | Explicit empty-body render-material DBObj now resolves and is validated against retail DATs |
| `DatReaderWriter/Generated/DBObjs/MaterialModifier.generated.cs` | `src/DBObjs/MaterialModifier.rs` | Verified | Explicit material-modifier DBObj now reads typed material properties and is validated against retail DATs |
| `DatReaderWriter/Generated/DBObjs/MaterialInstance.generated.cs` | `src/DBObjs/MaterialInstance.rs` | Verified | Explicit material-instance DBObj now reads material id/type, modifier refs, and booleans and is validated against retail DATs |
| `DatReaderWriter/Generated/Enums/ComponentType.generated.cs` | `src/Generated/Enums/ComponentType.rs` | Ported | Explicit spell-component enum mirrored from the reference |
| `DatReaderWriter/Types/ObfuscatedPStringBase.cs` | `src/Types/ObfuscatedPStringBase.rs` | Verified | Explicit obfuscated Windows-1252 string format now decodes/encodes with nibble rotation and alignment |
| `DatReaderWriter/Generated/Types/GfxObjInfo.generated.cs` | `src/Types/GfxObjInfo.rs` | Verified | Explicit degrade-entry payload with `QualifiedDataId<GfxObj>` and distance thresholds now roundtrips in focused tests |
| `DatReaderWriter/Generated/Types/SpellComponentBase.generated.cs` | `src/Types/SpellComponentBase.rs` | Verified | Explicit spell-component payload now reads obfuscated strings, icon ids, enums, and timing data and is covered by focused tests |
| `DatReaderWriter/Generated/DBObjs/GfxObjDegradeInfo.generated.cs` | `src/DBObjs/GfxObjDegradeInfo.rs` | Verified | Explicit degrade-info DBObj now reads typed degrade entries and is validated against retail DATs |
| `DatReaderWriter/Generated/DBObjs/QualityFilter.generated.cs` | `src/DBObjs/QualityFilter.rs` | Verified | Explicit quality-filter DBObj now reads all stat-filter arrays and is validated against retail DATs |
| `DatReaderWriter/Generated/DBObjs/SpellComponentTable.generated.cs` | `src/DBObjs/SpellComponentTable.rs` | Verified | Explicit spell-component table DBObj now reads typed packable component entries and is validated against retail DATs |
| `DatReaderWriter/Types/AC1LegacyPStringBase.cs` | `src/Types/AC1LegacyPStringBase.rs` | Verified | Explicit AC1 legacy packed string compatibility wrapper now mirrors the current byte-string read/write format used by contract records |
| `DatReaderWriter/Generated/Types/Contract.generated.cs` | `src/Types/Contract.rs` | Verified | Explicit contract payload now reads quest strings and positions and is covered by focused tests |
| `DatReaderWriter/Generated/Types/TabooTableEntry.generated.cs` | `src/Types/TabooTableEntry.rs` | Verified | Explicit taboo-entry payload now reads the key, flag, and banned pattern list and is covered by focused tests |
| `DatReaderWriter/Generated/DBObjs/BadDataTable.generated.cs` | `src/DBObjs/BadDataTable.rs` | Verified | Explicit bad-data table DBObj now reads its qualified-id to raw-value map and is validated against retail DATs |
| `DatReaderWriter/Generated/Types/ChatEmoteData.generated.cs` | `src/Types/ChatEmoteData.rs` | Verified | Explicit chat-emote payload now reads both legacy packed emote strings and is covered by focused tests |
| `DatReaderWriter/Generated/DBObjs/ChatPoseTable.generated.cs` | `src/DBObjs/ChatPoseTable.rs` | Verified | Explicit chat-pose table DBObj now reads legacy-string keyed pose and emote maps and is validated against retail DATs |
| `DatReaderWriter/Generated/DBObjs/ContractTable.generated.cs` | `src/DBObjs/ContractTable.rs` | Verified | Explicit contract table DBObj now reads typed contract entries and is validated against retail DATs |
| `DatReaderWriter/Generated/Types/ObjHierarchyNode.generated.cs` | `src/Types/ObjHierarchyNode.rs` | Verified | Explicit recursive hierarchy-node payload now reads menu text, WCID, and child nodes and is covered by focused tests |
| `DatReaderWriter/Generated/DBObjs/ObjectHierarchy.generated.cs` | `src/DBObjs/ObjectHierarchy.rs` | Verified | Explicit object-hierarchy DBObj now reads the recursive root node and is validated against retail DATs |
| `DatReaderWriter/Generated/Enums/DeviceType.generated.cs` | `src/Generated/Enums/DeviceType.rs` | Ported | Explicit input-device enum mirrored from the reference |
| `DatReaderWriter/Generated/Types/ControlSpecification.generated.cs` | `src/Types/ControlSpecification.rs` | Verified | Explicit key/modifier pair payload now roundtrips in focused tests |
| `DatReaderWriter/Generated/Types/DeviceKeyMapEntry.generated.cs` | `src/Types/DeviceKeyMapEntry.rs` | Verified | Explicit device-keymap entry now reads device type plus GUID and is covered by focused tests |
| `DatReaderWriter/Generated/Types/QualifiedControl.generated.cs` | `src/Types/QualifiedControl.rs` | Verified | Explicit input binding payload now reads control spec plus activation fields and is covered by focused tests |
| `DatReaderWriter/Generated/Types/CInputMap.generated.cs` | `src/Types/CInputMap.rs` | Verified | Explicit input-map payload now reads qualified-control lists and is covered by focused tests |
| `DatReaderWriter/Generated/DBObjs/MasterInputMap.generated.cs` | `src/DBObjs/MasterInputMap.rs` | Verified | Explicit master-input-map DBObj now reads name, GUID, devices, meta keys, and keyed input maps and is validated against retail DATs |
| `DatReaderWriter/Generated/DBObjs/TabooTable.generated.cs` | `src/DBObjs/TabooTable.rs` | Verified | Explicit taboo table DBObj now reads audience-keyed banned-pattern groups and is validated against retail DATs |
| `DatReaderWriter/Generated/Enums/EquipmentSet.generated.cs` | `src/Generated/Enums/EquipmentSet.rs` | Ported | Explicit equipment-set enum mirrored from the reference for `SpellTable` spell-set keys |
| `DatReaderWriter/Generated/Enums/ItemType.generated.cs` | `src/Generated/Enums/ItemType.rs` | Ported | Explicit item-type enum mirrored from the reference for spell targeting metadata |
| `DatReaderWriter/Generated/Enums/MagicSchool.generated.cs` | `src/Generated/Enums/MagicSchool.rs` | Ported | Explicit magic-school enum mirrored from the reference for spell records |
| `DatReaderWriter/Generated/Enums/SpellCategory.generated.cs` | `src/Generated/Enums/SpellCategory.rs` | Verified | Full named spell-category constant surface now mirrors the generated reference and is exercised in the enum-surface test |
| `DatReaderWriter/Generated/Enums/SpellIndex.generated.cs` | `src/Generated/Enums/SpellIndex.rs` | Ported | Explicit spell-index bitflags mirrored from the reference |
| `DatReaderWriter/Generated/Enums/SpellType.generated.cs` | `src/Generated/Enums/SpellType.rs` | Ported | Explicit meta-spell type enum mirrored from the reference |
| `DatReaderWriter/Types/PHashTable.cs` | `src/Types/PHashTable.rs` | Verified | Explicit packable hash-table variant now mirrors count/bucket storage without re-sorting entries and is covered by focused spell-table tests |
| `DatReaderWriter/Generated/Types/SpellSetTiers.generated.cs` | `src/Types/SpellSetTiers.rs` | Verified | Explicit spell-set tier payload now roundtrips tier spell lists in focused tests |
| `DatReaderWriter/Generated/Types/SpellSet.generated.cs` | `src/Types/SpellSet.rs` | Verified | Explicit spell-set payload now reads nested tier tables and is covered by focused spell-table tests |
| `DatReaderWriter/Types/SpellBase.cs` | `src/Types/SpellBase.rs` | Verified | Explicit spell payload now mirrors name/description hashing, encrypted component decoding, meta-spell variant fields, and roundtrips in focused tests |
| `DatReaderWriter/Generated/DBObjs/SpellTable.generated.cs` | `src/DBObjs/SpellTable.rs` | Verified | Explicit spell-table DBObj now reads typed spell records plus equipment-set spell groups and is validated against retail DATs |
| `DatReaderWriter/Enums/ToggleType.cs` | `src/Generated/Enums/ToggleType.rs` | Ported | Explicit action-map toggle enum mirrored from the reference |
| `DatReaderWriter/Types/UserBindingData.cs` | `src/Types/UserBindingData.rs` | Verified | Explicit action-map user-binding payload now roundtrips class/name/description ids in focused tests |
| `DatReaderWriter/Types/InputConflictsValue.cs` | `src/Types/InputsConflictsValue.rs` | Verified | Explicit conflicting-input-map payload now roundtrips conflict lists in focused tests |
| `DatReaderWriter/Types/ActionMapValue.cs` | `src/Types/ActionMapValue.rs` | Verified | Explicit action-map value payload now reads toggle mode plus user-binding data and is covered by focused tests |
| `DatReaderWriter/DBObjs/ActionMap.cs` | `src/DBObjs/ActionMap.rs` | Verified | Explicit action-map DBObj now reads nested input maps and conflict tables and is validated against retail DATs |
| `DatReaderWriter/DBObjs/MasterProperty.cs` | `src/DBObjs/MasterProperty.rs` | Verified | Explicit master-property DBObj now reads enum mapper data plus typed property descriptors and is validated against retail DATs |
| `DatReaderWriter/Generated/Enums/EnvCellFlags.generated.cs` | `src/Generated/Enums/EnvCellFlags.rs` | Ported | Explicit cell flag bitflags mirrored for `EnvCell` reads |
| `DatReaderWriter/Generated/Enums/PortalFlags.generated.cs` | `src/Generated/Enums/PortalFlags.rs` | Ported | Explicit cell and building portal flags mirrored for cell-side reads |
| `DatReaderWriter/Types/TerrainInfo.cs` | `src/Types/TerrainInfo.rs` | Verified | Explicit packed terrain-info bitfield now mirrors road, terrain, and scenery storage and is covered by focused tests |
| `DatReaderWriter/Generated/Types/Stab.generated.cs` | `src/Types/Stab.rs` | Verified | Explicit static-object placement payload is ported and exercised through cell-focused tests |
| `DatReaderWriter/Generated/Types/BuildingPortal.generated.cs` | `src/Types/BuildingPortal.rs` | Verified | Explicit building-portal payload is ported and exercised through cell-focused tests |
| `DatReaderWriter/Generated/Types/BuildingInfo.generated.cs` | `src/Types/BuildingInfo.rs` | Verified | Explicit building payload is ported and exercised through cell-focused tests |
| `DatReaderWriter/Generated/Types/CellPortal.generated.cs` | `src/Types/CellPortal.rs` | Verified | Explicit cell-portal payload is ported and exercised through cell-focused tests |
| `DatReaderWriter/Generated/Types/CellStruct.generated.cs` | `src/Types/CellStruct.rs` | Verified | Explicit environment cell-structure payload with portals and BSP refs is ported and exercised through cell-focused tests |
| `DatReaderWriter/Generated/DBObjs/Environment.generated.cs` | `src/DBObjs/Environment.rs` | Verified | Explicit environment DBObj now reads keyed cell structures and is covered by focused tests plus retail DAT validation |
| `DatReaderWriter/Generated/DBObjs/LandBlockInfo.generated.cs` | `src/DBObjs/LandBlockInfo.rs` | Verified | Explicit land-block-info DBObj now reads object, building, and restriction tables and is covered by focused tests plus retail DAT validation |
| `DatReaderWriter/Generated/DBObjs/EnvCell.generated.cs` | `src/DBObjs/EnvCell.rs` | Verified | Explicit environment-cell DBObj now reads surfaces, portals, visibility, static objects, and restriction refs and is covered by focused tests plus retail DAT validation |
| `DatReaderWriter/Generated/Enums/IncorporationFlags.generated.cs` | `src/Generated/Enums/IncorporationFlags.rs` | Ported | Explicit UI incorporation-flag bitfield mirrored for layout element/state reads |
| `DatReaderWriter/Types/StateDesc.cs` | `src/Types/StateDesc.rs` | Verified | Explicit layout state payload now reads base properties and media lists through master-property resolution and is covered by focused tests |
| `DatReaderWriter/Generated/Types/ElementDesc.generated.cs` | `src/Types/ElementDesc.rs` | Verified | Explicit recursive UI element payload now reads incorporation-gated geometry, state maps, and child elements and is covered by focused tests |
| `DatReaderWriter/Generated/DBObjs/LayoutDesc.generated.cs` | `src/DBObjs/LayoutDesc.rs` | Verified | Explicit local layout DBObj now reads width, height, and hashed element trees and is validated against retail DATs |
| `DatReaderWriter/DBObjs/DBProperties.cs` | `src/DBObjs/DBProperties.rs` | Verified | Explicit portal DB-properties DBObj now reads master-property-typed property maps and is validated against retail DATs |

## Updated Remaining Major Areas

- Remaining base-property variants and the broader `MasterProperty`/property-description pipeline beyond the current `DBProperties` and `LayoutDesc` read paths
- Remaining generated DBObjs outside the current asset, gameplay, CharGen, string, language, first material/render, degrade/filter/spell-component, and bad-data/contract/taboo subsets
- Remaining generated Types outside the currently ported dependency tree
- `DBObjAttributeCache` coverage beyond the currently ported Rust DBObj set, including broader Local and Cell coverage
- Generated database readers
- Any broader write-path parity not directly needed for client read support
- Broader real-DAT validation coverage as more DBObjs/types are added
