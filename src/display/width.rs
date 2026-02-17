// Display width calculation with ANSI stripping and CJK support
use unicode_width::UnicodeWidthStr;

pub fn strip_ansi(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            if chars.peek() == Some(&'[') {
                chars.next();
                while let Some(&c) = chars.peek() {
                    chars.next();
                    if c.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
        } else {
            result.push(ch);
        }
    }
    result
}

pub fn display_width(s: &str) -> usize {
    strip_ansi(s).width()
}

pub fn truncate_to_width(s: &str, max_width: usize) -> String {
    let mut result = String::new();
    let mut current_width = 0;
    let mut chars = s.chars().peekable();
    let mut in_escape = false;
    let mut escape_seq = String::new();

    while let Some(ch) = chars.next() {
        if ch == '\x1b' && chars.peek() == Some(&'[') {
            in_escape = true;
            escape_seq.clear();
            escape_seq.push(ch);
            continue;
        }

        if in_escape {
            escape_seq.push(ch);
            if ch.is_ascii_alphabetic() {
                result.push_str(&escape_seq);
                in_escape = false;
            }
            continue;
        }

        let char_width = ch.to_string().width();
        if current_width + char_width > max_width {
            break;
        }

        result.push(ch);
        current_width += char_width;
    }

    result
}
