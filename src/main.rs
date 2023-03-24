use std::path;

trait FileMetadata {
    fn exists(&self) -> bool;

    fn is_writeable(&self) -> bool;

    fn is_readable(&self) -> bool;
}

impl FileMetadata for path::Path {

    fn is_readable(&self) -> bool {
        match self.read_dir() {
            Ok(_) => true,
            Err(_) => false
        }

    }

    fn is_writeable(&self) -> bool {
        use std::fs;
        match fs::write(self.to_str().unwrap(), "") {
            Ok(_) => true,
            Err(_) => false
        }
    }

    fn exists(&self) -> bool {
        todo!();
    }
}

fn main() {
    // 
}

#[test]
fn writeable() {
    use std::fs;
    use tempfile;

    let f = tempfile::NamedTempFile::new().unwrap();
    assert!(f.path().is_writeable());

    fs::remove_file(f.path()).unwrap();
}

#[test]
fn read_only() {
    use std::fs;
    use tempfile;

    let f = tempfile::NamedTempFile::new().unwrap();
    let mut perms = fs::metadata(f.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(f.path(), perms).unwrap();
    assert_eq!(f.path().is_writeable(), false);
    let mut perms = fs::metadata(f.path()).unwrap().permissions();
    perms.set_readonly(false);
    fs::set_permissions(f.path(), perms).unwrap();
    fs::remove_file(f.path()).unwrap();
}
