mod scan;
mod size;
mod tree;

use std::path::Path;

fn main() {
    let root = scan::build(Path::new("."), false);
    println!("{} {}", root.name(), size::human(root.size));

    for child in &root.children {
        println!("  {} {}", child.name(), size::human(child.size));
    }
}