use dat_reader_writer::{
    DBObjs::{GfxObj::GfxObj, MotionTable::MotionTable, Palette::Palette, ParticleEmitter::ParticleEmitter, PhysicsScript::PhysicsScript, Region::Region, RenderSurface::RenderSurface, Scene::Scene, Surface::Surface, SurfaceTexture::SurfaceTexture, Wave::Wave},
    Generated::Enums::{AnimationHookDir::AnimationHookDir, CullMode::CullMode, EmitterType::EmitterType, GfxObjFlags::GfxObjFlags, MotionCommand::MotionCommand, ParticleType::ParticleType, PartsMask::PartsMask, PixelFormat::PixelFormat, Sound::Sound, StipplingType::StipplingType, SurfaceType::SurfaceType, TerrainTextureType::TerrainTextureType, TextureType::TextureType, VertexType::VertexType},
    Lib::IO::{DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable, IUnpackable::IUnpackable},
    Types::{
        AnimationHook::AnimationHook,
        AC1LegacyString::AC1LegacyString,
        AmbientSTBDesc::AmbientSTBDesc,
        AmbientSoundDesc::AmbientSoundDesc,
        AttackCone::AttackCone,
        BSPTrees::{DrawingBSPNode, DrawingBSPTree},
        ColorARGB::ColorARGB,
        Frame::Frame,
        LandDefs::LandDefs,
        LandSurf::LandSurf,
        MotionData::MotionData,
        ObjectDesc::ObjectDesc,
        Polygon::Polygon,
        PhysicsScriptData::PhysicsScriptData,
        PackedQualifiedDataId::PackedQualifiedDataId,
        QualifiedDataId::QualifiedDataId,
        RegionMisc::RegionMisc,
        SceneDesc::SceneDesc,
        SceneType::SceneType,
        SoundDesc::SoundDesc,
        TerrainAlphaMap::TerrainAlphaMap,
        TerrainDesc::TerrainDesc,
        TerrainTex::TerrainTex,
        TerrainType::TerrainType,
        TexMerge::TexMerge,
        TMTerrainDesc::TMTerrainDesc,
        Vec2Duv::Vec2Duv,
        VertexArray::VertexArray,
        SWVertex::SWVertex,
    },
};

#[test]
fn palette_roundtrip_reads_colors() {
    let palette = Palette { colors: vec![ColorARGB { blue: 1, green: 2, red: 3, alpha: 4 }, ColorARGB { blue: 5, green: 6, red: 7, alpha: 8 }], ..Default::default() };
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
    let surface = SurfaceTexture { texture_type: TextureType::TEXTURE2D, textures: vec![QualifiedDataId::new(0x06000001), QualifiedDataId::new(0x06000002)], ..Default::default() };
    let mut bytes = vec![0u8; 4 + 4 + 1 + 4 + 8];
    assert!(surface.pack(&mut DatBinWriter::new(&mut bytes)));
    let mut unpacked = SurfaceTexture::default();
    assert!(unpacked.unpack(&mut DatBinReader::new(&bytes)));
    assert_eq!(TextureType::TEXTURE2D, unpacked.texture_type);
    assert_eq!(0x06000002, unpacked.textures[1].data_id);
}

#[test]
fn render_surface_reads_palette_tail_for_indexed_formats() {
    let render = RenderSurface { width: 64, height: 32, format: PixelFormat::PFID_P8, source_data: vec![1,2,3], default_palette_id: 0x04000001, ..Default::default() };
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
    motion.style_defaults.insert(MotionCommand(1), MotionCommand(2));
    motion.cycles.insert(10, MotionData::default());
    let mut bytes = vec![0u8; 2048];
    assert!(motion.pack(&mut DatBinWriter::new(&mut bytes)));
    let mut unpacked = MotionTable::default();
    assert!(unpacked.unpack(&mut DatBinReader::new(&bytes)));
    assert_eq!(MotionCommand(0x12345678), unpacked.default_style);
    assert_eq!(Some(&MotionCommand(2)), unpacked.style_defaults.get(&MotionCommand(1)));
    assert!(unpacked.cycles.contains_key(&10));
}

