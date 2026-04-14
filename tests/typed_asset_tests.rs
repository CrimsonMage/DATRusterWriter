use dat_reader_writer::{
    DBObjs::{
        CharGen::CharGen, GfxObj::GfxObj, MotionTable::MotionTable, Palette::Palette,
        ParticleEmitter::ParticleEmitter, PhysicsScript::PhysicsScript, Region::Region,
        RenderSurface::RenderSurface, Scene::Scene, Surface::Surface,
        SurfaceTexture::SurfaceTexture, Wave::Wave,
    },
    Generated::Enums::{
        AnimationHookDir::AnimationHookDir, CullMode::CullMode, EmitterType::EmitterType,
        GfxObjFlags::GfxObjFlags, MotionCommand::MotionCommand, ParentLocation::ParentLocation,
        ParticleType::ParticleType, PartsMask::PartsMask, PixelFormat::PixelFormat,
        Placement::Placement, RenderPassType::RenderPassType, SkillId::SkillId, Sound::Sound,
        SpellCategory::SpellCategory, StipplingType::StipplingType, SurfaceType::SurfaceType,
        TerrainTextureType::TerrainTextureType, TextureType::TextureType, UIStateId::UIStateId,
        VertexType::VertexType, VitalId::VitalId,
    },
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::{
        AC1LegacyString::AC1LegacyString,
        AmbientSTBDesc::AmbientSTBDesc,
        AmbientSoundDesc::AmbientSoundDesc,
        AnimationHook::AnimationHook,
        AttackCone::AttackCone,
        BSPTrees::{DrawingBSPNode, DrawingBSPTree},
        ColorARGB::ColorARGB,
        EyeStripCG::EyeStripCG,
        FaceStripCG::FaceStripCG,
        Frame::Frame,
        HashTable::HashTable,
        HeritageGroupCG::HeritageGroupCG,
        LandDefs::LandDefs,
        LandSurf::LandSurf,
        MotionData::MotionData,
        ObjDesc::{ObjDesc, SubPalette, TextureMapChange},
        ObjectDesc::ObjectDesc,
        PStringBase::PStringBase,
        PackedQualifiedDataId::PackedQualifiedDataId,
        PhysicsScriptData::PhysicsScriptData,
        Polygon::Polygon,
        Position::Position,
        QualifiedDataId::QualifiedDataId,
        RegionMisc::RegionMisc,
        SWVertex::SWVertex,
        SceneDesc::SceneDesc,
        SceneType::SceneType,
        SexCG::SexCG,
        SkillCG::SkillCG,
        SoundDesc::SoundDesc,
        StartingArea::StartingArea,
        TMTerrainDesc::TMTerrainDesc,
        TemplateCG::TemplateCG,
        TerrainAlphaMap::TerrainAlphaMap,
        TerrainDesc::TerrainDesc,
        TerrainTex::TerrainTex,
        TerrainType::TerrainType,
        TexMerge::TexMerge,
        Vec2Duv::Vec2Duv,
        VertexArray::VertexArray,
    },
};

#[test]
fn palette_roundtrip_reads_colors() {
    let palette = Palette {
        colors: vec![
            ColorARGB {
                blue: 1,
                green: 2,
                red: 3,
                alpha: 4,
            },
            ColorARGB {
                blue: 5,
                green: 6,
                red: 7,
                alpha: 8,
            },
        ],
        ..Default::default()
    };
    let mut bytes = vec![0u8; 4 + 4 + palette.colors.len() * 4];
    assert!(palette.pack(&mut DatBinWriter::new(&mut bytes)));
    let mut unpacked = Palette::default();
    assert!(unpacked.unpack(&mut DatBinReader::new(&bytes)));
    assert_eq!(2, unpacked.colors.len());
    assert_eq!(4, unpacked.colors[0].alpha);
    assert_eq!(7, unpacked.colors[1].red);
}

#[test]
fn surface_texture_roundtrip_reads_texture_ids() {
    let surface = SurfaceTexture {
        texture_type: TextureType::TEXTURE2D,
        textures: vec![
            QualifiedDataId::new(0x06000001),
            QualifiedDataId::new(0x06000002),
        ],
        ..Default::default()
    };
    let mut bytes = vec![0u8; 4 + 4 + 1 + 4 + 8];
    assert!(surface.pack(&mut DatBinWriter::new(&mut bytes)));
    let mut unpacked = SurfaceTexture::default();
    assert!(unpacked.unpack(&mut DatBinReader::new(&bytes)));
    assert_eq!(TextureType::TEXTURE2D, unpacked.texture_type);
    assert_eq!(0x06000002, unpacked.textures[1].data_id);
}

#[test]
fn render_surface_reads_palette_tail_for_indexed_formats() {
    let render = RenderSurface {
        width: 64,
        height: 32,
        format: PixelFormat::PFID_P8,
        source_data: vec![1, 2, 3],
        default_palette_id: 0x04000001,
        ..Default::default()
    };
    let mut bytes = vec![0u8; 8 + 8 + 4 + 4 + 3 + 4];
    assert!(render.pack(&mut DatBinWriter::new(&mut bytes)));
    let mut unpacked = RenderSurface::default();
    assert!(unpacked.unpack(&mut DatBinReader::new(&bytes)));
    assert_eq!(64, unpacked.width);
    assert_eq!(PixelFormat::PFID_P8, unpacked.format);
    assert_eq!(0x04000001, unpacked.default_palette_id);
}

#[test]
fn motion_table_reads_defaults_and_cycles() {
    let mut motion = MotionTable::default();
    motion.default_style = MotionCommand(0x12345678);
    motion
        .style_defaults
        .insert(MotionCommand(1), MotionCommand(2));
    motion.cycles.insert(10, MotionData::default());
    let mut bytes = vec![0u8; 2048];
    assert!(motion.pack(&mut DatBinWriter::new(&mut bytes)));
    let mut unpacked = MotionTable::default();
    assert!(unpacked.unpack(&mut DatBinReader::new(&bytes)));
    assert_eq!(MotionCommand(0x12345678), unpacked.default_style);
    assert_eq!(
        Some(&MotionCommand(2)),
        unpacked.style_defaults.get(&MotionCommand(1))
    );
    assert!(unpacked.cycles.contains_key(&10));
}

#[test]
fn region_reads_sound_scene_terrain_and_misc() {
    let region = Region {
        region_number: 7,
        version: 3,
        region_name: AC1LegacyString {
            value: "Dereth".to_string(),
        },
        land_defs: LandDefs {
            land_height_table: vec![0.0; 256],
            ..Default::default()
        },
        parts_mask: PartsMask::HasSoundInfo | PartsMask::HasSceneInfo | PartsMask::HasRegionMisc,
        sound_info: Some(SoundDesc {
            stb_desc: vec![AmbientSTBDesc {
                stb_id: 11,
                ambient_sounds: vec![AmbientSoundDesc {
                    s_type: Sound::AMBIENT1,
                    volume: 0.75,
                    base_chance: 0.25,
                    min_rate: 1.0,
                    max_rate: 3.0,
                }],
            }],
        }),
        scene_info: Some(SceneDesc {
            scene_types: vec![SceneType {
                stb_index: 5,
                scenes: vec![QualifiedDataId::new(0x12000001)],
            }],
        }),
        terrain_info: TerrainDesc {
            terrain_types: vec![TerrainType {
                terrain_name: AC1LegacyString {
                    value: "Grass".to_string(),
                },
                terrain_color: ColorARGB {
                    blue: 1,
                    green: 2,
                    red: 3,
                    alpha: 4,
                },
                scene_types: vec![5, 6],
            }],
            land_surfaces: LandSurf {
                land_type: 9,
                tex_merge: TexMerge {
                    base_tex_size: 32,
                    corner_terrain_maps: vec![TerrainAlphaMap {
                        t_code: 1,
                        texture_id: QualifiedDataId::new(0x05000001),
                    }],
                    side_terrain_maps: vec![],
                    road_maps: vec![],
                    terrain_desc: vec![TMTerrainDesc {
                        terrain_type: TerrainTextureType(1),
                        terrain_tex: TerrainTex {
                            texture_id: QualifiedDataId::new(0x05000002),
                            tex_tiling: 2,
                            max_vert_bright: 3,
                            min_vert_bright: 4,
                            max_vert_saturate: 5,
                            min_vert_saturate: 6,
                            max_vert_hue: 7,
                            min_vert_hue: 8,
                            detail_tex_tiling: 9,
                            detail_texture_id: QualifiedDataId::new(0x05000003),
                        },
                    }],
                },
            },
        },
        region_misc: Some(RegionMisc {
            version: 1,
            game_map_id: 0x0600127D,
            autotest_map_id: 0x06000261,
            autotest_map_size: 4,
            clear_cell_id: 0x01000FDE,
            clear_monster_id: 0x01001612,
        }),
        ..Default::default()
    };

    let mut bytes = vec![0u8; 4096];
    let mut writer = DatBinWriter::new(&mut bytes);
    assert!(region.pack(&mut writer));
    let used = writer.offset();

    let mut unpacked = Region::default();
    assert!(unpacked.unpack(&mut DatBinReader::new(&bytes[..used])));
    assert_eq!(7, unpacked.region_number);
    assert_eq!("Dereth", unpacked.region_name.value);
    assert_eq!(11, unpacked.sound_info.as_ref().unwrap().stb_desc[0].stb_id);
    assert_eq!(
        Sound::AMBIENT1,
        unpacked.sound_info.as_ref().unwrap().stb_desc[0].ambient_sounds[0].s_type
    );
    assert_eq!(
        5,
        unpacked.scene_info.as_ref().unwrap().scene_types[0].stb_index
    );
    assert_eq!(
        "Grass",
        unpacked.terrain_info.terrain_types[0].terrain_name.value
    );
    assert_eq!(
        32,
        unpacked.terrain_info.land_surfaces.tex_merge.base_tex_size
    );
    assert_eq!(
        TerrainTextureType(1),
        unpacked.terrain_info.land_surfaces.tex_merge.terrain_desc[0].terrain_type
    );
    assert_eq!(
        0x0600127D,
        unpacked.region_misc.as_ref().unwrap().game_map_id
    );
    assert!(unpacked.raw_remainder.is_empty());
}

