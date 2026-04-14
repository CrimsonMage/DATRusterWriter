use std::any::TypeId;

use encoding_rs::WINDOWS_1252;
use uuid::Uuid;

use crate::Lib::IO::{
    IPackable::IPackable,
    Numerics::{Plane, Quaternion, Vector3},
};

pub struct DatBinWriter<'a> {
    data: &'a mut [u8],
    offset: usize,
}

impl<'a> DatBinWriter<'a> {
    pub fn new(data: &'a mut [u8]) -> Self {
        Self { data, offset: 0 }
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn align(&mut self, value: usize) {
        let align_delta = self.offset % value;
        if align_delta > 0 {
            self.skip(value - align_delta);
        }
    }

    pub fn skip(&mut self, num_bytes: usize) {
        self.offset += num_bytes;
    }

    pub fn write_item<T>(&mut self, item: &T)
    where
        T: IPackable,
    {
        let _ = item.pack(self);
    }

    pub fn write_byte(&mut self, value: u8) {
        self.get_span_and_advance_offset(1)[0] = value;
    }

    pub fn write_sbyte(&mut self, value: i8) {
        self.write_byte(value as u8);
    }

    pub fn write_bytes(&mut self, buffer: &[u8], num_bytes: usize) {
        self.get_span_and_advance_offset(num_bytes)
            .copy_from_slice(&buffer[..num_bytes]);
    }

    pub fn write_bool(&mut self, value: bool, size: usize) {
        match size {
            8 => self.write_u64(if value { 1 } else { 0 }),
            4 => self.write_u32(if value { 1 } else { 0 }),
            2 => self.write_u16(if value { 1 } else { 0 }),
            1 => self.write_byte(if value { 1 } else { 0 }),
            _ => panic!("Unsupported bool size: {size}"),
        }
    }

    pub fn write_i64(&mut self, value: i64) {
        self.get_span_and_advance_offset(8)
            .copy_from_slice(&value.to_le_bytes());
    }

    pub fn write_u64(&mut self, value: u64) {
        self.get_span_and_advance_offset(8)
            .copy_from_slice(&value.to_le_bytes());
    }

    pub fn write_i32(&mut self, value: i32) {
        self.get_span_and_advance_offset(4)
            .copy_from_slice(&value.to_le_bytes());
    }

    pub fn write_u32(&mut self, value: u32) {
        self.get_span_and_advance_offset(4)
            .copy_from_slice(&value.to_le_bytes());
    }

    pub fn write_i16(&mut self, value: i16) {
        self.get_span_and_advance_offset(2)
            .copy_from_slice(&value.to_le_bytes());
    }

    pub fn write_u16(&mut self, value: u16) {
        self.get_span_and_advance_offset(2)
            .copy_from_slice(&value.to_le_bytes());
    }

    pub fn write_single(&mut self, value: f32) {
        self.get_span_and_advance_offset(4)
            .copy_from_slice(&value.to_le_bytes());
    }

    pub fn write_double(&mut self, value: f64) {
        self.get_span_and_advance_offset(8)
            .copy_from_slice(&value.to_le_bytes());
    }

    pub fn write_vector3(&mut self, vec: Vector3) {
        self.write_single(vec.x);
        self.write_single(vec.y);
        self.write_single(vec.z);
    }

    pub fn write_quaternion(&mut self, quat: Quaternion) {
        self.write_single(quat.w);
        self.write_single(quat.x);
        self.write_single(quat.y);
        self.write_single(quat.z);
    }

    pub fn write_compressed_uint(&mut self, value: u32) {
        if value < 0x80 {
            self.write_byte(value as u8);
        } else if value < 0x4000 {
            self.write_byte(((value >> 8) as u8) | 0x80);
            self.write_byte((value & 0xFF) as u8);
        } else {
            self.write_byte(((value >> 24) as u8) | 0xC0);
            self.write_byte(((value >> 16) & 0xFF) as u8);
            self.write_byte(((value >> 8) & 0xFF) as u8);
            self.write_byte((value & 0xFF) as u8);
        }
    }

    pub fn write_plane(&mut self, value: Plane) {
        self.write_vector3(value.normal);
        self.write_single(value.d);
    }

    pub fn write_data_id_of_known_type(&mut self, data_id: u32, known_type: u32) {
        let offset = data_id - known_type;
        if offset <= 0x7FFF && offset < 0x4000 {
            self.write_u16(offset as u16);
        } else {
            let higher = (((offset >> 16) & 0x3FFF) as u16) | 0x8000;
            let lower = (offset & 0xFFFF) as u16;
            self.write_u16(higher);
            self.write_u16(lower);
        }
    }

    pub fn write_guid(&mut self, value: Uuid) {
        self.write_bytes(value.as_bytes(), 16);
    }

    pub fn write_string16_l_byte(&mut self, value: &str) {
        let (encoded, _, _) = WINDOWS_1252.encode(value);
        self.write_compressed_uint(encoded.len() as u32);
        self.write_bytes(encoded.as_ref(), encoded.len());
    }

    pub fn write_generic<T>(&mut self, value: T)
    where
        T: Copy + 'static,
    {
        let ty = TypeId::of::<T>();

        macro_rules! match_copy {
            ($kind:ty, $write:ident) => {
                if ty == TypeId::of::<$kind>() {
                    let value = unsafe { std::mem::transmute_copy::<T, $kind>(&value) };
                    self.$write(value);
                    return;
                }
            };
        }

        match_copy!(u32, write_u32);
        match_copy!(i32, write_i32);
        match_copy!(u64, write_u64);
        match_copy!(i64, write_i64);
        match_copy!(u16, write_u16);
        match_copy!(i16, write_i16);
        match_copy!(u8, write_byte);
        match_copy!(i8, write_sbyte);
        match_copy!(f32, write_single);
        match_copy!(f64, write_double);
        match_copy!(Uuid, write_guid);

        panic!("Type is not supported by write_generic yet")
    }

    fn get_span_and_advance_offset(&mut self, num_bytes: usize) -> &mut [u8] {
        let start = self.offset;
        let end = start + num_bytes;
        self.offset = end;
        &mut self.data[start..end]
    }
}
