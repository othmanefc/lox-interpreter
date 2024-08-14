pub fn trim_string(to_split: &String) -> String {
    let length = to_split.len();
    to_split[1..length - 1].to_string()
}

pub fn format_number_as_string(num_as_string: &String) -> String {
    let mut new_string = num_as_string.clone();
    if new_string.ends_with('.') {
        new_string.push('0')
    } else if !new_string.contains('.') {
        new_string.push_str(".0")
    } else if new_string.ends_with(".00") {
        new_string = new_string.replace(".00", ".0");
    }
    new_string
}