#[test]
fn scene_roundtrip_reads_object_descriptors() {
    let scene = Scene {
        objects: vec![ObjectDesc {
            object_id: 0x01000044,
            base_loc: Frame::default(),
            frequency: 0.5,
            displace_x: 1.0,
            displace_y: 2.0,
            min_scale: 0.8,
            max_scale: 1.2,
            max_rotation: 3.0,
            min_slope: 4.0,
            max_slope: 5.0,
            align: 6,
            orient: 7,
            weenie_obj: 0x02000008,
        }],
        ..Default::default()
    };

    let mut bytes = vec![0u8; 512];
    let mut writer = DatBinWriter::new(&mut bytes);
    assert!(scene.pack(&mut writer));
    let used = writer.offset();
    let mut unpacked = Scene::default();
    assert!(unpacked.unpack(&mut DatBinReader::new(&bytes[..used])));
    assert_eq!(1, unpacked.objects.len());
    assert_eq!(0x01000044, unpacked.objects[0].object_id);
    assert_eq!(7, unpacked.objects[0].orient);
}

#[test]
fn surface_roundtrip_reads_textured_surface() {
    let surface = Surface {
        surface_type: SurfaceType::Base1Image | SurfaceType::Diffuse,
        orig_texture_id: QualifiedDataId::new(0x05000011),
        orig_palette_id: QualifiedDataId::new(0x04000022),
        translucency: 0.25,
        luminosity: 0.5,
        diffuse: 0.75,
        ..Default::default()
    };

    let mut bytes = vec![0u8; 128];
    let mut writer = DatBinWriter::new(&mut bytes);
    assert!(surface.pack(&mut writer));
    let used = writer.offset();
    let mut unpacked = Surface::default();
    assert!(unpacked.unpack(&mut DatBinReader::new(&bytes[..used])));
    assert_eq!(0x05000011, unpacked.orig_texture_id.data_id);
    assert_eq!(0x04000022, unpacked.orig_palette_id.data_id);
    assert_eq!(0.75, unpacked.diffuse);
}

#[test]
fn gfx_obj_roundtrip_reads_surfaces_vertices_and_polygons() {
    let mut gfx = GfxObj {
        flags: GfxObjFlags::HasDrawing,
        surfaces: vec![QualifiedDataId::new(0x08000010)],
        vertex_array: VertexArray {
            vertex_type: VertexType(1),
            vertices: [(
                1u16,
                SWVertex {
                    uvs: vec![Vec2Duv { u: 1.0, v: 2.0 }],
                    ..Default::default()
                },
            )]
            .into_iter()
            .collect(),
        },
        polygons: [(
            7u16,
            Polygon {
                stippling: StipplingType::Positive | StipplingType::NoNeg,
                sides_type: CullMode::CLOCKWISE,
                pos_surface: 1,
                neg_surface: -1,
                vertex_ids: vec![1, 2, 3],
                pos_uv_indices: vec![0, 1, 2],
                neg_uv_indices: vec![],
            },
        )]
        .into_iter()
        .collect(),
        drawing_bsp: DrawingBSPTree {
            root: DrawingBSPNode::default(),
        },
        ..Default::default()
    };
    gfx.sort_center.x = 9.0;

    let mut bytes = vec![0u8; 2048];
    let mut writer = DatBinWriter::new(&mut bytes);
    assert!(gfx.pack(&mut writer));
    let used = writer.offset();
    let mut unpacked = GfxObj::default();
    assert!(unpacked.unpack(&mut DatBinReader::new(&bytes[..used])));
    assert_eq!(1, unpacked.surfaces.len());
    assert_eq!(0x08000010, unpacked.surfaces[0].data_id);
    assert_eq!(1, unpacked.vertex_array.vertices.len());
    assert_eq!(
        Some(&Polygon {
            stippling: StipplingType::Positive | StipplingType::NoNeg,
            sides_type: CullMode::CLOCKWISE,
            pos_surface: 1,
            neg_surface: -1,
            vertex_ids: vec![1, 2, 3],
            pos_uv_indices: vec![0, 1, 2],
            neg_uv_indices: vec![],
        }),
        unpacked.polygons.get(&7)
    );
}

#[test]
fn wave_roundtrip_reads_header_and_data() {
    let wave = Wave {
        header: vec![1, 2, 3, 4],
        data: vec![5, 6, 7],
        ..Default::default()
    };

    let mut bytes = vec![0u8; 128];
    let mut writer = DatBinWriter::new(&mut bytes);
    assert!(wave.pack(&mut writer));
    let used = writer.offset();
    let mut unpacked = Wave::default();
    assert!(unpacked.unpack(&mut DatBinReader::new(&bytes[..used])));
    assert_eq!(vec![1, 2, 3, 4], unpacked.header);
    assert_eq!(vec![5, 6, 7], unpacked.data);
}

#[test]
fn particle_emitter_roundtrip_reads_core_fields() {
    let emitter = ParticleEmitter {
        unknown: 9,
        emitter_type: EmitterType(1),
        particle_type: ParticleType(2),
        gfx_obj_id: QualifiedDataId::new(0x01000033),
        hw_gfx_obj_id: QualifiedDataId::new(0x01000044),
        birthrate: 2.5,
        max_particles: 3,
        initial_particles: 4,
        total_particles: 5,
        total_seconds: 6.0,
        lifespan: 7.0,
        lifespan_rand: 8.0,
        min_offset: 1.0,
        max_offset: 2.0,
        min_a: 3.0,
        max_a: 4.0,
        min_b: 5.0,
        max_b: 6.0,
        min_c: 7.0,
        max_c: 8.0,
        start_scale: 0.5,
        final_scale: 1.5,
        scale_rand: 0.1,
        start_trans: 0.2,
        final_trans: 0.3,
        trans_rand: 0.4,
        is_parent_local: true,
        ..Default::default()
    };

    let mut bytes = vec![0u8; 512];
    let mut writer = DatBinWriter::new(&mut bytes);
    assert!(emitter.pack(&mut writer));
    let used = writer.offset();
    let mut unpacked = ParticleEmitter::default();
    assert!(unpacked.unpack(&mut DatBinReader::new(&bytes[..used])));
    assert_eq!(EmitterType(1), unpacked.emitter_type);
    assert_eq!(ParticleType(2), unpacked.particle_type);
    assert_eq!(0x01000033, unpacked.gfx_obj_id.data_id);
    assert!(unpacked.is_parent_local);
}

