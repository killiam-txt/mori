use std::cmp::Reverse;
use std::path::PathBuf;

pub struct Node {
    pub path: PathBuf,
    pub size: u64,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new(path: PathBuf, size: u64) -> Self {
        Self {
            path,
            size,
            children: Vec::new(),
        }
    }

    pub fn push(&mut self, child: Node) {
        self.size += child.size;
        self.children.push(child);
    }

    pub fn sort(&mut self) {
        self.children.sort_by_key(|c| Reverse(c.size));
        self.children.iter_mut().for_each(Node::sort);
    }

    pub fn name(&self) -> String {
        self.path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| self.path.display().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::Node;
    use std::path::PathBuf;

    fn leaf(name: &str, size: u64) -> Node {
        Node::new(PathBuf::from(name), size)
    }

    #[test]
    fn push_adds_child_size() {
        let mut root = leaf("root", 0);
        root.push(leaf("a", 100));
        root.push(leaf("b", 50));
        assert_eq!(root.size, 150);
    }

    #[test]
    fn sort_is_descending_and_recursive() {
        let mut inner = leaf("inner", 0);
        inner.push(leaf("x", 1));
        inner.push(leaf("y", 9));

        let mut root = leaf("root", 0);
        root.push(leaf("small", 5));
        root.push(inner);
        root.sort();

        assert_eq!(root.children[0].name(), "inner");
        assert_eq!(root.children[0].children[0].name(), "y");
    }

    #[test]
    fn name_falls_back_to_full_path() {
        assert_eq!(leaf("/", 0).name(), "/");
    }
}