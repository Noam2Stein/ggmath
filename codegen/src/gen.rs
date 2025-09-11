use std::collections::HashMap;

use indoc::formatdoc;

use crate::gen_mod::{GenModDir, GenModFile};

#[derive(Debug, Clone)]
pub struct GenFn {
    content: String,
    test_content: String,
}

#[derive(Debug, Clone)]
pub struct GenImpl {
    content: String,
    test_content: String,
}

#[derive(Debug, Clone)]
pub struct GenTypeMod {
    name: String,
    declaration: String,
    api_base_impls: Vec<GenImpl>,
    api_ext_impls: Vec<GenImpl>,
    api_primitive_impls: HashMap<String, Vec<GenImpl>>,
    file_submods: Vec<GenModFile>,
    dir_submods: Vec<GenModDir>,
}

impl GenFn {
    pub fn new(content: impl Into<String>, test_content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            test_content: test_content.into(),
        }
    }
}

impl GenImpl {
    pub fn new(sig: impl Into<String>, fns: Vec<GenFn>) -> Self {
        let sig = sig.into();

        let src_fns = fns
            .iter()
            .map(|fn_| fn_.content.clone())
            .collect::<Vec<_>>()
            .join("\n")
            .replace("\n", "\n\t");

        let test_fns = fns
            .iter()
            .map(|fn_| fn_.test_content.clone())
            .collect::<Vec<_>>()
            .join("\n");

        Self {
            content: formatdoc! {r#"
                {sig} {{
                    {src_fns}
                }}
            "#},
            test_content: formatdoc! {r#"
                {test_fns}
            "#},
        }
    }
}

impl GenTypeMod {
    pub fn new(
        name: impl Into<String>,
        declaration: impl Into<String>,
        api_base_impls: Vec<GenImpl>,
        api_ext_impls: Vec<GenImpl>,
        api_primitive_impls: HashMap<String, Vec<GenImpl>>,
        file_submods: Vec<GenModFile>,
        dir_submods: Vec<GenModDir>,
    ) -> Self {
        Self {
            name: name.into(),
            declaration: declaration.into(),
            api_base_impls,
            api_ext_impls,
            api_primitive_impls,
            file_submods,
            dir_submods,
        }
    }

    pub fn resolve(self) -> GenModDir {
        GenModDir::new(
            self.name,
            formatdoc! {r#"
        
            "#},
            formatdoc! {r#"
            
            "#},
            self.file_submods.into_iter().chain([self.]).collect(),
            self.dir_submods,
        )
    }
}