#[test]
fn physics_script_roundtrip_reads_mixed_hooks() {
    let script = PhysicsScript {
        script_data: vec![
            PhysicsScriptData {
                start_time: 1.25,
                hook: AnimationHook::Sound {
                    direction: AnimationHookDir::FORWARD,
                    id: QualifiedDataId::new(0x0A000010),
                },
            },
            PhysicsScriptData {
                start_time: 2.5,
                hook: AnimationHook::ReplaceObject {
                    direction: AnimationHookDir::BACKWARD,
                    part_index: 7,
                    part_id: PackedQualifiedDataId::new(0x01000022),
                },
            },
            PhysicsScriptData {
                start_time: 3.75,
                hook: AnimationHook::CreateParticle {
                    direction: AnimationHookDir::BOTH,
                    emitter_info_id: QualifiedDataId::new(0x32000011),
                    part_index: 4,
                    offset: Frame::default(),
                    emitter_id: 99,
                },
            },
            PhysicsScriptData {
                start_time: 5.0,
                hook: AnimationHook::Attack {
                    direction: AnimationHookDir::FORWARD,
                    attack_cone: AttackCone {
                        part_index: 3,
                        radius: 4.0,
                        height: 5.0,
                        ..Default::default()
                    },
                },
            },
        ],
        ..Default::default()
    };

    let mut bytes = vec![0u8; 1024];
    let mut writer = DatBinWriter::new(&mut bytes);
    assert!(script.pack(&mut writer));
    let used = writer.offset();
    let mut unpacked = PhysicsScript::default();
    assert!(unpacked.unpack(&mut DatBinReader::new(&bytes[..used])));
    assert_eq!(4, unpacked.script_data.len());
    assert_eq!(
        AnimationHook::Sound {
            direction: AnimationHookDir::FORWARD,
            id: QualifiedDataId::new(0x0A000010)
        },
        unpacked.script_data[0].hook
    );
    assert_eq!(
        AnimationHook::ReplaceObject {
            direction: AnimationHookDir::BACKWARD,
            part_index: 7,
            part_id: PackedQualifiedDataId::new(0x01000022)
        },
        unpacked.script_data[1].hook
    );
    assert_eq!(
        AnimationHook::CreateParticle {
            direction: AnimationHookDir::BOTH,
            emitter_info_id: QualifiedDataId::new(0x32000011),
            part_index: 4,
            offset: Frame::default(),
            emitter_id: 99
        },
        unpacked.script_data[2].hook
    );
}

#[test]
fn sound_table_roundtrip_reads_hashes_and_entries() {
    use dat_reader_writer::{
        DBObjs::SoundTable::SoundTable,
        Types::{SoundData::SoundData, SoundEntry::SoundEntry, SoundHashData::SoundHashData},
    };

    let mut sound_table = SoundTable {
        hash_key: 77,
        ..Default::default()
    };
    sound_table.hashes.insert(
        0x1234,
        SoundHashData {
            priority: 1.0,
            probability: 0.5,
            volume: 0.25,
        },
    );
    sound_table.sounds.insert(
        Sound::AMBIENT1,
        SoundData {
            entries: vec![SoundEntry {
                id: QualifiedDataId::new(0x0A000020),
                priority: 2.0,
                probability: 0.75,
                volume: 0.5,
            }],
            unknown: 9,
        },
    );

    let mut bytes = vec![0u8; 512];
    let mut writer = DatBinWriter::new(&mut bytes);
    assert!(sound_table.pack(&mut writer));
    let used = writer.offset();

    let mut unpacked = SoundTable::default();
    assert!(unpacked.unpack(&mut DatBinReader::new(&bytes[..used])));
    assert_eq!(77, unpacked.hash_key);
    assert_eq!(
        Some(&0.25),
        unpacked.hashes.get(&0x1234).map(|value| &value.volume)
    );
    assert_eq!(
        0x0A000020,
        unpacked.sounds.get(&Sound::AMBIENT1).unwrap().entries[0]
            .id
            .data_id
    );
}

#[test]
fn physics_script_table_roundtrip_reads_script_map() {
    use dat_reader_writer::{
        DBObjs::PhysicsScriptTable::PhysicsScriptTable,
        Generated::Enums::PlayScript::PlayScript,
        Types::{
            PhysicsScriptTableData::PhysicsScriptTableData, ScriptAndModData::ScriptAndModData,
        },
    };

    let mut script_table = PhysicsScriptTable::default();
    script_table.script_table.insert(
        PlayScript::LAUNCH,
        PhysicsScriptTableData {
            scripts: vec![ScriptAndModData {
                mod_value: 1.5,
                script_id: QualifiedDataId::new(0x33000010),
            }],
        },
    );

    let mut bytes = vec![0u8; 256];
    let mut writer = DatBinWriter::new(&mut bytes);
    assert!(script_table.pack(&mut writer));
    let used = writer.offset();

    let mut unpacked = PhysicsScriptTable::default();
    assert!(unpacked.unpack(&mut DatBinReader::new(&bytes[..used])));
    assert_eq!(1, unpacked.script_table.len());
    assert_eq!(
        1.5,
        unpacked
            .script_table
            .get(&PlayScript::LAUNCH)
            .unwrap()
            .scripts[0]
            .mod_value
    );
    assert_eq!(
        0x33000010,
        unpacked
            .script_table
            .get(&PlayScript::LAUNCH)
            .unwrap()
            .scripts[0]
            .script_id
            .data_id
    );
}

#[test]
fn animation_roundtrip_reads_pos_frames_and_hooks() {
    use dat_reader_writer::{
        DBObjs::Animation::Animation, Generated::Enums::AnimationFlags::AnimationFlags,
        Types::AnimationFrame::AnimationFrame,
    };

    let animation = Animation {
        flags: AnimationFlags::PosFrames,
        num_parts: 2,
        pos_frames: vec![Frame::default()],
        part_frames: vec![AnimationFrame {
            frames: vec![Frame::default(), Frame::default()],
            hooks: vec![AnimationHook::AnimationDone {
                direction: AnimationHookDir::FORWARD,
            }],
        }],
        ..Default::default()
    };

    let mut bytes = vec![0u8; 512];
    let mut writer = DatBinWriter::new(&mut bytes);
    assert!(animation.pack(&mut writer));
    let used = writer.offset();

    let mut unpacked = Animation::default();
    assert!(unpacked.unpack(&mut DatBinReader::new(&bytes[..used])));
    assert_eq!(AnimationFlags::PosFrames, unpacked.flags);
    assert_eq!(2, unpacked.num_parts);
    assert_eq!(1, unpacked.pos_frames.len());
    assert_eq!(2, unpacked.part_frames[0].frames.len());
    assert_eq!(
        AnimationHook::AnimationDone {
            direction: AnimationHookDir::FORWARD
        },
        unpacked.part_frames[0].hooks[0]
    );
}

