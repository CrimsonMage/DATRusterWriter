use std::{any::TypeId, collections::BTreeMap, sync::Arc};

use encoding_rs::WINDOWS_1252;
use uuid::Uuid;

use crate::{
    Generated::Enums::BasePropertyType::BasePropertyType,
    Lib::IO::{
        IUnpackable::IUnpackable,
        Numerics::{Plane, Quaternion, Vector3},
    },
};

pub struct DatBinReader<'a> {
    data: &'a [u8],
    offset: usize,
    base_property_types: Option<Arc<BTreeMap<u32, BasePropertyType>>>,
}

impl<'a> DatBinReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            offset: 0,
            base_property_types: None,
        }
    }

    pub fn with_base_property_types(
        data: &'a [u8],
        base_property_types: Option<Arc<BTreeMap<u32, BasePropertyType>>>,
    ) -> Self {
        Self {
            data,
            offset: 0,
            base_property_types,
        }
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn length(&self) -> usize {
        self.data.len()
    }

    pub fn set_offset(&mut self, offset: usize) {
        assert!(offset <= self.data.len(), "Offset out of bounds");
        self.offset = offset;
    }

    pub fn rewind(&mut self, num_bytes: usize) {
        assert!(
            num_bytes <= self.offset,
            "Cannot rewind before start of buffer"
        );
        self.offset -= num_bytes;
    }

    pub fn align(&mut self, value: usize) {
        let align_delta = self.offset % value;
        if align_delta > 0 {
            self.skip(value - align_delta);
        }
    }

    pub fn skip(&mut self, num_bytes: usize) {
        self.set_offset(self.offset + num_bytes);
    }

    pub fn read_item<T>(&mut self) -> T
    where
        T: IUnpackable + Default,
    {
        let mut item = T::default();
        let _ = item.unpack(self);
        item
    }

    pub fn read_bytes(&mut self, count: usize) -> Vec<u8> {
        self.read_bytes_internal(count).to_vec()
    }

    pub fn read_remaining_bytes(&mut self) -> Vec<u8> {
        let remaining = self.length() - self.offset();
        self.read_bytes(remaining)
    }

    pub fn read_bytes_into(&mut self, destination: &mut [u8]) {
        destination.copy_from_slice(self.read_bytes_internal(destination.len()));
    }

    pub fn read_byte(&mut self) -> u8 {
        self.read_bytes_internal(1)[0]
    }

    pub fn read_sbyte(&mut self) -> i8 {
        self.read_byte() as i8
    }

    pub fn read_bool(&mut self, size: usize) -> bool {
        match size {
            8 => self.read_u64() != 0,
            4 => self.read_u32() != 0,
            2 => self.read_u16() != 0,
            1 => self.read_byte() != 0,
            _ => panic!("Unsupported bool size: {size}"),
        }
    }

    pub fn read_i64(&mut self) -> i64 {
        i64::from_le_bytes(self.read_array())
    }

    pub fn read_u64(&mut self) -> u64 {
        u64::from_le_bytes(self.read_array())
    }

    pub fn read_i32(&mut self) -> i32 {
        i32::from_le_bytes(self.read_array())
    }

    pub fn read_u32(&mut self) -> u32 {
        u32::from_le_bytes(self.read_array())
    }

    pub fn read_i16(&mut self) -> i16 {
        i16::from_le_bytes(self.read_array())
    }

    pub fn read_u16(&mut self) -> u16 {
        u16::from_le_bytes(self.read_array())
    }

    pub fn read_single(&mut self) -> f32 {
        f32::from_le_bytes(self.read_array())
    }

    pub fn read_double(&mut self) -> f64 {
        f64::from_le_bytes(self.read_array())
    }

    pub fn read_vector3(&mut self) -> Vector3 {
        Vector3::new(self.read_single(), self.read_single(), self.read_single())
    }

    pub fn read_quaternion(&mut self) -> Quaternion {
        let w = self.read_single();
        let x = self.read_single();
        let y = self.read_single();
        let z = self.read_single();
        Quaternion::new(x, y, z, w)
    }

    pub fn read_compressed_uint(&mut self) -> u32 {
        let b0 = self.read_byte();
        if (b0 & 0x80) == 0 {
            return b0 as u32;
        }

        let b1 = self.read_byte();
        if (b0 & 0x40) == 0 {
            return (((b0 & 0x7F) as u32) << 8) | b1 as u32;
        }

        let lower = self.read_u16();
        ((((b0 & 0x3F) as u32) << 8) | b1 as u32) << 16 | lower as u32
    }

    pub fn read_plane(&mut self) -> Plane {
        Plane::new(self.read_vector3(), self.read_single())
    }

    pub fn read_data_id_of_known_type(&mut self, known_type: u32) -> u32 {
        let value = self.read_u16();
        if (value & 0x8000) != 0 {
            let lower = self.read_u16();
            let higher = (value as u32 & 0x3FFF) << 16;
            return known_type + (higher | lower as u32);
        }

        known_type + value as u32
    }

    pub fn read_guid(&mut self) -> Uuid {
        Uuid::from_bytes(self.read_array())
    }

    pub fn read_string16_l_byte(&mut self) -> String {
        let length = self.read_compressed_uint() as usize;
        let bytes = self.read_bytes_internal(length);
        let (decoded, _, _) = WINDOWS_1252.decode(bytes);
        decoded.into_owned()
    }

    pub fn read_generic<T>(&mut self) -> T
    where
        T: Copy + 'static,
    {
        let ty = TypeId::of::<T>();

        macro_rules! match_copy {
            ($kind:ty, $value:expr) => {
                if ty == TypeId::of::<$kind>() {
                    let value = $value;
                    return unsafe { std::mem::transmute_copy::<$kind, T>(&value) };
                }
            };
        }

        match_copy!(u32, self.read_u32());
        match_copy!(i32, self.read_i32());
        match_copy!(u64, self.read_u64());
        match_copy!(i64, self.read_i64());
        match_copy!(u16, self.read_u16());
        match_copy!(i16, self.read_i16());
        match_copy!(u8, self.read_byte());
        match_copy!(i8, self.read_sbyte());
        match_copy!(bool, self.read_bool(4));
        match_copy!(f32, self.read_single());
        match_copy!(f64, self.read_double());
        match_copy!(Uuid, self.read_guid());

        panic!("Type is not supported by read_generic yet")
    }

    pub fn base_property_type(&self, property_id: u32) -> Option<BasePropertyType> {
        self.base_property_types
            .as_ref()
            .and_then(|types| types.get(&property_id).copied())
    }

    fn read_bytes_internal(&mut self, count: usize) -> &'a [u8] {
        let start = self.offset;
        let end = start + count;
        self.set_offset(end);
        &self.data[start..end]
    }

    fn read_array<const N: usize>(&mut self) -> [u8; N] {
        let mut bytes = [0_u8; N];
        bytes.copy_from_slice(self.read_bytes_internal(N));
        bytes
    }
}
