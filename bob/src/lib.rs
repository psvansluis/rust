pub fn reply(message: &str) -> &str {
    let trimmed = message.trim();
    let is_shouting =
        trimmed.to_ascii_uppercase() == trimmed && trimmed.to_ascii_lowercase() != trimmed;
    if trimmed.ends_with("?") {
        if is_shouting {
            return "Calm down, I know what I'm doing!";
        }
        return "Sure.";
    }

    if is_shouting {
        return "Whoa, chill out!";
    }

    if trimmed.is_empty() {
        return "Fine. Be that way!";
    }

    "Whatever."
}