#[test]
fn setup_roundtrip_reads_optional_arrays_and_default_refs() {
    use dat_reader_writer::Lib::IO::Numerics::Vector3;
    use dat_reader_writer::{
        DBObjs::Setup::Setup,
        Generated::Enums::{
            ParentLocation::ParentLocation, Placement::Placement, SetupFlags::SetupFlags,
        },
        Types::{
            AnimationFrame::AnimationFrame, CylSphere::CylSphere, LightInfo::LightInfo,
            LocationType::LocationType, Sphere::Sphere,
        },
    };

    let mut setup = Setup {
        flags: SetupFlags::HasParent | SetupFlags::HasDefaultScale,
        parts: vec![
            QualifiedDataId::new(0x01000001),
            QualifiedDataId::new(0x01000002),
        ],
        parent_index: vec![0, 1],
        default_scale: vec![Vector3::new(1.0, 1.0, 1.0), Vector3::new(2.0, 2.0, 2.0)],
        cyl_spheres: vec![CylSphere {
            radius: 3.0,
            height: 4.0,
            ..Default::default()
        }],
        spheres: vec![Sphere {
            radius: 5.0,
            ..Default::default()
        }],
        height: 6.0,
        radius: 7.0,
        step_up_height: 8.0,
        step_down_height: 9.0,
        sorting_sphere: Sphere {
            radius: 10.0,
            ..Default::default()
        },
        selection_sphere: Sphere {
            radius: 11.0,
            ..Default::default()
        },
        default_animation: QualifiedDataId::new(0x03000011),
        default_script: QualifiedDataId::new(0x33000012),
        default_motion_table: QualifiedDataId::new(0x09000013),
        default_sound_table: QualifiedDataId::new(0x20000014),
        default_script_table: QualifiedDataId::new(0x34000015),
        ..Default::default()
    };
    setup.holding_locations.insert(
        ParentLocation::RIGHT_HAND,
        LocationType {
            part_id: 3,
            frame: Frame::default(),
        },
    );
    setup.connection_points.insert(
        ParentLocation::LEFT_HAND,
        LocationType {
            part_id: 4,
            frame: Frame::default(),
        },
    );
    setup.placement_frames.insert(
        Placement::HOOK,
        AnimationFrame {
            frames: vec![Frame::default(), Frame::default()],
            hooks: vec![AnimationHook::DefaultScript {
                direction: AnimationHookDir::BOTH,
            }],
        },
    );
    setup.lights.insert(
        5,
        LightInfo {
            intensity: 1.5,
            falloff: 2.5,
            cone_angle: 3.5,
            ..Default::default()
        },
    );

    let mut bytes = vec![0u8; 2048];
    let mut writer = DatBinWriter::new(&mut bytes);
    assert!(setup.pack(&mut writer));
    let used = writer.offset();

    let mut unpacked = Setup::default();
    assert!(unpacked.unpack(&mut DatBinReader::new(&bytes[..used])));
    assert_eq!(2, unpacked.num_parts);
    assert_eq!(vec![0, 1], unpacked.parent_index);
    assert_eq!(2, unpacked.default_scale.len());
    assert_eq!(
        Some(&3),
        unpacked
            .holding_locations
            .get(&ParentLocation::RIGHT_HAND)
            .map(|value| &value.part_id)
    );
    assert_eq!(
        2,
        unpacked
            .placement_frames
            .get(&Placement::HOOK)
            .unwrap()
            .frames
            .len()
    );
    assert_eq!(1.5, unpacked.lights.get(&5).unwrap().intensity);
    assert_eq!(0x34000015, unpacked.default_script_table.data_id);
}

#[test]
fn obj_desc_roundtrip_reads_palette_texture_and_animation_changes() {
    let obj_desc = ObjDesc {
        unknown_byte: 7,
        palette_id: PackedQualifiedDataId::new(0x04000010),
        sub_palettes: vec![SubPalette {
            sub_id: PackedQualifiedDataId::new(0x04000011),
            offset: 2,
            num_colors: 3,
        }],
        texture_changes: vec![TextureMapChange {
            part_index: 4,
            old_texture: PackedQualifiedDataId::new(0x05000020),
            new_texture: PackedQualifiedDataId::new(0x05000021),
        }],
        anim_part_changes: vec![
            dat_reader_writer::Types::AnimationPartChange::AnimationPartChange {
                part_index: 5,
                part_id: PackedQualifiedDataId::new(0x01000030),
            },
        ],
    };

    let mut bytes = vec![0u8; 256];
    let mut writer = DatBinWriter::new(&mut bytes);
    assert!(obj_desc.pack(&mut writer));
    let used = writer.offset();

    let mut unpacked = ObjDesc::default();
    assert!(unpacked.unpack(&mut DatBinReader::new(&bytes[..used])));
    assert_eq!(7, unpacked.unknown_byte);
    assert_eq!(0x04000010, unpacked.palette_id.data_id);
    assert_eq!(0x04000011, unpacked.sub_palettes[0].sub_id.data_id);
    assert_eq!(0x05000020, unpacked.texture_changes[0].old_texture.data_id);
    assert_eq!(0x01000030, unpacked.anim_part_changes[0].part_id.data_id);
}

#[test]
fn char_gen_roundtrip_reads_heritage_groups() {
    use dat_reader_writer::Types::GearCG::GearCG;
    use dat_reader_writer::Types::HairStyleCG::HairStyleCG;

    let mut genders = HashTable::<i32, SexCG>::default();
    genders.insert(
        0,
        SexCG {
            name: PStringBase::from("Male"),
            scale: 100,
            setup_id: QualifiedDataId::new(0x02000010),
            sound_table: QualifiedDataId::new(0x20000011),
            icon_id: QualifiedDataId::new(0x06000012),
            base_palette: QualifiedDataId::new(0x04000013),
            skin_pal_set: QualifiedDataId::new(0x0F000014),
            physics_table: QualifiedDataId::new(0x34000015),
            motion_table: QualifiedDataId::new(0x09000016),
            combat_table: QualifiedDataId::new(0x30000017),
            base_obj_desc: ObjDesc {
                unknown_byte: 1,
                palette_id: PackedQualifiedDataId::new(0x04000020),
                sub_palettes: vec![SubPalette {
                    sub_id: PackedQualifiedDataId::new(0x04000021),
                    offset: 2,
                    num_colors: 3,
                }],
                texture_changes: vec![TextureMapChange {
                    part_index: 4,
                    old_texture: PackedQualifiedDataId::new(0x05000022),
                    new_texture: PackedQualifiedDataId::new(0x05000023),
                }],
                anim_part_changes: vec![
                    dat_reader_writer::Types::AnimationPartChange::AnimationPartChange {
                        part_index: 5,
                        part_id: PackedQualifiedDataId::new(0x01000024),
                    },
                ],
            },
            hair_colors: vec![0x11223344],
            hair_styles: vec![HairStyleCG {
                icon_id: QualifiedDataId::new(0x06000030),
                bald: false,
                alternate_setup: 0x02000031,
                obj_desc: ObjDesc::default(),
            }],
            eye_colors: vec![0x55667788],
            eye_strips: vec![EyeStripCG {
                icon_id: QualifiedDataId::new(0x06000032),
                bald_icon_id: 0x06000033,
                obj_desc: ObjDesc::default(),
                bald_obj_desc: ObjDesc::default(),
            }],
            nose_strips: vec![FaceStripCG {
                icon_id: QualifiedDataId::new(0x06000034),
                obj_desc: ObjDesc::default(),
            }],
            mouth_strips: vec![FaceStripCG {
                icon_id: QualifiedDataId::new(0x06000035),
                obj_desc: ObjDesc::default(),
            }],
            headgears: vec![GearCG {
                name: PStringBase::from("Helm"),
                clothing_table: QualifiedDataId::new(0x10000040),
                weenie_default: 0x02000041,
            }],
            shirts: vec![],
            pants: vec![],
            footwear: vec![],
            clothing_colors: vec![0x99AABBCC],
        },
    );

    let mut heritage_groups = HashTable::<u32, HeritageGroupCG>::default();
    heritage_groups.insert(
        1,
        HeritageGroupCG {
            name: PStringBase::from("Aluvian"),
            icon_id: QualifiedDataId::new(0x06000050),
            setup_id: QualifiedDataId::new(0x02000051),
            environment_setup_id: QualifiedDataId::new(0x02000052),
            attribute_credits: 10,
            skill_credits: 20,
            primary_start_areas: vec![100, 101],
            secondary_start_areas: vec![200],
            skills: vec![SkillCG {
                id: SkillId::BOW,
                normal_cost: 5,
                primary_cost: 3,
            }],
            templates: vec![TemplateCG {
                name: PStringBase::from("Archer"),
                icon_id: QualifiedDataId::new(0x06000053),
                title: 7,
                strength: 10,
                endurance: 11,
                coordination: 12,
                quickness: 13,
                focus: 14,
                self_value: 15,
                normal_skills: vec![SkillId::BOW],
                primary_skills: vec![SkillId::MISSILE_DEFENSE],
            }],
            genders,
        },
    );

    let char_gen = CharGen {
        data_id: QualifiedDataId::new(0x0E000002),
        starting_areas: vec![StartingArea {
            name: PStringBase::from("Training Hall"),
            locations: vec![Position {
                cell_id: 0x01020304,
                frame: Frame::default(),
            }],
        }],
        heritage_groups,
        ..Default::default()
    };

    let mut bytes = vec![0u8; 8192];
    let mut writer = DatBinWriter::new(&mut bytes);
    assert!(char_gen.pack(&mut writer));
    let used = writer.offset();

    let mut unpacked = CharGen::default();
    assert!(unpacked.unpack(&mut DatBinReader::new(&bytes[..used])));
    assert_eq!(0x0E000002, unpacked.data_id.data_id);
    assert_eq!("Training Hall", unpacked.starting_areas[0].name.value);
    assert_eq!(0x01020304, unpacked.starting_areas[0].locations[0].cell_id);
    let heritage = unpacked.heritage_groups.get(&1).unwrap();
    assert_eq!("Aluvian", heritage.name.value);
    assert_eq!(SkillId::BOW, heritage.skills[0].id);
    assert_eq!("Male", heritage.genders.get(&0).unwrap().name.value);
    assert_eq!(
        0x10000040,
        heritage.genders.get(&0).unwrap().headgears[0]
            .clothing_table
            .data_id
    );
}

