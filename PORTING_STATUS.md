# DatReaderWriter Rust Port Status

This document tracks what has been ported from `C:\Repo\NewAC_Client\vitaeum\ref\DatReaderWriter` into this crate and what remains.

## Rules

See `PORTING_RULES.md` for the tracking contract used during this port.

## Scope Note

- Primary target is now read functionality.
- Write functionality may still be scaffolded or partially retained where it helps preserve structure, tests, or future parity.
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
| `DatReaderWriter/Generated/Enums/MotionDataFlags.generated.cs` | `src/Generated/Enums/MotionDataFlags.rs` | Ported | Motion data flag bitflags |
| `DatReaderWriter/Generated/Enums/PartsMask.generated.cs` | `src/Generated/Enums/PartsMask.rs` | Ported | Region parts bitflags now used in typed Region reads |
| `DatReaderWriter/Generated/Enums/GfxObjFlags.generated.cs` | `src/Generated/Enums/GfxObjFlags.rs` | Ported | Read-side mesh flags |
| `DatReaderWriter/Generated/Enums/SurfaceType.generated.cs` | `src/Generated/Enums/SurfaceType.rs` | Ported | Surface/material bitflags |
| `DatReaderWriter/Generated/Enums/AnimationHookType.generated.cs` | `src/Generated/Enums/AnimationHookType.rs` | Ported | Hook discriminator constants |
| `DatReaderWriter/Generated/Enums/AnimationHookDir.generated.cs` | `src/Generated/Enums/AnimationHookDir.rs` | Ported | Hook direction constants |
| `DatReaderWriter/Generated/Enums/AnimationFlags.generated.cs` | `src/Generated/Enums/AnimationFlags.rs` | Ported | Read-side animation presence bitflags |
| `DatReaderWriter/Generated/Enums/ParentLocation.generated.cs` | `src/Generated/Enums/ParentLocation.rs` | Partial | Read-first numeric wrapper for setup attachment slots |
| `DatReaderWriter/Generated/Enums/Placement.generated.cs` | `src/Generated/Enums/Placement.rs` | Partial | Read-first numeric wrapper for setup placement keys |
| `DatReaderWriter/Generated/Enums/PlayScript.generated.cs` | `src/Generated/Enums/PlayScript.rs` | Ported | Explicit named play-script constants now mirrored into the Rust wrapper surface |
| `DatReaderWriter/Generated/Enums/SetupFlags.generated.cs` | `src/Generated/Enums/SetupFlags.rs` | Ported | Setup optional-data bitflags |
| DatReaderWriter/Generated/Enums/SkillId.generated.cs | src/Generated/Enums/SkillId.rs | Partial | Explicit named skill-id constants added for the CharGen read path; enum surface is not complete yet |
| `DatReaderWriter/Generated/Enums/EmitterType.generated.cs` | `src/Generated/Enums/EmitterType.rs` | Partial | Read-first numeric wrapper for particle emitter mode |
| `DatReaderWriter/Generated/Enums/ParticleType.generated.cs` | `src/Generated/Enums/ParticleType.rs` | Partial | Read-first numeric wrapper for particle motion mode |
| `DatReaderWriter/Generated/Enums/VertexType.generated.cs` | `src/Generated/Enums/VertexType.rs` | Partial | Read-first numeric wrapper for vertex array type |
| `DatReaderWriter/Generated/Enums/CullMode.generated.cs` | `src/Generated/Enums/CullMode.rs` | Partial | Read-first numeric wrapper for polygon cull mode |
| `DatReaderWriter/Generated/Enums/StipplingType.generated.cs` | `src/Generated/Enums/StipplingType.rs` | Ported | Polygon stippling bitflags |
| `DatReaderWriter/Enums/BSPNodeType.cs` | `src/Generated/Enums/BSPNodeType.rs` | Partial | Read-first numeric constants for BSP node decoding |
| `DatReaderWriter/Generated/Enums/Sound.generated.cs` | `src/Generated/Enums/Sound.rs` | Ported | Explicit named sound constants now mirrored into the Rust wrapper surface |
| `DatReaderWriter/Generated/Enums/TerrainTextureType.generated.cs` | `src/Generated/Enums/TerrainTextureType.rs` | Partial | Read-first numeric wrapper for terrain texture ids |
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
| `DatReaderWriter/Lib/IO/BlockAllocators/IDatBlockAllocator.cs` | `src/Lib/IO/BlockAllocators/IDatBlockAllocator.rs` | Partial | Read-path contract only |
| `DatReaderWriter/Lib/IO/BlockAllocators/BaseBlockAllocator.cs` | `src/Lib/IO/BlockAllocators/BaseBlockAllocator.rs` | Scaffolded | Placeholder while read logic lives in concrete allocator |
| `DatReaderWriter/Lib/IO/BlockAllocators/MemoryMappedBlockAllocator.cs` | `src/Lib/IO/BlockAllocators/MemoryMappedBlockAllocator.rs` | Partial | Read-only memory-mapped implementation |
| `DatReaderWriter/Lib/IO/BlockAllocators/StreamBlockAllocator.cs` | `src/Lib/IO/BlockAllocators/StreamBlockAllocator.rs` | Scaffolded | Deferred in favor of memory-mapped read path |
| `DatReaderWriter/Lib/IO/DatBTree/DatBTreeFileFlags.cs` | `src/Lib/IO/DatBTree/DatBTreeFileFlags.rs` | Ported | Bitflags port |
| `DatReaderWriter/Lib/IO/DatBTree/DatBTreeFile.cs` | `src/Lib/IO/DatBTree/DatBTreeFile.rs` | Ported | Entry unpack and basic pack parity |
| `DatReaderWriter/Lib/IO/DatBTree/DatBTreeNode.cs` | `src/Lib/IO/DatBTree/DatBTreeNode.rs` | Verified | Read-path node pack/unpack symmetry and traversal helpers tested |
| `DatReaderWriter/Lib/IO/DatBTree/DatBTreeReaderWriter.cs` | `src/Lib/IO/DatBTree/DatBTreeReaderWriter.rs` | Verified | Read-path lookup, enumeration, caching, and range traversal tested |
| `DatReaderWriter/Lib/Attributes/DBObjTypeAttribute.cs` | `src/Lib/Attributes/DBObjTypeAttribute.rs` | Ported | Rust metadata descriptor |
| `DatReaderWriter/Lib/DBObjAttributeCache.cs` | `src/Lib/DBObjAttributeCache.rs` | Partial | Manual id mapping for currently ported objects |
| `DatReaderWriter/Types/DBObj.cs` | `src/Types/DBObj.rs` | Partial | Rust DB object base abstraction |
| `DatReaderWriter/Types/AC1LegacyPStringBase.cs` | `src/Types/AC1LegacyString.rs` | Partial | Byte-string read path only |
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
| `DatReaderWriter/Types/BSPTree.cs`, `DatReaderWriter/Types/PhysicsBSPNode.cs`, `DatReaderWriter/Types/DrawingBSPNode.cs` | `src/Types/BSPTrees.rs` | Partial | Read-side BSP tree/node port collapsed into one Rust module |
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
| DatReaderWriter/Types/HashTable.cs | src/Types/HashTable.rs | Partial | Explicit read/write hash-table wrapper currently supports primitive keys and unpackable values used by CharGen |
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
| `DatReaderWriter/DatDatabase.cs` | `src/DatDatabase.rs` | Partial | Raw file entry lookup, byte/decompression read support, typed `try_get<T>()`, and typed id enumeration |
| `DatReaderWriter/DatCollection.cs` | `src/DatCollection.rs` | Partial | Typed `try_get<T>()`, portal/high-res fallback, and typed id enumeration now ported for read use |
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
| `DatReaderWriter/Generated/DBObjs/GfxObj.generated.cs` | `src/DBObjs/GfxObj.rs` | Partial | Core mesh/surface/vertex/polygon/BSP read path ported |
| `DatReaderWriter/Generated/DBObjs/Wave.generated.cs` | `src/DBObjs/Wave.rs` | Verified | Audio sample container ported |
| DatReaderWriter/Generated/DBObjs/CharGen.generated.cs | src/DBObjs/CharGen.rs | Verified | Character-generation root now reads starting areas and heritage-group hash tables |
| DatReaderWriter/Generated/DBObjs/PalSet.generated.cs | src/DBObjs/PalSet.rs | Ported | Palette-set DBObj ported for CharGen references |
| DatReaderWriter/Generated/DBObjs/ClothingTable.generated.cs | src/DBObjs/ClothingTable.rs | Partial | Marker DBObj added so typed CharGen references stay explicit; full clothing effects remain pending |
| DatReaderWriter/Generated/DBObjs/CombatTable.generated.cs | src/DBObjs/CombatTable.rs | Partial | Marker DBObj added so typed CharGen references stay explicit; full combat maneuver data remains pending |
| `DatReaderWriter/Generated/DBObjs/ParticleEmitter.generated.cs` | `src/DBObjs/ParticleEmitter.rs` | Ported | Core particle emitter data ported for script references |
| `DatReaderWriter/Generated/DBObjs/PhysicsScript.generated.cs` | `src/DBObjs/PhysicsScript.rs` | Verified | Physics script list + hook decoding ported |
| `DatReaderWriter/Generated/DBObjs/SoundTable.generated.cs` | `src/DBObjs/SoundTable.rs` | Verified | Explicit read-side sound table port with hash and named sound maps |
| `DatReaderWriter/Generated/DBObjs/PhysicsScriptTable.generated.cs` | `src/DBObjs/PhysicsScriptTable.rs` | Verified | Explicit play-script to script-list table now ported |
| `DatReaderWriter.Tests/IO/DatBinReadWriteSelfTests.cs` | `tests/dat_bin_read_write_self_tests.rs` | Verified | Initial Rust equivalents passing |
| `DatReaderWriter.Tests/IO/DatBTree/DatBTreeReaderWriterTests.cs` | `tests/btree_tests.rs` | Partial | Read-path node and traversal coverage via mock allocator |
| `DatReaderWriter.Tests/*` options-adjacent behavior | `tests/options_tests.rs` | Verified | Rust-specific coverage for options defaults and overrides |
| `DatReaderWriter` database/collection constructor behavior | `tests/collection_tests.rs` | Verified | Synthetic header-backed tests for wrapper validation, path resolution, high-res fallback, typed id enumeration, and qualified-id resolution |
| typed DB object read behavior | `tests/typed_dbobj_tests.rs` | Verified | `Iteration` id resolution, typed read, and typed id enumeration |
| typed asset object read behavior | `tests/typed_asset_tests.rs` | Verified | Palette / SurfaceTexture / RenderSurface / MotionTable / Setup / Animation / SoundTable / PhysicsScriptTable / ObjDesc / CharGen / Region / Scene / Surface / GfxObj / Wave / ParticleEmitter / PhysicsScript coverage |

