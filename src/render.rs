use crate::size;
use crate::tree::Node;

const BAR_WIDTH: usize = 20;

pub fn print(root: &Node, depth: usize, limit: usize) {
    line(root.size, root.size, &root.name());
    branch(root, root.size, "", depth, limit);
}

fn branch(node: &Node, total: u64, prefix: &str, depth: usize, limit: usize) {
    if depth == 0 {
        return;
    }

    let shown = node.children.len().min(limit);
    let rest = &node.children[shown..];

    for (i, child) in node.children[..shown].iter().enumerate() {
        let last = i == shown - 1 && rest.is_empty();
        let (connector, indent) = if last {
            ("└── ", "    ")
        } else {
            ("├── ", "│   ")
        };

        line(child.size, total, &format!("{prefix}{connector}{}", child.name()));
        branch(child, total, &format!("{prefix}{indent}"), depth - 1, limit);
    }

    if !rest.is_empty() {
        let bytes = rest.iter().map(|c| c.size).sum();
        line(bytes, total, &format!("{prefix}└── ... {} more", rest.len()));
    }
}

fn line(bytes: u64, total: u64, label: &str) {
    let ratio = if total == 0 {
        0.0
    } else {
        bytes as f64 / total as f64
    };

    println!(
        "{:>10} {} {:>5.1}%  {label}",
        size::human(bytes),
        bar(ratio),
        ratio * 100.0
    );
}

fn bar(ratio: f64) -> String {
    let filled = (ratio * BAR_WIDTH as f64).round() as usize;
    "█".repeat(filled) + &"░".repeat(BAR_WIDTH - filled)
}

#[cfg(test)]
mod tests {
    use super::{bar, BAR_WIDTH};

    #[test]
    fn bar_has_fixed_width() {
        for ratio in [0.0, 0.33, 0.5, 1.0] {
            assert_eq!(bar(ratio).chars().count(), BAR_WIDTH);
        }
    }

    #[test]
    fn half_bar_is_half_filled() {
        assert_eq!(bar(0.5).matches('█').count(), BAR_WIDTH / 2);
    }
}