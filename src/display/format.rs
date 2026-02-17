// Number formatting utilities

pub fn format_tokens(n: u64) -> String {
    if n < 1000 {
        n.to_string()
    } else if n < 1_000_000 {
        format!("{:.1}K", n as f64 / 1000.0)
    } else {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    }
}

pub fn format_tokens_short(n: u64) -> String {
    if n >= 100_000_000 {
        format!("{}M", n / 1_000_000)
    } else if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 100_000 {
        format!("{}K", n / 1_000)
    } else if n >= 1_000 {
        format!("{:.1}K", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}

pub fn format_cost(cost: f64) -> String {
    if cost < 0.01 {
        format!("${:.4}", cost)
    } else if cost < 1.0 {
        format!("${:.2}", cost)
    } else {
        format!("${:.2}", cost)
    }
}

pub fn format_cost_ref(cost: f64) -> String {
    if cost < 0.01 {
        format!("${:.4}", cost)
    } else if cost < 1.0 {
        format!("${:.3}", cost)
    } else {
        format!("${:.2}", cost)
    }
}

pub fn format_duration(seconds: i64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    if hours > 0 {
        if minutes > 0 {
            format!("{}h{}m", hours, minutes)
        } else {
            format!("{}h", hours)
        }
    } else if minutes > 0 {
        format!("{}m", minutes)
    } else {
        format!("{}s", secs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_tokens_short() {
        assert_eq!(format_tokens_short(500), "500");
        assert_eq!(format_tokens_short(1_500), "1.5K");
        assert_eq!(format_tokens_short(100_000), "100K");
        assert_eq!(format_tokens_short(1_500_000), "1.5M");
        assert_eq!(format_tokens_short(100_000_000), "100M");
    }

    #[test]
    fn test_format_cost_ref() {
        assert_eq!(format_cost_ref(0.005), "$0.0050");
        assert_eq!(format_cost_ref(0.05), "$0.050");
        assert_eq!(format_cost_ref(1.23), "$1.23");
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(45), "45s");
        assert_eq!(format_duration(150), "2m");
        assert_eq!(format_duration(3665), "1h1m");
        assert_eq!(format_duration(3600), "1h");
        assert_eq!(format_duration(6671), "1h51m");
    }
}
