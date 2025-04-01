/// Returns a minimum prefix of the input string that is unique to all
/// items in the list.
pub fn minimum_prefix(input: &str, items: &Vec<String>) -> String {
    for len in 1..=input.len() {
        let prefix = &input[..len];
        let mut matching = Vec::new();

        for candidate in items {
            if candidate.starts_with(prefix) {
                matching.push(candidate);
            }
        }

        if matching.len() == 1 {
            return prefix.to_string();
        }
    }
    input.to_string()
}
