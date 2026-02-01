use arabic_reshaper::ArabicReshaper;
use unicode_bidi::BidiInfo;

pub fn pdx_text(input: &str) -> String {
    let has_arabic = input.chars().any(|c| ('\u{0600}'..='\u{06FF}').contains(&c));

    if !has_arabic {
        return input.to_string();
    }

    let reshaper = ArabicReshaper::new();
    let shaped = reshaper.reshape(input);

    let bidi = BidiInfo::new(&shaped, None);
    let para = &bidi.paragraphs[0];

    bidi.reorder_line(para, 0..shaped.len()).to_string()
}

/// Detect if text contains Arabic characters
pub fn is_arabic(text: &str) -> bool {
    text.chars().any(|c| ('\u{0600}'..='\u{06FF}').contains(&c))
}

/// Detect text direction
pub fn detect_direction(text: &str) -> crate::Direction {
    if is_arabic(text) {
        crate::Direction::RTL
    } else {
        crate::Direction::LTR
    }
}