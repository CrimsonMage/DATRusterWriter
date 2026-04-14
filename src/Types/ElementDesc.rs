use std::collections::BTreeMap;

use crate::{
    Generated::Enums::{IncorporationFlags::IncorporationFlags, UIStateId::UIStateId},
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::StateDesc::StateDesc,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ElementDesc {
    pub state_desc: StateDesc,
    pub read_order: u32,
    pub element_id: u32,
    pub element_type: u32,
    pub base_element: u32,
    pub base_layout_id: u32,
    pub default_state: UIStateId,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub z_level: u32,
    pub left_edge: u32,
    pub top_edge: u32,
    pub right_edge: u32,
    pub bottom_edge: u32,
    pub states: BTreeMap<UIStateId, StateDesc>,
    pub children: BTreeMap<u32, ElementDesc>,
}

impl IUnpackable for ElementDesc {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.state_desc = reader.read_item::<StateDesc>();
        self.read_order = reader.read_u32();
        self.element_id = reader.read_u32();
        self.element_type = reader.read_u32();
        self.base_element = reader.read_u32();
        self.base_layout_id = reader.read_u32();
        self.default_state = UIStateId::from(reader.read_u32());

        if self
            .state_desc
            .incorporation_flags
            .contains(IncorporationFlags::X)
        {
            self.x = reader.read_u32();
        }
        if self
            .state_desc
            .incorporation_flags
            .contains(IncorporationFlags::Y)
        {
            self.y = reader.read_u32();
        }
        if self
            .state_desc
            .incorporation_flags
            .contains(IncorporationFlags::Width)
        {
            self.width = reader.read_u32();
        }
        if self
            .state_desc
            .incorporation_flags
            .contains(IncorporationFlags::Height)
        {
            self.height = reader.read_u32();
        }
        if self
            .state_desc
            .incorporation_flags
            .contains(IncorporationFlags::ZLevel)
        {
            self.z_level = reader.read_u32();
        }

        self.left_edge = reader.read_u32();
        self.top_edge = reader.read_u32();
        self.right_edge = reader.read_u32();
        self.bottom_edge = reader.read_u32();

        let _states_bucket_size = reader.read_byte();
        let state_count = reader.read_compressed_uint() as usize;
        self.states.clear();
        for _ in 0..state_count {
            let key = UIStateId::from(reader.read_u32());
            let value = reader.read_item::<StateDesc>();
            self.states.insert(key, value);
        }

        let _children_bucket_size = reader.read_byte();
        let child_count = reader.read_compressed_uint() as usize;
        self.children.clear();
        for _ in 0..child_count {
            let key = reader.read_u32();
            let value = reader.read_item::<ElementDesc>();
            self.children.insert(key, value);
        }

        true
    }
}

impl IPackable for ElementDesc {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_item(&self.state_desc);
        writer.write_u32(self.read_order);
        writer.write_u32(self.element_id);
        writer.write_u32(self.element_type);
        writer.write_u32(self.base_element);
        writer.write_u32(self.base_layout_id);
        writer.write_u32(self.default_state.into());

        if self
            .state_desc
            .incorporation_flags
            .contains(IncorporationFlags::X)
        {
            writer.write_u32(self.x);
        }
        if self
            .state_desc
            .incorporation_flags
            .contains(IncorporationFlags::Y)
        {
            writer.write_u32(self.y);
        }
        if self
            .state_desc
            .incorporation_flags
            .contains(IncorporationFlags::Width)
        {
            writer.write_u32(self.width);
        }
        if self
            .state_desc
            .incorporation_flags
            .contains(IncorporationFlags::Height)
        {
            writer.write_u32(self.height);
        }
        if self
            .state_desc
            .incorporation_flags
            .contains(IncorporationFlags::ZLevel)
        {
            writer.write_u32(self.z_level);
        }

        writer.write_u32(self.left_edge);
        writer.write_u32(self.top_edge);
        writer.write_u32(self.right_edge);
        writer.write_u32(self.bottom_edge);

        writer.write_byte(1);
        writer.write_compressed_uint(self.states.len() as u32);
        for (key, value) in &self.states {
            writer.write_u32((*key).into());
            writer.write_item(value);
        }

        writer.write_byte(1);
        writer.write_compressed_uint(self.children.len() as u32);
        for (key, value) in &self.children {
            writer.write_u32(*key);
            writer.write_item(value);
        }
        true
    }
}