#[test]
fn region_reads_sound_scene_terrain_and_misc() {
    let region = Region {
        region_number: 7,
        version: 3,
        region_name: AC1LegacyString { value: "Dereth".to_string() },
        land_defs: LandDefs { land_height_table: vec![0.0; 256], ..Default::default() },
        parts_mask: PartsMask::HasSoundInfo | PartsMask::HasSceneInfo | PartsMask::HasRegionMisc,
        sound_info: Some(SoundDesc {
            stb_desc: vec![AmbientSTBDesc {
                stb_id: 11,
                ambient_sounds: vec![AmbientSoundDesc {
                    s_type: Sound(0x46),
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
                terrain_name: AC1LegacyString { value: "Grass".to_string() },
                terrain_color: ColorARGB { blue: 1, green: 2, red: 3, alpha: 4 },
                scene_types: vec![5, 6],
            }],
            land_surfaces: LandSurf {
                land_type: 9,
                tex_merge: TexMerge {
                    base_tex_size: 32,
                    corner_terrain_maps: vec![TerrainAlphaMap { t_code: 1, texture_id: QualifiedDataId::new(0x05000001) }],
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
    assert_eq!(Sound(0x46), unpacked.sound_info.as_ref().unwrap().stb_desc[0].ambient_sounds[0].s_type);
    assert_eq!(5, unpacked.scene_info.as_ref().unwrap().scene_types[0].stb_index);
    assert_eq!("Grass", unpacked.terrain_info.terrain_types[0].terrain_name.value);
    assert_eq!(32, unpacked.terrain_info.land_surfaces.tex_merge.base_tex_size);
    assert_eq!(TerrainTextureType(1), unpacked.terrain_info.land_surfaces.tex_merge.terrain_desc[0].terrain_type);
    assert_eq!(0x0600127D, unpacked.region_misc.as_ref().unwrap().game_map_id);
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
        vertex_array: VertexArray { vertex_type: VertexType(1), vertices: [(1u16, SWVertex { uvs: vec![Vec2Duv { u: 1.0, v: 2.0 }], ..Default::default() })].into_iter().collect() },
        polygons: [(7u16, Polygon {
            stippling: StipplingType::Positive | StipplingType::NoNeg,
            sides_type: CullMode::CLOCKWISE,
            pos_surface: 1,
            neg_surface: -1,
            vertex_ids: vec![1, 2, 3],
            pos_uv_indices: vec![0, 1, 2],
            neg_uv_indices: vec![],
        })].into_iter().collect(),
        drawing_bsp: DrawingBSPTree { root: DrawingBSPNode::default() },
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
    assert_eq!(Some(&Polygon {
        stippling: StipplingType::Positive | StipplingType::NoNeg,
        sides_type: CullMode::CLOCKWISE,
        pos_surface: 1,
        neg_surface: -1,
        vertex_ids: vec![1, 2, 3],
        pos_uv_indices: vec![0, 1, 2],
        neg_uv_indices: vec![],
    }), unpacked.polygons.get(&7));
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
                    attack_cone: AttackCone { part_index: 3, radius: 4.0, height: 5.0, ..Default::default() },
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
        AnimationHook::Sound { direction: AnimationHookDir::FORWARD, id: QualifiedDataId::new(0x0A000010) },
        unpacked.script_data[0].hook
    );
    assert_eq!(
        AnimationHook::ReplaceObject { direction: AnimationHookDir::BACKWARD, part_index: 7, part_id: PackedQualifiedDataId::new(0x01000022) },
        unpacked.script_data[1].hook
    );
    assert_eq!(
        AnimationHook::CreateParticle { direction: AnimationHookDir::BOTH, emitter_info_id: QualifiedDataId::new(0x32000011), part_index: 4, offset: Frame::default(), emitter_id: 99 },
        unpacked.script_data[2].hook
    );
}
