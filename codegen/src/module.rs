use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

use genco::lang::rust::Tokens;

const WORKPLACE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/..");

pub trait TokensExt {
    fn to_mod_file(self, name: impl Into<String>) -> ModFile;
    fn to_mod_dir(self, name: impl Into<String>) -> ModDir;
    fn to_test_file(self, local_path: impl Into<PathBuf>) -> TestFile;
}

#[derive(Debug, Clone)]
pub struct ModFile {
    name: String,
    content: ModContent,
    test_files: Vec<TestFile>,
}

#[derive(Debug, Clone)]
pub struct ModDir {
    name: String,
    content: ModContent,
    submod_files: Vec<ModFile>,
    submod_dirs: Vec<ModDir>,
    test_files: Vec<TestFile>,
}

#[derive(Debug, Clone)]
pub struct TestFile {
    local_path: PathBuf,
    content: ModContent,
}

#[derive(Debug, Clone)]
struct ModContent {
    content: Tokens,
}

impl TokensExt for Tokens {
    fn to_mod_file(self, name: impl Into<String>) -> ModFile {
        ModFile {
            name: name.into(),
            content: ModContent { content: self },
            test_files: vec![],
        }
    }

    fn to_mod_dir(self, name: impl Into<String>) -> ModDir {
        ModDir {
            name: name.into(),
            content: ModContent { content: self },
            submod_files: vec![],
            submod_dirs: vec![],
            test_files: vec![],
        }
    }

    fn to_test_file(self, local_path: impl Into<PathBuf>) -> TestFile {
        TestFile {
            local_path: local_path.into().with_extension("rs"),
            content: ModContent { content: self },
        }
    }
}

impl ModFile {
    #[allow(dead_code)]
    pub fn with_test_file(mut self, test_file: TestFile) -> Self {
        self.test_files.push(test_file);
        self
    }

    #[allow(dead_code)]
    pub fn with_test_files(mut self, test_files: impl IntoIterator<Item = TestFile>) -> Self {
        self.test_files.extend(test_files);
        self
    }

    fn write(self, path: impl AsRef<Path>) {
        self.content.write_in_src(path);

        for test_file in self.test_files {
            test_file.write();
        }
    }
}

impl ModDir {
    #[allow(dead_code)]
    pub fn with_submod_file(mut self, submod_file: ModFile) -> Self {
        self.submod_files.push(submod_file);
        self
    }

    #[allow(dead_code)]
    pub fn with_submod_files(mut self, submod_files: impl IntoIterator<Item = ModFile>) -> Self {
        self.submod_files.extend(submod_files);
        self
    }

    #[allow(dead_code)]
    pub fn with_submod_dir(mut self, submod_dir: ModDir) -> Self {
        self.submod_dirs.push(submod_dir);
        self
    }

    #[allow(dead_code)]
    pub fn with_submod_dirs(mut self, submod_dirs: impl IntoIterator<Item = ModDir>) -> Self {
        self.submod_dirs.extend(submod_dirs);
        self
    }

    #[allow(dead_code)]
    pub fn with_test_file(mut self, test_file: TestFile) -> Self {
        self.test_files.push(test_file);
        self
    }

    #[allow(dead_code)]
    pub fn with_test_files(mut self, test_files: impl IntoIterator<Item = TestFile>) -> Self {
        self.test_files.extend(test_files);
        self
    }

    pub fn write_as_root(self) {
        let src_dir_path = Path::new(WORKPLACE_DIR).join("src");
        let tests_dir_path = Path::new(WORKPLACE_DIR).join("tests");

        assert!(src_dir_path.ends_with("src"));
        std::fs::remove_dir_all(&src_dir_path).unwrap();

        assert!(tests_dir_path.ends_with("tests"));
        std::fs::remove_dir_all(&tests_dir_path).unwrap();

        let librs_path = src_dir_path.join("lib.rs");
        self.content.write_in_src(&librs_path);

        for test_file in self.test_files {
            test_file.write();
        }

        for submod_file in self.submod_files {
            let submod_path = src_dir_path.join(&submod_file.name).with_extension("rs");
            submod_file.write(submod_path);
        }

        for submod_dir in self.submod_dirs {
            let submod_dir_path = src_dir_path.join(&submod_dir.name);
            submod_dir.write(submod_dir_path);
        }

        Command::new("cargo")
            .current_dir(WORKPLACE_DIR)
            .arg("fmt")
            .arg("-p")
            .arg("ggmath")
            .status()
            .unwrap();
    }

    fn write(self, dir_path: impl AsRef<Path>) {
        let dir_path = dir_path.as_ref();

        let modrs_path = dir_path.join("mod.rs");
        self.content.write_in_src(&modrs_path);

        for test_file in self.test_files {
            test_file.write();
        }

        for submod_file in self.submod_files {
            let submod_path = dir_path.join(&submod_file.name).with_extension("rs");
            submod_file.write(submod_path);
        }

        for submod_dir in self.submod_dirs {
            let submod_dir_path = dir_path.join(&submod_dir.name);
            submod_dir.write(submod_dir_path);
        }
    }
}

impl TestFile {
    fn write(self) {
        self.content.write_in_tests(&self.local_path);
    }
}

impl ModContent {
    fn write_in_src(self, local_path: impl AsRef<Path>) {
        let local_path = local_path.as_ref();

        let path = Path::new(WORKPLACE_DIR).join("src").join(&local_path);

        self.write_with_full_path(path);
    }

    fn write_in_tests(self, local_path: impl AsRef<Path>) {
        let local_path = local_path.as_ref();

        let path = Path::new(WORKPLACE_DIR).join("tests").join(&local_path);

        self.write_with_full_path(path);
    }

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
