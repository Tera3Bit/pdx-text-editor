use arabic_reshaper::ArabicReshaper;
use unicode_bidi::BidiInfo;

pub fn pdx_text(input: &str) -> String {
    let has_arabic = input.chars().any(|c| ('\u{0600}'..='\u{06FF}').contains(&c));

    if !has_arabic {
        return input.to_string();
    }

    let reshaper = ArabicReshaper::new();
    let shaped = reshaper.reshape(input);

    if shaped.is_empty() {
        return input.to_string();
    }

    let bidi = BidiInfo::new(&shaped, None);

    // Guard against empty paragraphs (e.g. whitespace-only input)
    if bidi.paragraphs.is_empty() {
        return shaped;
    }

    let para = &bidi.paragraphs[0];
    let line_range = 0..shaped.len().min(para.range.end);

    if line_range.is_empty() {
        return shaped;
    }

    bidi.reorder_line(para, line_range).to_string()
}