use std::collections::BTreeMap;

use crate::{
    Generated::Enums::IncorporationFlags::IncorporationFlags,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable,
    },
    Types::{BaseProperty::BaseProperty, MediaDesc::MediaDesc},
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct StateDesc {
    pub state_id: u32,
    pub pass_to_children: bool,
    pub incorporation_flags: IncorporationFlags,
    pub properties: BTreeMap<u32, BaseProperty>,
    pub media: Vec<MediaDesc>,
}

impl IUnpackable for StateDesc {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        self.state_id = reader.read_u32();
        self.pass_to_children = reader.read_bool(1);
        self.incorporation_flags = IncorporationFlags::from_bits_truncate(reader.read_u32());

        let _num_buckets = reader.read_byte();
        let property_count = reader.read_compressed_uint() as usize;
        self.properties.clear();
        for _ in 0..property_count {
            let key = reader.read_u32();
            let Some(value) = BaseProperty::unpack_generic(reader) else {
                return false;
            };
            self.properties.insert(key, value);
        }

        let media_count = reader.read_compressed_uint() as usize;
        self.media.clear();
        for _ in 0..media_count {
            let mut value = MediaDesc::default();
            if !value.unpack(reader) {
                return false;
            }
            self.media.push(value);
        }
        true
    }
}

impl IPackable for StateDesc {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_u32(self.state_id);
        writer.write_bool(self.pass_to_children, 1);
        writer.write_u32(self.incorporation_flags.bits());
        writer.write_byte(0);
        writer.write_compressed_uint(self.properties.len() as u32);
        for (key, value) in &self.properties {
            writer.write_u32(*key);
            writer.write_item(value);
        }
        writer.write_compressed_uint(self.media.len() as u32);
        for media in &self.media {
            writer.write_item(media);
        }
        true
    }
}
