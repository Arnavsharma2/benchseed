/// Parse a CSV delimiter known to be safe for every generated table.
pub(crate) fn parse_delimiter(value: &str) -> Result<char, String> {
    let delimiter = match value {
        "\\t" => '\t',
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

    match delimiter {
        ',' | '|' | '\t' | ';' => Ok(delimiter),
        _ => Err(supported_delimiters_error(value)),
    }
}

fn delimiter_length_error(value: &str) -> String {
    format!(
        "Delimiter must be a single character or escape sequence (\\t, \\n, \\r, \\\\), got: '{value}'"
    )
}

fn supported_delimiters_error(value: &str) -> String {
    format!(
        "Unsupported CSV delimiter '{value}'; supported delimiters are ',' (comma), '|' (pipe), '\\t' (tab), and ';' (semicolon)"
    )
}

#[cfg(test)]
mod tests {
    use super::parse_delimiter;

    #[test]
    fn accepts_safe_delimiters() {
        assert_eq!(parse_delimiter(","), Ok(','));
        assert_eq!(parse_delimiter("|"), Ok('|'));
        assert_eq!(parse_delimiter("\\t"), Ok('\t'));
        assert_eq!(parse_delimiter("\t"), Ok('\t'));
        assert_eq!(parse_delimiter(";"), Ok(';'));
    }

    #[test]
    fn rejects_unsafe_delimiters() {
        for value in ["-", "_", ".", "/", ":", "@", "#", " ", "\\n"] {
            let error = parse_delimiter(value).expect_err("delimiter should be rejected");
            assert!(error.contains("supported delimiters"), "{error}");
        }
    }

    #[test]
    fn rejects_invalid_width_and_non_ascii_values() {
        assert!(parse_delimiter("").is_err());
        assert!(parse_delimiter("::").is_err());
        assert!(parse_delimiter("€").is_err());
    }
}
