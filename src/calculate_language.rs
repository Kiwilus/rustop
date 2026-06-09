pub fn extract_country(language: &str) -> String {
    language
    .split(',')
    .next()
    .unwrap_or("")
    .split('=')
    .nth(1)
    .unwrap_or("")
    .split('/')
    .nth(1)
    .unwrap_or("")
    .split(':')
    .next()
    .unwrap_or("")
    .to_uppercase()
}