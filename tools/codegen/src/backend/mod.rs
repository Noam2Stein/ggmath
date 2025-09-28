use std::path::Path;

use genco::lang::rust::Tokens;

const WORKPLACE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");

mod raw;
use raw::*;

pub trait TokensExt {
    fn to_srcfile(self, name: impl Into<String>) -> SrcFile;
    fn to_srcdir(self, name: impl Into<String>) -> SrcDir;
    fn to_testfile(self, name: impl Into<String>) -> TestFile;
    fn to_testdir(self, name: impl Into<String>) -> TestDir;
}

#[derive(Debug, Clone)]
pub struct SrcFile(RawFile);

#[derive(Debug, Clone)]
pub struct SrcDir(RawDir);

#[derive(Debug, Clone)]
pub struct TestFile(RawFile);

#[derive(Debug, Clone)]
pub struct TestDir(RawDir);

impl TokensExt for Tokens {
    fn to_srcfile(self, name: impl Into<String>) -> SrcFile {
        SrcFile(RawFile {
            name: name.into(),
            content: ModContent { tokens: self },
        })
    }

    fn to_srcdir(self, name: impl Into<String>) -> SrcDir {
        SrcDir(RawDir {
            name: name.into(),
            content: ModContent { tokens: self },
            submod_files: vec![],
            submod_dirs: vec![],
        })
    }

    fn to_testfile(self, name: impl Into<String>) -> TestFile {
        TestFile(RawFile {
            name: name.into(),
            content: ModContent { tokens: self },
        })
    }

    fn to_testdir(self, name: impl Into<String>) -> TestDir {
        TestDir(RawDir {
            name: name.into(),
            content: ModContent { tokens: self },
            submod_files: vec![],
            submod_dirs: vec![],
        })
    }
}

impl SrcDir {
    pub fn with_submod_file(mut self, submod_file: SrcFile) -> Self {
        self.0.submod_files.push(submod_file.0);
        self
    }

    #[expect(dead_code)]
    pub fn with_submod_files(mut self, submod_files: impl IntoIterator<Item = SrcFile>) -> Self {
        self.0
            .submod_files
            .extend(submod_files.into_iter().map(|file| file.0));
        self
    }

    pub fn with_submod_dir(mut self, submod_dir: SrcDir) -> Self {
        self.0.submod_dirs.push(submod_dir.0);
        self
    }

    pub fn with_submod_dirs(mut self, submod_dirs: impl IntoIterator<Item = SrcDir>) -> Self {
        self.0
            .submod_dirs
            .extend(submod_dirs.into_iter().map(|dir| dir.0));
        self
    }

    pub fn export_as_root(self) {
        let root_dir_path = Path::new(WORKPLACE_DIR).join("src");

        assert!(root_dir_path.ends_with("src"));
        self.0.export(&root_dir_path, "lib");
    }
}

impl TestDir {
    #[expect(dead_code)]
    pub fn with_submod_file(mut self, submod_file: TestFile) -> Self {
        self.0.submod_files.push(submod_file.0);
        self
    }

    pub fn with_submod_files(mut self, submod_files: impl IntoIterator<Item = TestFile>) -> Self {
        self.0
            .submod_files
            .extend(submod_files.into_iter().map(|file| file.0));
        self
    }

    pub fn with_submod_dir(mut self, submod_dir: TestDir) -> Self {
        self.0.submod_dirs.push(submod_dir.0);
        self
    }

    #[expect(dead_code)]
    pub fn with_submod_dirs(mut self, submod_dirs: impl IntoIterator<Item = TestDir>) -> Self {
        self.0
            .submod_dirs
            .extend(submod_dirs.into_iter().map(|dir| dir.0));
        self
    }

    pub fn export_as_root(self) {
        let root_dir_path = Path::new(WORKPLACE_DIR).join("tests");

        assert!(root_dir_path.ends_with("tests"));
        self.0.export(&root_dir_path, "mod");
    }
}
