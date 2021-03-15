// src/hotspot/share/classfile/classLoader.hpp
// src/hotspot/share/classfile/classLoader.cpp

mod entry;

use std::path::Path;

use entry::{ClassPathDirEntry, ClassPathEntry, ClassPathZipEntry};

/// 从 path 构建 entry
/// path: "." or "$JAVA_HOME/jre/lib"
fn create_class_path_entry(path: &str) -> Box<dyn ClassPathEntry> {
    let p = Path::new(path);
    if p.is_dir() {
        return Box::new(ClassPathDirEntry::new(path));
    }

    if let Some(ext) = p.extension() {
        if ext == "jar" || ext == "zip" {
            return Box::new(ClassPathZipEntry::new(path));
        }
    }

    panic!("path: {} 错误", path);
}

#[cfg(test)]
mod tests {
    use super::create_class_path_entry;

    #[test]
    fn test_create_class_path_entry() {
        let class_path = create_class_path_entry("a.jar");
        // println!("class path name: {}", class_path.name());
        assert_eq!("a.jar", class_path.name());
    }
}
