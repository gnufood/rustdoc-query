use super::Token;

/// Byte span of one identifier part within its word.
type Span = (usize, usize);

/// Emit parts and adjacent concatenations, excluding the full-word duplicate.
pub(super) fn emit_parts(tokens: &mut Vec<Token>, word: &str, offset: usize) {
    let spans = split(word);
    for &(start, end) in &spans {
        push(tokens, &word[start..end], offset + start, offset + end);
    }
    for pair in spans.windows(2) {
        let (start, _) = pair[0];
        let (_, end) = pair[1];
        if start == 0 && end == word.len() {
            continue;
        }
        let joined = format!("{}{}", &word[start..pair[0].1], &word[pair[1].0..end]);
        push(tokens, &joined, offset + start, offset + end);
    }
}

/// Split an identifier word on underscores, camel-case boundaries, and the
/// end of an acronym (`HTTPClient` → `http`, `client`).
fn split(word: &str) -> Vec<Span> {
    let mut spans = Vec::new();
    let mut part_start = 0;
    let mut previous: Option<(usize, char)> = None;
    let mut before_previous: Option<(usize, char)> = None;

    for (index, ch) in word.char_indices() {
        if ch == '_' {
            record(&mut spans, part_start, index);
            part_start = index + ch.len_utf8();
            previous = None;
            before_previous = None;
            continue;
        }
        if let Some((previous_index, previous_char)) = previous {
            let starts_camel_part = previous_char.is_lowercase() && ch.is_uppercase();
            let ends_acronym = before_previous.is_some_and(|(_, prior)| {
                prior.is_uppercase() && previous_char.is_uppercase() && ch.is_lowercase()
            });
            if let Some(split_at) = starts_camel_part
                .then_some(index)
                .or_else(|| ends_acronym.then_some(previous_index))
            {
                record(&mut spans, part_start, split_at);
                part_start = split_at;
            }
        }
        before_previous = previous;
        previous = Some((index, ch));
    }
    record(&mut spans, part_start, word.len());
    spans
}

fn record(spans: &mut Vec<Span>, start: usize, end: usize) {
    if start < end {
        spans.push((start, end));
    }
}

fn push(tokens: &mut Vec<Token>, part: &str, start: usize, end: usize) {
    if !part.is_empty() {
        tokens.push(Token {
            term: part.chars().flat_map(char::to_lowercase).collect(),
            start,
            end,
            colocated: true,
        });
    }
}
