use crate::classfile::as_u16_be;
use crate::classfile::as_u32_be;
use crate::classfile::as_u64_be;

#[derive(Debug)]
pub struct ClassFileStream {
    pub data: Vec<u8>,
    current_position: usize,
}

impl ClassFileStream {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data,
            current_position: 0,
        }
    }

    pub fn get_u8(&mut self) -> u8 {
        let ret = self.data[self.current_position];
        self.current_position += 1;
        ret
    }

    pub fn get_u16(&mut self) -> u16 {
        let start = self.current_position;
        self.current_position += 2;

        let end = start + 2;
        let v = &self.data[start..end];
        as_u16_be(v, start)
    }

    pub fn get_u32(&mut self) -> u32 {
        let start = self.current_position;
        self.current_position += 4;

        let end = start + 4;
        let v = &self.data[start..end];
        as_u32_be(v, start)
    }

    pub fn get_u64(&mut self) -> u64 {
        let start = self.current_position;
        self.current_position += 8;
        
        let end = start + 8;
        let v = &self.data[start..end];
        as_u64_be(v, start)
    }

    pub fn print_data(&self) {
        let len = self.data.len();
        let row = len / 16;
        for i in 0..=row as usize {
            let start = i * 16;
            let end;
            let temp = i * 16 + 16;
            if temp > len {
                end = len;
            } else {
                end = temp;
            }
            let row_data = &self.data[start..end];
            println!("{:2}: {:?}", i, row_data);
        }
    }
}
