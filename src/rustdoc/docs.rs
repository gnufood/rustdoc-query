/// Max summary length in chars before truncation with `…`.
pub(crate) const SUMMARY_MAX: usize = 120;

/// Strip leading ATX headings, return `(first_paragraph, was_truncated)`.
///
/// Paragraphs split on blank lines. Returns `(None, false)` for absent/blank docs.
pub(crate) fn first_paragraph(docs: Option<&str>) -> (Option<String>, bool) {
    let Some(docs) = docs else {
        return (None, false);
    };
    let mut body = docs.trim_start();
    while body.starts_with('#') {
        match body.split_once('\n') {
            Some((_, rest)) => body = rest.trim_start(),
            None => return (None, false),
        }
    }
    let trimmed = body.trim();
    if trimmed.is_empty() {
        return (None, false);
    }
    match trimmed.split_once("\n\n") {
        Some((first, rest)) => {
            let truncated = !rest.trim().is_empty();
            (Some(first.trim().to_owned()), truncated)
        }
        None => (Some(trimmed.to_owned()), false),
    }
}

/// First paragraph of `docs` joined into one line, capped at [`SUMMARY_MAX`] chars (`…` appended when cut).
pub(crate) fn summary_line(docs: Option<&str>) -> Option<String> {
    let (para, _) = first_paragraph(docs);
    let para = para?;
    let joined: String = para
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    if joined.chars().count() <= SUMMARY_MAX {
        return Some(joined);
    }
    let cut: String = joined.chars().take(SUMMARY_MAX).collect();
    Some(format!("{cut}…"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_paragraph_splits_and_flags() {
        assert_eq!(first_paragraph(None), (None, false));
        assert_eq!(first_paragraph(Some("   ")), (None, false));
        assert_eq!(
            first_paragraph(Some("One line.")),
            (Some("One line.".into()), false)
        );
        let (first, trunc) = first_paragraph(Some("Para one.\n\nPara two."));
        assert_eq!(first, Some("Para one.".into()));
        assert!(trunc);
    }

    #[test]
    fn first_paragraph_strips_leading_headings() {
        let (first, trunc) = first_paragraph(Some("# reqwest\n\nReal intro.\n\nmore"));
        assert_eq!(first, Some("Real intro.".into()));
        assert!(trunc);
        assert_eq!(first_paragraph(Some("# a\n## b\n")), (None, false));
        assert_eq!(
            first_paragraph(Some("# Title\n\nOnly para.")),
            (Some("Only para.".into()), false)
        );
    }

    #[test]
    fn summary_line_skips_empty_and_caps() {
        assert_eq!(summary_line(None), None);
        assert_eq!(
            summary_line(Some("\n\n  \nHello.\nmore")),
            Some("Hello. more".into())
        );
        let long = "x".repeat(200);
        let s = summary_line(Some(&long)).unwrap();
        assert_eq!(s.chars().count(), SUMMARY_MAX + 1); // 120 + ellipsis
    }
}
