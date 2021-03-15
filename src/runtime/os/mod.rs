use std::path;

pub fn file_separator() -> String {
    path::MAIN_SEPARATOR.to_string()
}

#[cfg(test)]
mod tests {

    use super::file_separator;

    #[test]
    fn test_file_separator() {
        if cfg!(windows) {
            assert_eq!("\\", file_separator());
        }
    }
}
