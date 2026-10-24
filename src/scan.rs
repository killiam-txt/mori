use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::Path;

use crate::tree::Node;

pub fn build(path: &Path, hidden: bool) -> Node {
    let device = fs::symlink_metadata(path).map(|m| m.dev()).unwrap_or(0);
    let mut root = walk(path, hidden, device);
    root.sort();
    root
}

fn walk(path: &Path, hidden: bool, device: u64) -> Node {
    let mut node = Node::new(path.to_path_buf(), 0);

    let Ok(entries) = fs::read_dir(path) else {
        return node;
    };

    for entry in entries.flatten() {
        let child = entry.path();

        if !hidden && is_hidden(&child) {
            continue;
        }

        let Ok(meta) = entry.metadata() else {
            continue;
        };

        // stay on one filesystem so scanning the root doesn't descend into /proc or mounted drives
        if meta.dev() != device {
            continue;
        }

        // symlinks are skipped to avoid loops and double counting
        if meta.file_type().is_symlink() {
            continue;
        }

        if meta.is_dir() {
            node.push(walk(&child, hidden, device));
        } else {
            // blocks * 512 instead of len(): sparse files report a fake size
            node.push(Node::new(child, meta.blocks() * 512));
        }
    }

    node
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

    #[test]
    fn hidden_entries_are_skipped() {
        let dir = std::env::temp_dir().join(format!("mori-test-{}", std::process::id()));
        fs::create_dir_all(dir.join("sub")).unwrap();
        fs::write(dir.join("visible"), "x").unwrap();
        fs::write(dir.join(".secret"), "x").unwrap();

        let without = build(&dir, false);
        let with = build(&dir, true);
        fs::remove_dir_all(&dir).unwrap();

        assert_eq!(without.children.len(), 2);
        assert_eq!(with.children.len(), 3);
    }
}