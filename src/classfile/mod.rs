mod bytecode;
mod loader;
mod parser;
mod stream;

// ClassLoader
//     ↓
// ClassPathEntry
//     ↓
// ClassFileStream
//     ↓
// ClassFileParser
//     ↓ bytecode
// ClassFile

pub fn as_u16_be(v: &[u8], offset: usize) -> u16 {
    let mut ret = (v[offset] as u16) << 8;
    ret += (v[offset + 1] as u16) << 0;
    ret
}

pub fn as_u32_be(v: &[u8], offset: usize) -> u32 {
    let mut ret = (v[offset] as u32) << 24;
    ret += (v[offset + 1] as u32) << 16;
    ret += (v[offset + 2] as u32) << 8;
    ret += (v[offset + 3] as u32) << 0;
    ret
}

pub fn as_u64_be(v: &[u8], offset: usize) -> u64 {
    let mut ret = (v[offset] as u64) << 56;
    ret += (v[offset + 1] as u64) << 48;
    ret += (v[offset + 2] as u64) << 40;
    ret += (v[offset + 3] as u64) << 32;
    ret += (v[offset + 4] as u64) << 24;
    ret += (v[offset + 5] as u64) << 16;
    ret += (v[offset + 6] as u64) << 8;
    ret += (v[offset + 7] as u64) << 0;
    ret
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_as_u16_be() {
        let v: Vec<u8> = vec![0x43, 0x75];
        let num = as_u16_be(&v, 0);
        assert_eq!(0x4375, num);
    }

    #[test]
    fn test_as_u32_be() {
        let v: Vec<u8> = vec![0x43, 0x75, 0x72, 0x44];
        let num = as_u32_be(&v, 0);
        assert_eq!(0x43757244, num);
    }

    #[test]
    fn test_as_u64_be() {
        let v: Vec<u8> = vec![0x43, 0x75, 0x72, 0x44, 0x00, 0x00, 0x00, 0x34];
        let num = as_u64_be(&v, 0);
        assert_eq!(0x4375724400000034, num);
    }
}
