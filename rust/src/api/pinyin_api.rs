use pinyin::{lazy_pinyin, pinyin, Args, Style, ToPinyin};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinyinStyle {
    Normal,
    Tone,
    Tone2,
    FirstLetter,
    Initials,
    Finals,
    FinalsTone,
    FinalsTone2,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PinyinArgs {
    pub style: PinyinStyle,
    pub heteronym: bool,
}

impl Default for PinyinArgs {
    fn default() -> Self {
        Self {
            style: PinyinStyle::Normal,
            heteronym: false,
        }
    }
}

fn to_pinyin_style(style: PinyinStyle) -> Style {
    match style {
        PinyinStyle::Normal => Style::Normal,
        PinyinStyle::Tone => Style::Tone,
        PinyinStyle::Tone2 => Style::Tone2,
        PinyinStyle::FirstLetter => Style::FirstLetter,
        PinyinStyle::Initials => Style::Initials,
        PinyinStyle::Finals => Style::Finals,
        PinyinStyle::FinalsTone => Style::FinalsTone,
        PinyinStyle::FinalsTone2 => Style::FinalsTone2,
    }
}

fn to_args(args: &PinyinArgs) -> Args {
    Args {
        style: to_pinyin_style(args.style),
        heteronym: args.heteronym,
    }
}

fn first_letter_of(text: &str, digit_fallback: &str) -> String {
    let trimmed = text.trim();
    let Some(c) = trimmed.chars().next() else {
        return String::new();
    };
    if c.is_ascii_digit() {
        return digit_fallback.to_string();
    }
    if c.is_ascii_alphabetic() {
        return c.to_ascii_uppercase().to_string();
    }
    if let Some(py) = c.to_pinyin() {
        return py
            .plain()
            .chars()
            .next()
            .map(|ch| ch.to_ascii_uppercase().to_string())
            .unwrap_or_default();
    }
    digit_fallback.to_string()
}

#[flutter_rust_bridge::frb(sync)]
pub fn to_lazy_pinyin(text: String, args: PinyinArgs) -> Vec<String> {
    lazy_pinyin(&text, &to_args(&args))
}

#[flutter_rust_bridge::frb(sync)]
pub fn to_pinyin(text: String, args: PinyinArgs) -> Vec<Vec<String>> {
    pinyin(&text, &to_args(&args))
}

#[flutter_rust_bridge::frb(sync)]
pub fn get_first_letter(text: String, digit_fallback: Option<String>) -> String {
    first_letter_of(&text, digit_fallback.as_deref().unwrap_or("#"))
}

#[flutter_rust_bridge::frb(sync)]
pub fn get_first_letters(texts: Vec<String>, digit_fallback: Option<String>) -> Vec<String> {
    let fallback = digit_fallback.as_deref().unwrap_or("#");
    texts
        .iter()
        .map(|text| first_letter_of(text, fallback))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_first_letter_chinese() {
        assert_eq!(first_letter_of("张三", "#"), "Z");
    }

    #[test]
    fn test_get_first_letter_english() {
        assert_eq!(first_letter_of("Apple", "#"), "A");
    }

    #[test]
    fn test_get_first_letter_digit_default() {
        assert_eq!(first_letter_of("123", "#"), "#");
    }

    #[test]
    fn test_get_first_letter_digit_custom() {
        assert_eq!(first_letter_of("123", "*"), "*");
    }

    #[test]
    fn test_get_first_letter_trimmed() {
        assert_eq!(first_letter_of("  李四", "#"), "L");
    }

    #[test]
    fn test_get_first_letter_empty() {
        assert_eq!(first_letter_of("", "#"), "");
    }

    #[test]
    fn test_to_lazy_pinyin_normal() {
        let args = PinyinArgs::default();
        assert_eq!(
            to_lazy_pinyin("中国人".to_string(), args),
            vec!["zhong", "guo", "ren"]
        );
    }

    #[test]
    fn test_get_first_letters_batch() {
        let result = get_first_letters(
            vec!["张三".to_string(), "Bob".to_string(), "007".to_string()],
            None,
        );
        assert_eq!(result, vec!["Z", "B", "#"]);
    }

    #[test]
    fn test_get_first_letter_mixed_chinese_first() {
        assert_eq!(first_letter_of("张3abc", "#"), "Z");
    }

    #[test]
    fn test_get_first_letter_mixed_english_first() {
        assert_eq!(first_letter_of("A张三", "#"), "A");
    }

    #[test]
    fn test_get_first_letter_mixed_digit_first() {
        assert_eq!(first_letter_of("1张三", "#"), "#");
    }

    #[test]
    fn test_get_first_letter_punctuation() {
        assert_eq!(first_letter_of("!hello", "#"), "#");
        assert_eq!(first_letter_of("@test", "#"), "#");
    }

    #[test]
    fn test_get_first_letter_punctuation_custom_fallback() {
        assert_eq!(first_letter_of("!hello", "*"), "*");
    }
}
