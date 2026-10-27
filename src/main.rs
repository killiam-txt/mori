mod render;
mod scan;
mod size;
mod tree;

use std::path::Path;

fn main() {
    let root = scan::build(Path::new("."), false);
    render::print(&root, 2, 5);
}