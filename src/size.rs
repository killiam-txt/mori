const UNITS: [&str; 5] = ["B", "KiB", "MiB", "GiB", "TiB"];

pub fn human(bytes: u64) -> String {
    let mut value = bytes as f64;
    let mut unit = 0;

    while value >= 1024.0 && unit < UNITS.len() - 1 {
        value /= 1024.0;
        unit += 1;
    }

    if unit == 0 {
        format!("{bytes} {}", UNITS[0])
    } else {
        format!("{value:.1} {}", UNITS[unit])
    }
}

#[cfg(test)]
mod tests {
    use super::human;

    #[test]
    fn bytes_stay_whole() {
        assert_eq!(human(512), "512 B");
    }

    #[test]
    fn kibibytes() {
        assert_eq!(human(1536), "1.5 KiB");
    }

    #[test]
    fn gibibytes() {
        assert_eq!(human(3 * 1024u64.pow(3)), "3.0 GiB");
    }
}