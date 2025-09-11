use std::{fs::File, io::Write, path::Path};

use const_format::formatcp;

const WORKPLACE_DIR: &str = formatcp!("{}/..", env!("CARGO_MANIFEST_DIR"));

#[derive(Debug, Clone)]
pub struct GenModFile {
    name: String,
    content: GenModContent,
}

#[derive(Debug, Clone)]
pub struct GenModDir {
    name: String,
    content: GenModContent,
    submod_files: Vec<GenModFile>,
    submod_dirs: Vec<GenModDir>,
}

#[derive(Debug, Clone)]
struct GenModContent {
    content: String,
    test_content: String,
}

impl GenModFile {
    pub fn new(
        name: impl Into<String>,
        content: impl Into<String>,
        test_content: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            content: GenModContent {
                content: content.into(),
                test_content: test_content.into(),
            },
        }
    }

    fn write(self, path: impl AsRef<Path>) {
        self.content.write(path);
    }
}

impl GenModDir {
    pub fn new(
        name: impl Into<String>,
        content: impl Into<String>,
        test_content: impl Into<String>,
        submod_files: Vec<GenModFile>,
        submod_dirs: Vec<GenModDir>,
    ) -> Self {
        Self {
            name: name.into(),
            content: GenModContent {
                content: content.into(),
                test_content: test_content.into(),
            },
            submod_files,
            submod_dirs,
        }
    }

    pub fn write_as_root(self) {
        let dir_path = Path::new(WORKPLACE_DIR);

        let librs_path = dir_path.join("lib.rs");
        self.content.write(&librs_path);

        for submod_file in self.submod_files {
            let submod_path = dir_path.join(&submod_file.name);
            submod_file.write(submod_path);
        }

        for submod_dir in self.submod_dirs {
            let submod_dir_path = dir_path.join(&submod_dir.name);
            submod_dir.write(submod_dir_path);
        }
    }

    fn write(self, dir_path: impl AsRef<Path>) {
        let dir_path = dir_path.as_ref();

        let modrs_path = dir_path.join("mod.rs");
        self.content.write(&modrs_path);

        for submod_file in self.submod_files {
            let submod_path = dir_path.join(&submod_file.name);
            submod_file.write(submod_path);
        }

        for submod_dir in self.submod_dirs {
            let submod_dir_path = dir_path.join(&submod_dir.name);
            submod_dir.write(submod_dir_path);
        }
    }
}

impl GenModContent {
    fn write(self, local_path: impl AsRef<Path>) {
        let local_path = local_path.as_ref();

        if !self.content.is_empty() {
            let path = Path::new(WORKPLACE_DIR).join("src").join(&local_path);

            let mut file = File::create(path).unwrap();

            file.write_all(self.content.as_bytes()).unwrap();
        }

        if !self.test_content.is_empty() {
            let path = Path::new(WORKPLACE_DIR).join("tests").join(&local_path);

            let mut file = File::create(path).unwrap();

            file.write_all(self.test_content.as_bytes()).unwrap();
        }
    }
}
