use crate::config::InvocationMode;
use crate::Config;
use anyhow::{ensure, Context, Result};
use std::fs::{read_dir, DirEntry};
use std::path::Path;

/// represents a view of a folder, with a filtered list of files
/// 
/// supported image formats are hardcoded
pub struct FolderView {
    elements: Vec<DirEntry>,
    cur: usize,
}

impl FolderView {
    pub fn from(config: &Config) -> Result<FolderView> {
        let path_to_folder = match &config.invoked_as {
            InvocationMode::Directory => std::fs::canonicalize(&config.args.path)?,
            InvocationMode::File => std::fs::canonicalize(&config.args.path)?
                .parent()
                .with_context(|| {
                    format!(
                        "Could not find parent directory of file: {:?}",
                        &config.args.path
                    )
                })?
                .to_path_buf(),
        };

        let elems = read_dir(&path_to_folder)
            .with_context(|| format!("Could not read directory: {:?}", path_to_folder))?;

        let supported_files: Vec<DirEntry> = elems
            .filter_map(|elem| {
                elem.ok().filter(|ok_elem| {
                    ok_elem.metadata().map_or(false, |metadata| {
                        metadata.is_file() && FolderView::is_supported_file_format(&ok_elem.path())
                    })
                })
            })
            .collect();

        ensure!(
            !supported_files.is_empty(),
            "No supported image files in the directory."
        );

        let current_elem = match config.invoked_as {
            InvocationMode::Directory => 0,
            InvocationMode::File => supported_files
                .iter()
                .position(|direntry| direntry.path() == Path::new(&config.args.path).as_os_str())
                .unwrap_or(0),
        };

        Ok(FolderView {
            elements: supported_files,
            cur: current_elem,
        })
    }

    fn is_supported_file_format(p: &Path) -> bool {
        let supported_extensions = ["jpeg", "jpg", "png", "webp"];
        p.extension().is_some_and(|ext| {
            ext.to_str()
                .is_some_and(|ext| supported_extensions.contains(&ext.to_lowercase().as_str()))
        })
    }

    pub fn next(&mut self) -> &DirEntry {
        if self.cur == self.elements.len() - 1 {
            self.cur = 0;
        } else {
            self.cur += 1;
        }

        assert!(self.cur < self.elements.len());
        self.elements.get(self.cur).unwrap()
    }

    pub fn prev(&mut self) -> &DirEntry {
        if self.cur == 0 {
            self.cur = self.elements.len() - 1;
        } else {
            self.cur -= 1;
        }
        assert!(self.cur < self.elements.len());
        self.elements.get(self.cur).unwrap()
    }

    pub fn cur(&self) -> &DirEntry {
        self.elements.get(self.cur).unwrap()
    }
}
