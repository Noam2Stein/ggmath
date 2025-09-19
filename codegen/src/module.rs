use std::{fs::File, io::Write, path::Path};

use genco::lang::rust::Tokens;

const WORKPLACE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/..");

pub trait TokensExt {
    fn to_src_file(self, name: impl Into<String>) -> SrcFile;
    fn to_src_dir(self, name: impl Into<String>) -> SrcDir;
    fn to_test_file(self, name: impl Into<String>) -> TestFile;
    fn to_test_dir(self, name: impl Into<String>) -> TestDir;
}

#[derive(Debug, Clone)]
pub struct SrcFile(ModFile);

#[derive(Debug, Clone)]
pub struct SrcDir(ModDir);

#[derive(Debug, Clone)]
pub struct TestFile(ModFile);

#[derive(Debug, Clone)]
pub struct TestDir(ModDir);

#[derive(Debug, Clone)]
struct ModFile {
    name: String,
    content: ModContent,
}

#[derive(Debug, Clone)]
struct ModDir {
    name: String,
    content: ModContent,
    submod_files: Vec<ModFile>,
    submod_dirs: Vec<ModDir>,
}

#[derive(Debug, Clone)]
struct ModContent {
    content: Tokens,
}

impl TokensExt for Tokens {
    fn to_src_file(self, name: impl Into<String>) -> SrcFile {
        SrcFile(ModFile {
            name: name.into(),
            content: ModContent { content: self },
        })
    }

    fn to_src_dir(self, name: impl Into<String>) -> SrcDir {
        SrcDir(ModDir {
            name: name.into(),
            content: ModContent { content: self },
            submod_files: vec![],
            submod_dirs: vec![],
        })
    }

    fn to_test_file(self, name: impl Into<String>) -> TestFile {
        TestFile(ModFile {
            name: name.into(),
            content: ModContent { content: self },
        })
    }

    fn to_test_dir(self, name: impl Into<String>) -> TestDir {
        TestDir(ModDir {
            name: name.into(),
            content: ModContent { content: self },
            submod_files: vec![],
            submod_dirs: vec![],
        })
    }
}

impl SrcDir {
    #[allow(dead_code)]
    pub fn with_submod_file(mut self, submod_file: SrcFile) -> Self {
        self.0.submod_files.push(submod_file.0);
        self
    }

    #[allow(dead_code)]
    pub fn with_submod_files(mut self, submod_files: impl IntoIterator<Item = SrcFile>) -> Self {
        self.0
            .submod_files
            .extend(submod_files.into_iter().map(|file| file.0));
        self
    }

    #[allow(dead_code)]
    pub fn with_submod_dir(mut self, submod_dir: SrcDir) -> Self {
        self.0.submod_dirs.push(submod_dir.0);
        self
    }

    #[allow(dead_code)]
    pub fn with_submod_dirs(mut self, submod_dirs: impl IntoIterator<Item = SrcDir>) -> Self {
        self.0
            .submod_dirs
            .extend(submod_dirs.into_iter().map(|dir| dir.0));
        self
    }

    pub fn write_as_root(self) {
        let root_dir_path = Path::new(WORKPLACE_DIR).join("src");

        assert!(root_dir_path.ends_with("src"));
        std::fs::remove_dir_all(&root_dir_path).unwrap();

        self.0.write(&root_dir_path, "lib");
    }
}

impl TestDir {
    #[allow(dead_code)]
    pub fn with_submod_file(mut self, submod_file: TestFile) -> Self {
        self.0.submod_files.push(submod_file.0);
        self
    }

    #[allow(dead_code)]
    pub fn with_submod_files(mut self, submod_files: impl IntoIterator<Item = TestFile>) -> Self {
        self.0
            .submod_files
            .extend(submod_files.into_iter().map(|file| file.0));
        self
    }

    #[allow(dead_code)]
    pub fn with_submod_dir(mut self, submod_dir: TestDir) -> Self {
        self.0.submod_dirs.push(submod_dir.0);
        self
    }

    #[allow(dead_code)]
    pub fn with_submod_dirs(mut self, submod_dirs: impl IntoIterator<Item = TestDir>) -> Self {
        self.0
            .submod_dirs
            .extend(submod_dirs.into_iter().map(|dir| dir.0));
        self
    }

    pub fn write_as_root(self) {
        let root_dir_path = Path::new(WORKPLACE_DIR).join("tests");

        assert!(root_dir_path.ends_with("tests"));
        std::fs::remove_dir_all(&root_dir_path).unwrap();

        self.0.write(&root_dir_path, "mod");
    }
}

impl ModFile {
    fn write(self, parent_path: &Path) {
        let path = parent_path.join(&self.name).with_extension("rs");
        self.content.write_with_full_path(path);
    }
}

impl ModDir {
    fn write(self, parent_path: &Path, file_name: impl Into<String>) {
        let dir_path = parent_path.join(&self.name);
        let modrs_path = dir_path.join(file_name.into()).with_extension("rs");

        self.content.write_with_full_path(modrs_path);

        for submod_file in self.submod_files {
            submod_file.write(&dir_path);
        }

        for submod_dir in self.submod_dirs {
            submod_dir.write(&dir_path, "mod");
        }
    }
}

impl ModContent {
    fn write_with_full_path(self, full_path: impl AsRef<Path>) {
        if !full_path.as_ref().exists() {
            std::fs::create_dir_all(full_path.as_ref().parent().unwrap()).unwrap();
        }

        let mut file = File::create(full_path).unwrap();

        writeln!(
            file,
            "// This file was generated by an associated codegen crate.\n// To modify this file, modify the source code of the associated codegen crate.\n\n{content}",
            content = self.content.to_file_string().unwrap()
        )
        .unwrap();
    }
}