## What Works Now

- Open the crate and compile successfully.
- Read and parse DAT headers.
- Use a memory-mapped allocator for read-only byte access.
- Traverse the B-tree in read mode.
- Resolve raw file entries by id.
- Read raw file bytes and auto-decompress compressed entries.
- Configure database and collection options from Rust.
- Open the standard four DATs through `DatCollection`.
- Validate specialized database wrappers against the header's DAT type.
- Resolve typed objects through `DatCollection` with portal-to-high-res fallback.
- Enumerate ids by DB object type through both `DatDatabase` and `DatCollection`.
- Resolve `QualifiedDataId<T>` references through `DatCollection`.
- Resolve packed qualified ids for known-mask DBObj references.
- Resolve and read typed `Iteration`, `Palette`, `SurfaceTexture`, `RenderSurface`, `MotionTable`, `Region`, `Scene`, and `Surface` objects.
- Read the core `GfxObj` mesh layout including surfaces, vertex arrays, polygons, and drawing/physics BSP structures.
- Read `Wave`, `ParticleEmitter`, and `PhysicsScript` objects with typed animation hook decoding.
- Read `SoundTable` and `PhysicsScriptTable` objects with explicit typed map payloads.
- Read `Setup` and `Animation` objects with explicit part/frame structures and default asset references.`r`n- Read `CharGen` objects with explicit heritage-group, gender, starting-area, and appearance payloads.
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

## Remaining Major Areas

- Full named generated enum surfaces where read-first wrappers are still used: `Sound`, `TerrainTextureType`, and future enum-heavy ports
- DB object attribute cache and id-to-type resolution beyond the currently hardcoded set
- Generated DBObjs beyond the current asset-focused subset
- Generated Types beyond the current asset-focused subset
- Generated database readers
- Richer cache behavior and higher-level object graph traversal beyond direct `QualifiedDataId<T>` lookups
- Write-path allocator and B-tree behavior if parity is still desired later
- Broader test suite migration with real DAT-backed fixtures that remain external to the crate





