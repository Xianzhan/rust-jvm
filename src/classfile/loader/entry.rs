use std::{fs::File, io::Read, path::Path};

use zip::ZipArchive;

use crate::classfile::stream::ClassFileStream;
use crate::runtime::os;

pub trait ClassPathEntry {
    fn name(&self) -> String;

    // ClassFileStream* open_stream(const char* name, TRAPS);
    fn open_stream(&self, name: &str) -> Option<ClassFileStream>;
}

#[derive(Debug)]
pub struct ClassPathDirEntry {
    pub name: String,
    pub jar_v: Vec<ClassPathZipEntry>,
}

impl ClassPathDirEntry {
    pub fn new(path: &str) -> Self {
        // 找寻 jar 包
        let mut jar_v = vec![];
        let p = Path::new(path);
        let dir_result = p.read_dir();
        let dir = dir_result.unwrap();
        for entry_result in dir {
            let entry = entry_result.unwrap();
            if entry.file_type().unwrap().is_dir() {
                continue;
            }

            let entry_path = entry.path();
            let ext_option = entry_path.extension();
            if ext_option.is_none() {
                continue;
            }
            let ext = ext_option.unwrap();
            let ext_str = ext.to_str().unwrap();
            if ext_str != "jar" && ext_str != "zip" {
                continue;
            }

            let current_path = path.to_string();
            let current_jar =
                current_path + os::file_separator().as_str() + entry.file_name().to_str().unwrap();
            jar_v.push(ClassPathZipEntry::new(&current_jar));
        }

        Self {
            name: path.to_string(),
            jar_v,
        }
    }
}

impl ClassPathEntry for ClassPathDirEntry {
    fn name(&self) -> String {
        self.name.clone()
    }

    /// 在该目录下查找名称为 name 的 class 文件
    /// name: java/lang/Object.class
    fn open_stream(&self, name: &str) -> Option<ClassFileStream> {
        let file_name = self.name() + os::file_separator().as_str() + name;
        let file_result = File::open(file_name);
        match file_result {
            Ok(mut file) => {
                let mut data = vec![];
                let read_result = file.read_to_end(&mut data);
                if let Err(e) = read_result {
                    panic!("ClassPathDirEntry::open_stream read file err: {}", e);
                }

                Some(ClassFileStream::new(data))
            }
            Err(_) => {
                // 尝试在 jar 包读取，如果有 jar 包
                for i in 0..self.jar_v.len() {
                    let stream_option = self.jar_v[i].open_stream(name);
                    if stream_option.is_some() {
                        return stream_option;
                    }
                }
                None
            }
        }
    }
}

#[derive(Debug)]
pub struct ClassPathZipEntry {
    pub name: String,
}

impl ClassPathZipEntry {
    pub fn new(path: &str) -> Self {
        Self {
            name: path.to_string(),
        }
    }
}

impl ClassPathEntry for ClassPathZipEntry {
    fn name(&self) -> String {
        self.name.clone()
    }

    /// name: java/lang/Object.class
    fn open_stream(&self, name: &str) -> Option<ClassFileStream> {
        let jar_name = self.name();
        let jar_file_result = File::open(&jar_name);
        match jar_file_result {
            Err(err) => {
                println!("打开 {} 错误: {}", jar_name, err);
                return None;
            }
            Ok(file) => {
                let mut zip_archive = ZipArchive::new(file).unwrap();
                let len = zip_archive.len();
                for i in 0..len {
                    let mut data = vec![];
                    let mut zip_file = zip_archive.by_index(i).unwrap();
                    let zip_file_name = zip_file.name();
                    // println!("{}", zip_file_name);
                    if zip_file_name != name {
                        continue;
                    }

                    let read_result = zip_file.read_to_end(&mut data);
                    match read_result {
                        Err(err) => {
                            println!("ClassPathZipEntry::open_stream 错误：{}", err);
                            continue;
                        }
                        Ok(buf_usize) => {
                            if buf_usize == 0 {
                                continue;
                            }
                            return Some(ClassFileStream::new(data));
                        }
                    }
                }
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::runtime::os;

    use super::ClassPathDirEntry;
    use super::ClassPathEntry;
    use super::ClassPathZipEntry;

    #[test]
    fn test_class_path_dir_entry_open_stream() {
        if cfg!(windows) {
            let lib = "lib";

            let dir = ClassPathDirEntry::new(&lib);
            let stream = dir.open_stream("java/lang/Object.class");
            if let Some(s) = stream {
                s.print_data();
            }
        }
    }

    #[test]
    fn test_class_path_zip_entry_open_stream() {
        let zip = "lib".to_string() + os::file_separator().as_str() + "rt.jar";
        println!("{}", zip);
        let zip_entry = ClassPathZipEntry::new(&zip);
        let stream = zip_entry.open_stream("java/lang/Object.class");
        if let Some(s) = stream {
            s.print_data();
        }
    }
}