#[test]
fn drawing_bsp_portal_roundtrip_reads_children_polygons_and_portals() {
    use dat_reader_writer::Generated::Enums::BSPNodeType::BSPNodeType;
    use dat_reader_writer::Lib::IO::Numerics::{Plane, Vector3};
    use dat_reader_writer::Types::{
        BSPTrees::{DrawingBSPNode, DrawingBSPTree},
        PortalRef::PortalRef,
        Sphere::Sphere,
    };

    let tree = DrawingBSPTree {
        root: DrawingBSPNode {
            node_type: BSPNodeType::PORTAL,
            splitting_plane: Plane::new(Vector3::new(1.0, 0.0, 0.0), 2.5),
            pos_node: Some(Box::new(DrawingBSPNode {
                node_type: BSPNodeType::LEAF,
                leaf_index: 10,
                ..Default::default()
            })),
            neg_node: Some(Box::new(DrawingBSPNode {
                node_type: BSPNodeType::LEAF,
                leaf_index: 11,
                ..Default::default()
            })),
            bounding_sphere: Sphere {
                origin: Vector3::new(3.0, 4.0, 5.0),
                radius: 6.0,
            },
            polygons: vec![7, 8],
            portals: vec![PortalRef {
                poly_id: 9,
                portal_index: 2,
            }],
            ..Default::default()
        },
    };

    let mut bytes = vec![0u8; 512];
    let mut writer = DatBinWriter::new(&mut bytes);
    assert!(tree.pack(&mut writer));
    let used = writer.offset();

    let mut unpacked = DrawingBSPTree::default();
    assert!(unpacked.unpack(&mut DatBinReader::new(&bytes[..used])));
    assert_eq!(BSPNodeType::PORTAL, unpacked.root.node_type);
    assert_eq!(
        Some(10),
        unpacked.root.pos_node.as_ref().map(|node| node.leaf_index)
    );
    assert_eq!(
        Some(11),
        unpacked.root.neg_node.as_ref().map(|node| node.leaf_index)
    );
    assert_eq!(vec![7, 8], unpacked.root.polygons);
    assert_eq!(
        PortalRef {
            poly_id: 9,
            portal_index: 2
        },
        unpacked.root.portals[0]
    );
}

#[test]
fn physics_bsp_portal_nodes_are_rejected_like_reference() {
    use dat_reader_writer::Generated::Enums::BSPNodeType::BSPNodeType;
    use dat_reader_writer::Types::BSPTrees::PhysicsBSPNode;

    let mut bytes = vec![0u8; 8];
    let mut writer = DatBinWriter::new(&mut bytes);
    writer.write_i32(BSPNodeType::PORTAL.into());
    let used = writer.offset();

    let mut node = PhysicsBSPNode::default();
    assert!(!node.unpack(&mut DatBinReader::new(&bytes[..used])));
}

#[test]
fn gfx_obj_roundtrip_reads_physics_drawing_and_degrade() {
    use dat_reader_writer::Generated::Enums::BSPNodeType::BSPNodeType;
    use dat_reader_writer::Lib::IO::Numerics::{Plane, Vector3};
    use dat_reader_writer::Types::{
        BSPTrees::{DrawingBSPNode, DrawingBSPTree, PhysicsBSPNode, PhysicsBSPTree},
        Sphere::Sphere,
    };

    let gfx = GfxObj {
        flags: GfxObjFlags::HasPhysics | GfxObjFlags::HasDrawing | GfxObjFlags::HasDIDDegrade,
        surfaces: vec![QualifiedDataId::new(0x08000010)],
        vertex_array: VertexArray::default(),
        physics_polygons: [(3u16, Polygon::default())].into_iter().collect(),
        physics_bsp: PhysicsBSPTree {
            root: PhysicsBSPNode {
                node_type: BSPNodeType::LEAF,
                leaf_index: 4,
                solid: 1,
                bounding_sphere: Sphere {
                    origin: Vector3::new(1.0, 2.0, 3.0),
                    radius: 4.0,
                },
                polygons: vec![3],
                ..Default::default()
            },
        },
        sort_center: Vector3::new(9.0, 8.0, 7.0),
        polygons: [(7u16, Polygon::default())].into_iter().collect(),
        drawing_bsp: DrawingBSPTree {
            root: DrawingBSPNode {
                node_type: BSPNodeType::BPIN,
                splitting_plane: Plane::new(Vector3::new(0.0, 1.0, 0.0), 1.5),
                pos_node: Some(Box::new(DrawingBSPNode {
                    node_type: BSPNodeType::LEAF,
                    leaf_index: 12,
                    ..Default::default()
                })),
                neg_node: Some(Box::new(DrawingBSPNode {
                    node_type: BSPNodeType::LEAF,
                    leaf_index: 13,
                    ..Default::default()
                })),
                bounding_sphere: Sphere {
                    origin: Vector3::new(5.0, 6.0, 7.0),
                    radius: 8.0,
                },
                polygons: vec![7],
                ..Default::default()
            },
        },
        did_degrade: 0x01000099,
        ..Default::default()
    };

    let mut bytes = vec![0u8; 2048];
    let mut writer = DatBinWriter::new(&mut bytes);
    assert!(gfx.pack(&mut writer));
    let used = writer.offset();

    let mut unpacked = GfxObj::default();
    assert!(unpacked.unpack(&mut DatBinReader::new(&bytes[..used])));
    assert_eq!(
        GfxObjFlags::HasPhysics | GfxObjFlags::HasDrawing | GfxObjFlags::HasDIDDegrade,
        unpacked.flags
    );
    assert_eq!(Some(&Polygon::default()), unpacked.physics_polygons.get(&3));
    assert_eq!(4, unpacked.physics_bsp.root.leaf_index);
    assert_eq!(
        Some(12),
        unpacked
            .drawing_bsp
            .root
            .pos_node
            .as_ref()
            .map(|node| node.leaf_index)
    );
    assert_eq!(
        Some(13),
        unpacked
            .drawing_bsp
            .root
            .neg_node
            .as_ref()
            .map(|node| node.leaf_index)
    );
    assert_eq!(0x01000099, unpacked.did_degrade);
}

#[test]
fn generated_enum_surfaces_cover_remaining_skill_terrain_and_asset_constants() {
    assert_eq!(SkillId::CHALLENGE, SkillId::from(0x35));
    assert_eq!(SkillId::SUMMONING, SkillId::from(0x36));
    assert_eq!(
        TerrainTextureType::ROAD_TYPE,
        TerrainTextureType::from(0x20)
    );
    assert_eq!(
        TerrainTextureType::WATER_DEEP_SEA,
        TerrainTextureType::from(0x14)
    );
    assert_eq!(ParentLocation::LeftWeapon, ParentLocation::from(0x08));
    assert_eq!(Placement::Random10, Placement::from(0x82));
    assert_eq!(EmitterType::BirthratePerMeter, EmitterType::from(0x02));
    assert_eq!(ParticleType::GlobalVelocity, ParticleType::from(0x0C));
    assert_eq!(VertexType::CSWVertexType, VertexType::from(0x01));
    assert_eq!(CullMode::CounterClockwise, CullMode::from(0x03));
    assert_eq!(CullMode::Clockwise, CullMode::CLOCKWISE);
    assert_eq!(UIStateId::LockedUI, UIStateId::from(0x10000063));
    assert_eq!(UIStateId::Dialog_pending_true, UIStateId::from(0x18));
    assert_eq!(RenderPassType::AL_1DL_7PL_Fog, RenderPassType::from(0x2C));
    assert_eq!(RenderPassType::AlphaBlend, RenderPassType::from(0x0A));
    assert_eq!(SpellCategory::PortalSending, SpellCategory::from(0xD6));
    assert_eq!(SpellCategory::SummoningLowering, SpellCategory::from(0x2B9));
    assert_eq!(VitalId::MaximumHealth, VitalId::from(0x01));
    assert_eq!(VitalId::MaximumMana, VitalId::from(0x05));
}

