use super::hash_util::get_string_hash;

pub fn generate_uid(input: &str) -> String {
    let mut result = get_string_hash(input);
    result.truncate(8);
    result
}
