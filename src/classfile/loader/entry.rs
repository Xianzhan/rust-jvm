pub trait ClassPathEntry {
    fn name(&self) -> String;

    // ClassFileStream* open_stream(const char* name, TRAPS);
    fn open_stream(&self, name: &str);
}

#[derive(Debug)]
pub struct ClassPathDirEntry {
    pub name: String,
}

impl ClassPathEntry for ClassPathDirEntry {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn open_stream(&self, _name: &str) {
        todo!()
    }
}

#[derive(Debug)]
pub struct ClassPathZipEntry {
    pub name: String,
}

impl ClassPathEntry for ClassPathZipEntry {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn open_stream(&self, _name: &str) {
        todo!()
    }
}