#[test]
fn clothing_table_roundtrip_reads_base_and_subpal_effects() {
    use dat_reader_writer::{
        DBObjs::ClothingTable::ClothingTable,
        Types::{
            CloObjectEffect::CloObjectEffect, CloSubPalEffect::CloSubPalEffect,
            CloSubPalette::CloSubPalette, CloSubPaletteRange::CloSubPaletteRange,
            CloTextureEffect::CloTextureEffect, ClothingBaseEffect::ClothingBaseEffect,
            PackableHashTable::PackableHashTable,
        },
    };

    let mut table = ClothingTable::default();
    table.clothing_base_effects = PackableHashTable::default();
    table.clothing_base_effects.insert(
        QualifiedDataId::new(0x02000010),
        ClothingBaseEffect {
            clo_object_effects: vec![CloObjectEffect {
                index: 1,
                model_id: QualifiedDataId::new(0x01000020),
                clo_texture_effects: vec![CloTextureEffect {
                    old_texture: QualifiedDataId::new(0x05000030),
                    new_texture: QualifiedDataId::new(0x05000031),
                }],
            }],
        },
    );
    table.clothing_sub_pal_effects = PackableHashTable::default();
    table.clothing_sub_pal_effects.insert(
        9,
        CloSubPalEffect {
            icon: QualifiedDataId::new(0x06000040),
            clo_sub_palettes: vec![CloSubPalette {
                ranges: vec![CloSubPaletteRange {
                    offset: 2,
                    num_colors: 3,
                }],
                palette_set: QualifiedDataId::new(0x0F000050),
            }],
        },
    );

    let mut bytes = vec![0u8; 2048];
    let mut writer = DatBinWriter::new(&mut bytes);
    assert!(table.pack(&mut writer));
    let used = writer.offset();

    let mut unpacked = ClothingTable::default();
    assert!(unpacked.unpack(&mut DatBinReader::new(&bytes[..used])));
    assert_eq!(1, unpacked.clothing_base_effects.len());
    assert_eq!(
        0x01000020,
        unpacked
            .clothing_base_effects
            .get(&QualifiedDataId::new(0x02000010))
            .unwrap()
            .clo_object_effects[0]
            .model_id
            .data_id
    );
    assert_eq!(
        0x05000031,
        unpacked
            .clothing_base_effects
            .get(&QualifiedDataId::new(0x02000010))
            .unwrap()
            .clo_object_effects[0]
            .clo_texture_effects[0]
            .new_texture
            .data_id
    );
    assert_eq!(
        0x0F000050,
        unpacked
            .clothing_sub_pal_effects
            .get(&9)
            .unwrap()
            .clo_sub_palettes[0]
            .palette_set
            .data_id
    );
}

#[test]
fn combat_table_roundtrip_reads_maneuvers() {
    use dat_reader_writer::{
        DBObjs::CombatTable::CombatTable,
        Generated::Enums::{
            AttackHeight::AttackHeight, AttackType::AttackType, MotionStance::MotionStance,
        },
        Types::CombatManeuver::CombatManeuver,
    };

    let table = CombatTable {
        combat_maneuvers: vec![CombatManeuver {
            style: MotionStance::SWORD_COMBAT,
            attack_height: AttackHeight::HIGH,
            attack_type: AttackType::Slash | AttackType::DoubleSlash,
            min_skill_level: 250,
            motion: MotionCommand(0x12345678),
        }],
        ..Default::default()
    };

    let mut bytes = vec![0u8; 512];
    let mut writer = DatBinWriter::new(&mut bytes);
    assert!(table.pack(&mut writer));
    let used = writer.offset();

    let mut unpacked = CombatTable::default();
    assert!(unpacked.unpack(&mut DatBinReader::new(&bytes[..used])));
    assert_eq!(1, unpacked.combat_maneuvers.len());
    assert_eq!(
        MotionStance::SWORD_COMBAT,
        unpacked.combat_maneuvers[0].style
    );
    assert!(
        unpacked.combat_maneuvers[0]
            .attack_type
            .contains(AttackType::Slash)
    );
    assert_eq!(250, unpacked.combat_maneuvers[0].min_skill_level);
}

#[test]
fn vital_table_roundtrip_reads_formulas() {
    use dat_reader_writer::{
        DBObjs::VitalTable::VitalTable, Generated::Enums::AttributeId::AttributeId,
        Types::SkillFormula::SkillFormula,
    };

    let table = VitalTable {
        health: SkillFormula {
            additive_bonus: 10,
            attribute1_multiplier: 1,
            attribute2_multiplier: 0,
            divisor: 2,
            attribute1: AttributeId::STRENGTH,
            attribute2: AttributeId::ENDURANCE,
        },
        stamina: SkillFormula {
            additive_bonus: 20,
            attribute1_multiplier: 1,
            attribute2_multiplier: 1,
            divisor: 3,
            attribute1: AttributeId::ENDURANCE,
            attribute2: AttributeId::SELF,
        },
        mana: SkillFormula {
            additive_bonus: 30,
            attribute1_multiplier: 0,
            attribute2_multiplier: 1,
            divisor: 4,
            attribute1: AttributeId::FOCUS,
            attribute2: AttributeId::SELF,
        },
        ..Default::default()
    };

    let mut bytes = vec![0u8; 256];
    let mut writer = DatBinWriter::new(&mut bytes);
    assert!(table.pack(&mut writer));
    let used = writer.offset();

    let mut unpacked = VitalTable::default();
    assert!(unpacked.unpack(&mut DatBinReader::new(&bytes[..used])));
    assert_eq!(10, unpacked.health.additive_bonus);
    assert_eq!(AttributeId::STRENGTH, unpacked.health.attribute1);
    assert_eq!(AttributeId::SELF, unpacked.stamina.attribute2);
    assert_eq!(4, unpacked.mana.divisor);
}

#[test]
fn skill_table_roundtrip_reads_skill_entries() {
    use dat_reader_writer::{
        DBObjs::RenderSurface::RenderSurface,
        DBObjs::SkillTable::SkillTable,
        Generated::Enums::{
            AttributeId::AttributeId, SkillCategory::SkillCategory, SkillId::SkillId,
        },
        Types::{
            PackableHashTable::PackableHashTable, QualifiedDataId::QualifiedDataId,
            SkillBase::SkillBase, SkillFormula::SkillFormula,
        },
    };

    let mut skills = PackableHashTable::<SkillId, SkillBase>::default();
    skills.insert(
        SkillId::BOW,
        SkillBase {
            description: AC1LegacyString {
                value: "Bow skill".to_string(),
            },
            name: AC1LegacyString {
                value: "Bow".to_string(),
            },
            icon_id: QualifiedDataId::<RenderSurface>::new(0x06000044),
            trained_cost: 6,
            specialized_cost: 4,
            category: SkillCategory::COMBAT,
            chargen_use: true,
            min_level: 7,
            formula: SkillFormula {
                additive_bonus: 1,
                attribute1_multiplier: 1,
                attribute2_multiplier: 1,
                divisor: 3,
                attribute1: AttributeId::COORDINATION,
                attribute2: AttributeId::QUICKNESS,
            },
            upper_bound: 400.0,
            lower_bound: 5.0,
            learn_mod: 1.25,
        },
    );

    let table = SkillTable {
        skills,
        ..Default::default()
    };

    let mut bytes = vec![0u8; 1024];
    let mut writer = DatBinWriter::new(&mut bytes);
    assert!(table.pack(&mut writer));
    let used = writer.offset();

    let mut unpacked = SkillTable::default();
    assert!(unpacked.unpack(&mut DatBinReader::new(&bytes[..used])));
    let bow = unpacked.skills.get(&SkillId::BOW).unwrap();
    assert_eq!("Bow", bow.name.value);
    assert_eq!(SkillCategory::COMBAT, bow.category);
    assert!(bow.chargen_use);
    assert_eq!(AttributeId::COORDINATION, bow.formula.attribute1);
    assert_eq!(0x06000044, bow.icon_id.data_id);
}

