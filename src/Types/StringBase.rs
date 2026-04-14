pub trait StringBase {
    fn value(&self) -> &str;

    fn equals_string(&self, other: &str) -> bool {
        self.value() == other
    }

    fn ac_string_hash(&self) -> i32 {
        string_hash(self.value())
    }
}

pub fn string_hash(value: &str) -> i32 {
    if value.is_empty() {
        return 0;
    }

    let mut hash: u32 = 0;
    for ch in value.chars() {
        hash = ch as u32 + (hash << 4);

        let high_bits = hash & 0xF000_0000;
        if high_bits != 0 {
            hash = (hash ^ (high_bits >> 24)) & 0x0FFF_FFFF;
        }
    }

    if hash == 0xFFFF_FFFF {
        return -2;
    }

    hash as i32
}
