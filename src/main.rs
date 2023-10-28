fn concatenate_strings(s1: &str, s2: &str) -> String {
    let mut result = String::new();

    result.push_str(s1);
    result.push_str(s2);

    return result;
}

fn main() {
    let string1 = "Hello, You and ";
    let string2 = "World!";
    let concatenate_string = concatenate_strings(string1, string2);
    println!("{}", concatenate_string);
}