#[test]
fn experience_table_roundtrip_reads_progression_arrays() {
    use dat_reader_writer::DBObjs::ExperienceTable::ExperienceTable;

    let table = ExperienceTable {
        attributes: vec![0, 10, 20, 30],
        vitals: vec![0, 40, 50],
        trained_skills: vec![0, 60, 70],
        specialized_skills: vec![0, 80, 90],
        levels: vec![0, 100, 200, 300],
        skill_credits: vec![0, 6, 8, 10],
        ..Default::default()
    };

    let mut bytes = vec![0u8; 256];
    let mut writer = DatBinWriter::new(&mut bytes);
    assert!(table.pack(&mut writer));
    let used = writer.offset();

    let mut unpacked = ExperienceTable::default();
    assert!(unpacked.unpack(&mut DatBinReader::new(&bytes[..used])));
    assert_eq!(vec![0, 10, 20, 30], unpacked.attributes);
    assert_eq!(vec![0, 40, 50], unpacked.vitals);
    assert_eq!(vec![0, 80, 90], unpacked.specialized_skills);
    assert_eq!(vec![0, 100, 200, 300], unpacked.levels);
    assert_eq!(vec![0, 6, 8, 10], unpacked.skill_credits);
}

#[test]
fn string_info_roundtrip_reads_override_and_table_link() {
    use dat_reader_writer::{
        Generated::Enums::StringInfoOverrideFlag::StringInfoOverrideFlag,
        Types::StringInfo::StringInfo,
    };

    let value = StringInfo {
        token: 7,
        string_id: 0x52BA517,
        table_id: QualifiedDataId::new(0x23000001),
        override_flag: StringInfoOverrideFlag::Literal | StringInfoOverrideFlag::AutoGen,
        english: 1,
        comment: 2,
    };

    let mut bytes = vec![0u8; 32];
    let mut writer = DatBinWriter::new(&mut bytes);
    assert!(value.pack(&mut writer));
    let used = writer.offset();

    let mut unpacked = StringInfo::default();
    assert!(unpacked.unpack(&mut DatBinReader::new(&bytes[..used])));
    assert_eq!(7, unpacked.token);
    assert_eq!(0x52BA517, unpacked.string_id);
    assert_eq!(0x23000001, unpacked.table_id.data_id);
    assert!(
        unpacked
            .override_flag
            .contains(StringInfoOverrideFlag::Literal)
    );
    assert!(
        unpacked
            .override_flag
            .contains(StringInfoOverrideFlag::AutoGen)
    );
}

#[test]
fn string_base_helpers_match_expected_equality_and_hash_behavior() {
    use dat_reader_writer::Types::StringBase::StringBase;

    let packed = PStringBase::<u8>::from("Portal");
    let legacy = AC1LegacyString {
        value: "Portal".to_string(),
    };
    let empty = PStringBase::<u8>::from("");

    assert!(packed.equals_string("Portal"));
    assert!(legacy.equals_string("Portal"));
    assert_eq!(packed.ac_string_hash(), legacy.ac_string_hash());
    assert_eq!(0, empty.ac_string_hash());
}

#[test]
fn string_info_base_property_roundtrip_reads_wrapper_payload() {
    use dat_reader_writer::{
        Generated::Enums::StringInfoOverrideFlag::StringInfoOverrideFlag,
        Types::{
            BaseProperty::{BaseProperty, BasePropertyHeader},
            StringInfo::StringInfo,
            StringInfoBaseProperty::StringInfoBaseProperty,
        },
    };

    let property = StringInfoBaseProperty {
        header: BasePropertyHeader {
            master_property_id: 0x99,
            should_pack_master_property_id: true,
        },
        value: StringInfo {
            token: 12,
            string_id: 0x0102_0304,
            table_id: QualifiedDataId::new(0x23000001),
            override_flag: StringInfoOverrideFlag::Literal,
            english: 7,
            comment: 8,
        },
    };

    let mut bytes = vec![0u8; 64];
    let mut writer = DatBinWriter::new(&mut bytes);
    assert!(property.pack(&mut writer));
    let used = writer.offset();

    let mut unpacked = StringInfoBaseProperty {
        header: BasePropertyHeader {
            master_property_id: property.header.master_property_id,
            should_pack_master_property_id: true,
        },
        ..Default::default()
    };
    let mut reader = DatBinReader::new(&bytes[4..used]);
    assert!(unpacked.unpack(&mut reader));
    assert_eq!(12, unpacked.value.token);
    assert_eq!(0x0102_0304, unpacked.value.string_id);
    assert_eq!(0x23000001, unpacked.value.table_id.data_id);
    assert_eq!(
        BaseProperty::StringInfo {
            header: property.header.clone(),
            value: property.value.clone(),
        },
        unpacked.as_base_property()
    );
}

#[test]
fn media_desc_roundtrip_reads_multiple_variants() {
    use dat_reader_writer::{
        Generated::Enums::{
            DrawModeType::DrawModeType, MediaType::MediaType, UIStateId::UIStateId,
        },
        Types::{
            MediaDesc::MediaDesc, MediaDescAnimation::MediaDescAnimation,
            MediaDescImage::MediaDescImage, MediaDescMovie::MediaDescMovie,
            MediaDescSound::MediaDescSound, MediaDescState::MediaDescState,
        },
    };

    let cases = vec![
        MediaDesc::Movie(MediaDescMovie {
            ty: MediaType::Stretch,
            file_name: PStringBase::from("intro.bik"),
            stretch_to_full_screen: true,
        }),
        MediaDesc::Image(MediaDescImage {
            ty: MediaType::Image,
            file: 0x06000011,
            draw_mode: DrawModeType::Overlay,
        }),
        MediaDesc::Animation(MediaDescAnimation {
            ty: MediaType::Animation,
            duration: 1.5,
            draw_mode: DrawModeType::Alphablend,
            frames: vec![10, 20, 30],
        }),
        MediaDesc::Sound(MediaDescSound {
            ty: MediaType::Sound,
            file: 0x0A000010,
            sound: Sound::AMBIENT1,
        }),
        MediaDesc::State(MediaDescState {
            ty: MediaType::State,
            state_id: UIStateId::Active,
            probability: 0.75,
        }),
    ];

    for value in cases {
        let mut bytes = vec![0u8; 256];
        let mut writer = DatBinWriter::new(&mut bytes);
        assert!(value.pack(&mut writer));
        let used = writer.offset();

        let mut unpacked = MediaDesc::default();
        assert!(unpacked.unpack(&mut DatBinReader::new(&bytes[..used])));
        assert_eq!(value, unpacked);
    }
}

#[test]
fn base_property_roundtrip_reads_string_info_and_scalar_variants() {
    use dat_reader_writer::{
        Generated::Enums::BasePropertyType::BasePropertyType,
        Lib::IO::Numerics::Vector3,
        Types::{
            BaseProperty::{BaseProperty, BasePropertyHeader},
            ColorARGB::ColorARGB,
            StringInfo::StringInfo,
        },
    };

    let cases = vec![
        BaseProperty::Integer {
            header: BasePropertyHeader {
                master_property_id: 10,
                should_pack_master_property_id: true,
            },
            value: 42,
        },
        BaseProperty::Bool {
            header: BasePropertyHeader::default(),
            value: true,
        },
        BaseProperty::Float {
            header: BasePropertyHeader::default(),
            value: 3.5,
        },
        BaseProperty::Vector {
            header: BasePropertyHeader::default(),
            value: Vector3::new(1.0, 2.0, 3.0),
        },
        BaseProperty::Color {
            header: BasePropertyHeader::default(),
            value: ColorARGB {
                blue: 1,
                green: 2,
                red: 3,
                alpha: 4,
            },
        },
        BaseProperty::StringInfo {
            header: BasePropertyHeader::default(),
            value: StringInfo {
                token: 9,
                string_id: 0x100,
                table_id: QualifiedDataId::new(0x23000001),
                override_flag: dat_reader_writer::Generated::Enums::StringInfoOverrideFlag::StringInfoOverrideFlag::Literal,
                english: 1,
                comment: 0,
            },
        },
        BaseProperty::Bitfield64 {
            header: BasePropertyHeader::default(),
            value: 0x1122334455667788,
        },
    ];

    for value in cases {
        let property_type = value.property_type();
        let mut bytes = vec![0u8; 128];
        let mut writer = DatBinWriter::new(&mut bytes);
        assert!(value.pack(&mut writer));
        let used = writer.offset();

        let start = match &value {
            BaseProperty::Integer { header, .. }
            | BaseProperty::Bool { header, .. }
            | BaseProperty::LongInteger { header, .. }
            | BaseProperty::Float { header, .. }
            | BaseProperty::Vector { header, .. }
            | BaseProperty::Color { header, .. }
            | BaseProperty::StringInfo { header, .. }
            | BaseProperty::String { header, .. }
            | BaseProperty::Enum { header, .. }
            | BaseProperty::DataId { header, .. }
            | BaseProperty::InstanceId { header, .. }
            | BaseProperty::Bitfield32 { header, .. }
            | BaseProperty::Bitfield64 { header, .. }
            | BaseProperty::Array { header, .. }
            | BaseProperty::Struct { header, .. }
            | BaseProperty::Waveform { header, .. }
            | BaseProperty::Position { header, .. }
            | BaseProperty::TimeStamp { header, .. }
            | BaseProperty::StringToken { header, .. }
            | BaseProperty::PropertyName { header, .. }
            | BaseProperty::TriState { header, .. } => {
                if header.should_pack_master_property_id {
                    4
                } else {
                    0
                }
            }
        };

        let mut reader = DatBinReader::new(&bytes[start..used]);
        let unpacked =
            BaseProperty::unpack_generic_master_property(&mut reader, property_type).unwrap();
        assert_eq!(property_type, unpacked.property_type());
    }

    assert_eq!(
        BasePropertyType::STRING_INFO,
        BasePropertyType::from(0x8_u32)
    );
}

