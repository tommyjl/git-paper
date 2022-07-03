use std::fs::{read_dir, ReadDir};
use std::path::{Path, PathBuf};

const IGNORE_DIRS: &[&str] = &["node_modules"];

pub struct FileFinder {
    root: PathBuf,
    ignore_dir: Option<Box<dyn FnMut(&Path) -> bool>>,
}

impl FileFinder {
    pub fn new(directory: PathBuf) -> Self {
        Self {
            root: directory,
            ignore_dir: None,
        }
    }

    pub fn ignore_dir_fn(mut self, func: impl FnMut(&Path) -> bool + 'static) -> Self {
        self.ignore_dir = Some(Box::new(func));
        self
    }
}

impl IntoIterator for FileFinder {
    type Item = PathBuf;

    type IntoIter = Iter;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            dirs: vec![self.root.clone()],
            read_dir: None,
            ignore_dir: self.ignore_dir,
        }
    }
}

pub struct Iter {
    dirs: Vec<PathBuf>,
    read_dir: Option<ReadDir>,
    ignore_dir: Option<Box<dyn FnMut(&Path) -> bool>>,
}

impl Iterator for Iter {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.read_dir.is_none() && !self.read_next_dir() {
                return None;
            }

            if let Some(ref mut dir) = self.read_dir {
                for entry in dir.by_ref() {
                    let entry = entry.unwrap();
                    let file_name = entry.file_name().into_string().unwrap();
                    let metadata = entry.metadata().unwrap();
                    let path = entry.path();

                    if !metadata.is_dir() || IGNORE_DIRS.contains(&file_name.as_str()) {
                        continue;
                    }

                    if !self.ignore_dir.as_mut().map_or(false, |f| f(&path)) {
                        self.dirs.push(path.clone());
                    }
                    return Some(path);
                }

                self.read_dir = None;
                continue;
            } else {
                return None;
            }
        }
    }
}

impl Iter {
    fn read_next_dir(&mut self) -> bool {
        if let Some(dir) = self.dirs.pop() {
            self.read_dir = Some(read_dir(dir).unwrap());
            true
        } else {
            false
        }
    }
}
