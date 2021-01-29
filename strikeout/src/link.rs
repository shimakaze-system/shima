use crate::{error::Result, transfer};
use std::{
    fs,
    path::{Path, PathBuf},
};

/// from: "a/b/01.text"
/// src: "a"
/// dest : "d"
/// expect result: "d/b/S01E01.text"
pub fn map_to_dest(file: &Path, src: &Path, dest: &Path) -> PathBuf {
    let mut path = PathBuf::from(dest);
    let file_name: PathBuf = file.file_name().expect("Error: file name Not found").into();
    let inner_path = file.strip_prefix(src).expect("Error: find files not in src");
    path.push(inner_path.parent().expect("Error: root file"));
    if let Ok(transfered_file_name) = transfer::trans(&file_name) {
        path.push(transfered_file_name);
    } else {
        // just use original name when can not parse
        path.push(file_name);
    }
    path
}

/// create all dir need
pub fn link_to(from: &Path, dest: &Path) -> Result<()> {
    let dir = dest.parent().expect("Error: root file");
    fs::create_dir_all(dir).ok();
    fs::hard_link(from, dest)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::Path;

    #[test]
    fn map_1() {
        let from = Path::new("a/b/1.text");
        let src = Path::new("a");
        let dest = Path::new("d");
        assert_eq!(map_to_dest(&from, &src, &dest), PathBuf::from("d/b/S01E01.text"));
    }

    #[test]
    fn map_2() {
        let from = Path::new("a/1.mkv");
        let src = Path::new("a");
        let dest = Path::new("d");
        assert_eq!(map_to_dest(&from, &src, &dest), PathBuf::from("d/S01E01.mkv"));
    }

    #[test]
    fn map_3() {
        let from = Path::new("a/sss.mkv");
        let src = Path::new("a");
        let dest = Path::new("d");
        assert_eq!(map_to_dest(&from, &src, &dest), PathBuf::from("d/sss.mkv"));
    }
}
