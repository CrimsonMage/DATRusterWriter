use crate::{
    Generated::Enums::MotionDataFlags::MotionDataFlags,
    Lib::IO::{
        DatBinReader::DatBinReader, DatBinWriter::DatBinWriter, IPackable::IPackable,
        IUnpackable::IUnpackable, Numerics::Vector3,
    },
    Types::AnimData::AnimData,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotionData {
    pub bitfield: u8,
    pub flags: MotionDataFlags,
    pub anims: Vec<AnimData>,
    pub velocity: Vector3,
    pub omega: Vector3,
}

impl IUnpackable for MotionData {
    fn unpack(&mut self, reader: &mut DatBinReader<'_>) -> bool {
        let num_anims = reader.read_byte() as usize;
        self.bitfield = reader.read_byte();
        self.flags = MotionDataFlags::from_bits_truncate(reader.read_byte());
        reader.align(4);
        self.anims.clear();
        for _ in 0..num_anims {
            self.anims.push(reader.read_item::<AnimData>());
        }
        if self.flags.contains(MotionDataFlags::HasVelocity) {
            self.velocity = reader.read_vector3();
        }
        if self.flags.contains(MotionDataFlags::HasOmega) {
            self.omega = reader.read_vector3();
        }
        true
    }
}

impl IPackable for MotionData {
    fn pack(&self, writer: &mut DatBinWriter<'_>) -> bool {
        writer.write_byte(self.anims.len() as u8);
        writer.write_byte(self.bitfield);
        writer.write_byte(self.flags.bits());
        writer.align(4);
        for item in &self.anims {
            let _ = item.pack(writer);
        }
        if self.flags.contains(MotionDataFlags::HasVelocity) {
            writer.write_vector3(self.velocity);
        }
        if self.flags.contains(MotionDataFlags::HasOmega) {
            writer.write_vector3(self.omega);
        }
        true
    }
}
