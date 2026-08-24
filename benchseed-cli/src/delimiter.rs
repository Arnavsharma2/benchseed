/// Parse a single-byte CSV delimiter, including common escape sequences.
pub(crate) fn parse_delimiter(value: &str) -> Result<char, String> {
    let delimiter = match value {
        "\\t" => '\t',
        "\\n" => '\n',
        "\\r" => '\r',
        "\\\\" => '\\',
        _ => {
            let mut chars = value.chars();
            let Some(delimiter) = chars.next() else {
                return Err(delimiter_length_error(value));
            };
            if chars.next().is_some() {
                return Err(delimiter_length_error(value));
            }
            delimiter
        }
    };

    if !delimiter.is_ascii() {
        return Err(format!(
            "Delimiter must be an ASCII character, got: '{delimiter}'"
        ));
    }

    Ok(delimiter)
}

fn delimiter_length_error(value: &str) -> String {
    format!(
        "Delimiter must be a single character or escape sequence (\\t, \\n, \\r, \\\\), got: '{value}'"
    )
}

#[cfg(test)]
mod tests {
    use super::parse_delimiter;

    #[test]
    fn parses_characters_and_escape_sequences() {
        assert_eq!(parse_delimiter(","), Ok(','));
        assert_eq!(parse_delimiter("\\t"), Ok('\t'));
        assert_eq!(parse_delimiter("\\n"), Ok('\n'));
        assert_eq!(parse_delimiter("\\r"), Ok('\r'));
        assert_eq!(parse_delimiter("\\\\"), Ok('\\'));
    }

    #[test]
    fn rejects_invalid_width_and_non_ascii_values() {
        assert!(parse_delimiter("").is_err());
        assert!(parse_delimiter("::").is_err());
        assert!(parse_delimiter("€").is_err());
    }
}
