use std::{
    fs::{DirBuilder, File},
    io::Write,
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
};

use super::*;

#[derive(Debug)]
pub struct Mod {
    path: PathBuf,
    content: String,
}

#[derive(Debug)]
pub struct ModDir {
    inner: Mod,
}

impl ModDir {
    pub fn lib_rs() -> Self {
        Self {
            inner: Mod {
                path: Path::new(SRC_DIR).join("lib.rs"),
                content: String::new(),
            },
        }
    }

    pub fn submod(&self, name: &str) -> Mod {
        Mod {
            path: self.path.parent().unwrap().join(name).with_extension("rs"),
            content: String::new(),
        }
    }

    pub fn submod_dir(&self, name: &str) -> ModDir {
        Self {
            inner: Mod {
                path: self.path.parent().unwrap().join(name).join("mod.rs"),
                content: String::new(),
            },
        }
    }
}

impl Deref for ModDir {
    type Target = Mod;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for ModDir {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Drop for Mod {
    fn drop(&mut self) {
        DirBuilder::new()
            .recursive(true)
            .create(self.path.parent().unwrap())
            .expect("failed to create module directory");

        let mut file = File::create(&self.path).expect("failed to create module file");

        write!(file, "{}", self.content).expect("failed to write module file");
    }
}

impl std::fmt::Write for Mod {
    fn write_fmt(&mut self, args: std::fmt::Arguments<'_>) -> std::fmt::Result {
        self.content.write_fmt(args)
    }
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.content.write_str(s)
    }
    fn write_char(&mut self, c: char) -> std::fmt::Result {
        self.content.write_char(c)
    }
}
impl std::fmt::Write for ModDir {
    fn write_fmt(&mut self, args: std::fmt::Arguments<'_>) -> std::fmt::Result {
        self.inner.write_fmt(args)
    }
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.inner.write_str(s)
    }
    fn write_char(&mut self, c: char) -> std::fmt::Result {
        self.inner.write_char(c)
    }
}
