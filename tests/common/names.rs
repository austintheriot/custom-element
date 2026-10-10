use rand::{distributions::Alphanumeric, Rng};

fn get_random_letter_string() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .filter(|character| character.is_ascii_alphabetic())
        .map(|character| character.to_ascii_lowercase())
        .take(10)
        .map(char::from)
        .collect()
}

pub fn get_random_custom_element_name() -> String {
    let random_string_a = get_random_letter_string();
    let random_string_b = get_random_letter_string();
    let random_string_c = get_random_letter_string();
    format!("{random_string_a}-{random_string_b}-{random_string_c}")
}