#[test]
fn base_property_desc_roundtrip_reads_bounds_flags_and_available_properties() {
    use std::collections::BTreeMap;

    use dat_reader_writer::{
        Generated::Enums::{
            BasePropertyType::BasePropertyType, PatchFlags::PatchFlags,
            PropertyCachingType::PropertyCachingType, PropertyDatFileType::PropertyDatFileType,
            PropertyGroupName::PropertyGroupName, PropertyInheritanceType::PropertyInheritanceType,
            PropertyPropagationType::PropertyPropagationType,
        },
        Types::{
            BaseProperty::{BaseProperty, BasePropertyHeader},
            BasePropertyDesc::BasePropertyDesc,
        },
    };

    let mut available_properties = BTreeMap::new();
    available_properties.insert(10, 20);
    available_properties.insert(11, 21);

    let desc = BasePropertyDesc {
        name: 7,
        property_type: BasePropertyType::Integer,
        group: PropertyGroupName::GameUI,
        provider: 8,
        data: 9,
        patch_flags: PatchFlags::EmapperId,
        default_value: Some(BaseProperty::Integer {
            header: BasePropertyHeader::default(),
            value: 100,
        }),
        max_value: Some(BaseProperty::Integer {
            header: BasePropertyHeader::default(),
            value: 200,
        }),
        min_value: Some(BaseProperty::Integer {
            header: BasePropertyHeader::default(),
            value: 50,
        }),
        prediction_timeout: 1.25,
        inheritance_type: PropertyInheritanceType::Either,
        dat_file_type: PropertyDatFileType::SharedData,
        propagation_type: PropertyPropagationType::WorldSharedWithServersAndClients,
        caching_type: PropertyCachingType::Internal,
        required: true,
        read_only: false,
        no_checkpoint: true,
        recorded: false,
        do_not_replay: true,
        absolute_time_stamp: false,
        groupable: true,
        propagate_to_children: true,
        available_properties,
    };

    let mut bytes = vec![0u8; 256];
    let mut writer = DatBinWriter::new(&mut bytes);
    assert!(desc.pack(&mut writer));
    let used = writer.offset();

    let mut unpacked = BasePropertyDesc::default();
    assert!(unpacked.unpack(&mut DatBinReader::new(&bytes[..used])));
    assert_eq!(7, unpacked.name);
    assert_eq!(PropertyGroupName::GameUI, unpacked.group);
    assert_eq!(PatchFlags::EmapperId, unpacked.patch_flags);
    assert_eq!(
        PropertyPropagationType::WorldSharedWithServersAndClients,
        unpacked.propagation_type
    );
    assert_eq!(Some(&20), unpacked.available_properties.get(&10));
    match unpacked.default_value.unwrap() {
        BaseProperty::Integer { value, .. } => assert_eq!(100, value),
        other => panic!("unexpected property variant: {other:?}"),
    }
}

#[test]
fn array_and_struct_base_property_wrappers_roundtrip() {
    use std::{collections::BTreeMap, sync::Arc};

    use dat_reader_writer::Types::{
        ArrayBaseProperty::ArrayBaseProperty,
        BaseProperty::{BaseProperty, BasePropertyHeader},
        StringInfo::StringInfo,
        StructBaseProperty::StructBaseProperty,
    };

    let array = ArrayBaseProperty {
        header: BasePropertyHeader {
            master_property_id: 0x1234,
            should_pack_master_property_id: false,
        },
        value: vec![
            BaseProperty::Integer {
                header: BasePropertyHeader {
                    master_property_id: 10,
                    should_pack_master_property_id: true,
                },
                value: 42,
            },
            BaseProperty::Bool {
                header: BasePropertyHeader {
                    master_property_id: 11,
                    should_pack_master_property_id: true,
                },
                value: true,
            },
        ],
    };

    let mut struct_value = BTreeMap::new();
    struct_value.insert(
        7,
        BaseProperty::Integer {
            header: BasePropertyHeader {
                master_property_id: 12,
                should_pack_master_property_id: true,
            },
            value: 77,
        },
    );
    struct_value.insert(
        8,
        BaseProperty::StringInfo {
            header: BasePropertyHeader {
                master_property_id: 13,
                should_pack_master_property_id: true,
            },
            value: StringInfo {
                token: 5,
                string_id: 0x0102_0304,
                table_id: QualifiedDataId::new(0x2300_0001),
                override_flag: dat_reader_writer::Generated::Enums::StringInfoOverrideFlag::StringInfoOverrideFlag::Literal,
                english: 6,
                comment: 7,
            },
        },
    );
    let structured = StructBaseProperty {
        header: BasePropertyHeader {
            master_property_id: 0x5678,
            should_pack_master_property_id: false,
        },
        value: struct_value,
    };

    let mut array_bytes = vec![0u8; 256];
    let mut array_writer = DatBinWriter::new(&mut array_bytes);
    assert!(array.pack(&mut array_writer));
    let array_used = array_writer.offset();

    let mut struct_bytes = vec![0u8; 256];
    let mut struct_writer = DatBinWriter::new(&mut struct_bytes);
    assert!(structured.pack(&mut struct_writer));
    let struct_used = struct_writer.offset();

    let mut unpacked_array = ArrayBaseProperty {
        header: BasePropertyHeader {
            master_property_id: array.header.master_property_id,
            should_pack_master_property_id: false,
        },
        ..Default::default()
    };
    let mut unpacked_struct = StructBaseProperty {
        header: BasePropertyHeader {
            master_property_id: structured.header.master_property_id,
            should_pack_master_property_id: false,
        },
        ..Default::default()
    };

    let base_property_types = Arc::new(BTreeMap::from([
        (
            10_u32,
            dat_reader_writer::Generated::Enums::BasePropertyType::BasePropertyType::Integer,
        ),
        (
            11_u32,
            dat_reader_writer::Generated::Enums::BasePropertyType::BasePropertyType::Bool,
        ),
        (
            12_u32,
            dat_reader_writer::Generated::Enums::BasePropertyType::BasePropertyType::Integer,
        ),
        (
            13_u32,
            dat_reader_writer::Generated::Enums::BasePropertyType::BasePropertyType::StringInfo,
        ),
    ]));

    assert!(
        unpacked_array.unpack(&mut DatBinReader::with_base_property_types(
            &array_bytes[..array_used],
            Some(base_property_types.clone()),
        ))
    );
    assert!(
        unpacked_struct.unpack(&mut DatBinReader::with_base_property_types(
            &struct_bytes[..struct_used],
            Some(base_property_types),
        ))
    );

    assert_eq!(2, unpacked_array.value.len());
    assert!(matches!(
        unpacked_array.value[0],
        BaseProperty::Integer { value: 42, .. }
    ));
    assert!(matches!(
        unpacked_array.value[1],
        BaseProperty::Bool { value: true, .. }
    ));
    assert_eq!(
        Some(&77),
        unpacked_struct.value.get(&7).and_then(|value| match value {
            BaseProperty::Integer { value, .. } => Some(value),
            _ => None,
        })
    );
    assert!(matches!(
        unpacked_struct.value.get(&8),
        Some(BaseProperty::StringInfo { value, .. }) if value.token == 5 && value.table_id.data_id == 0x2300_0001
    ));
}
