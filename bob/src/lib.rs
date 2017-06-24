pub fn reply(query: &'static str) -> &'static str {
    if query.ends_with("?") {
        return "Sure.";
    }
    if query == "" {
        return "Fine. Be that way!";
    }
    if query.to_uppercase() == query {
        return "Whoa, chill out!";
    }
    return "Whatever.";
}
