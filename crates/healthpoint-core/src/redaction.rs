//! Secret redaction helpers for logs, diagnostics, and error messages.

/// Redact known secrets from a string using literal replacement only.
///
/// This deliberately avoids regular expressions so the core crate stays small and predictable.
/// Secrets shorter than four characters are ignored to prevent excessive false positives.
pub fn redact_known_secrets<I, S>(input: &str, secrets: I) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut output = input.to_owned();
    for secret in secrets {
        let secret = secret.as_ref();
        if secret.len() >= 4 {
            output = output.replace(secret, "[REDACTED]");
        }
    }
    output
}

/// Redact common bearer/API-key-looking fragments from diagnostic text.
pub fn redact_common_patterns(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let lowered = line.to_ascii_lowercase();
            if lowered.contains("authorization:")
                || lowered.contains("x-api-key")
                || lowered.contains("api_key")
                || lowered.contains("apikey")
            {
                "[REDACTED]".to_owned()
            } else {
                line.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal_secret_redaction_ignores_tiny_values() {
        assert_eq!(
            redact_known_secrets("token abcdef", ["abcdef"]),
            "token [REDACTED]"
        );
        assert_eq!(redact_known_secrets("id abc", ["abc"]), "id abc");
    }

    #[test]
    fn common_pattern_redaction_removes_auth_lines() {
        assert_eq!(
            redact_common_patterns("ok\nAuthorization: Bearer secret"),
            "ok\n[REDACTED]"
        );
    }
}
