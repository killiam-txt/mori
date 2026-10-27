use std::collections::HashSet;
use std::fs::{self, Metadata};
use std::os::unix::fs::MetadataExt;
use std::path::Path;

use crate::tree::Node;

struct Scanner {
    hidden: bool,
    device: u64,
    seen: HashSet<(u64, u64)>,
}

pub fn build(path: &Path, hidden: bool) -> Node {
    let Ok(meta) = fs::symlink_metadata(path) else {
        return Node::new(path.to_path_buf(), 0);
    };

    let mut scanner = Scanner {
        hidden,
        device: meta.dev(),
        seen: HashSet::new(),
    };

    let mut root = scanner.walk(path, &meta);
    root.sort();
    root
}

impl Scanner {
    fn walk(&mut self, path: &Path, meta: &Metadata) -> Node {
        let mut node = Node::new(path.to_path_buf(), disk_usage(meta));

        let Ok(entries) = fs::read_dir(path) else {
            return node;
        };

        for entry in entries.flatten() {
            let child = entry.path();

            if !self.hidden && is_hidden(&child) {
                continue;
            }

            let Ok(meta) = entry.metadata() else {
                continue;
            };

            // stay on one filesystem so scanning the root doesn't descend into /proc or mounted drives
            if meta.dev() != self.device {
                continue;
            }

            // symlinks are skipped to avoid loops and double counting
            if meta.file_type().is_symlink() {
                continue;
            }

            if meta.is_dir() {
                node.push(self.walk(&child, &meta));
            } else if self.first_link(&meta) {
                node.push(Node::new(child, disk_usage(&meta)));
            }
        }

        node
    }

    // hard links share an inode, so count it once like du does
    fn first_link(&mut self, meta: &Metadata) -> bool {
        meta.nlink() == 1 || self.seen.insert((meta.dev(), meta.ino()))
    }
}

// blocks * 512 instead of len(): sparse files report a fake size
fn disk_usage(meta: &Metadata) -> u64 {
    meta.blocks() * 512
}

fn is_hidden(path: &Path) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|n| n.starts_with('.'))
}

#[cfg(test)]
mod tests {
    use super::build;
    use std::fs;
    use std::path::PathBuf;

    fn temp_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("mori-{name}-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn hidden_entries_are_skipped() {
        let dir = temp_dir("hidden");
        fs::create_dir_all(dir.join("sub")).unwrap();
        fs::write(dir.join("visible"), "x").unwrap();
        fs::write(dir.join(".secret"), "x").unwrap();

        let without = build(&dir, false);
        let with = build(&dir, true);
        fs::remove_dir_all(&dir).unwrap();

        assert_eq!(without.children.len(), 2);
        assert_eq!(with.children.len(), 3);
    }

    #[test]
    fn hard_links_count_once() {
        let dir = temp_dir("links");
        fs::write(dir.join("a"), vec![0u8; 8192]).unwrap();
        fs::hard_link(dir.join("a"), dir.join("b")).unwrap();

        let root = build(&dir, false);
        fs::remove_dir_all(&dir).unwrap();

        assert_eq!(root.children.len(), 1);
    }
}