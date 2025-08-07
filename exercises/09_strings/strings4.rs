// Calls of this function should be replaced with calls of `string_slice` or `string`.
fn placeholder(arg: &str) {
    // This function is just a placeholder.
    // It should not be called directly.
    // Instead, use `string_slice` or `string`.
    println!("This is a placeholder function.");
    string_slice(arg);
    string(arg.to_string());
}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: Here are a bunch of values - some are `String`, some are `&str`.
// Your task is to replace `placeholder(…)` with either `string_slice(…)`
// or `string(…)` depending on what you think each value is.
fn main() {
    string_slice("blue");

    string_slice("red");

    string_slice("hi");

    string_slice("rust is fun!");

    string("nice weather".into());

    string(format!("Interpolation {}", "Station"));

    // WARNING: This is byte indexing, not character indexing.
    // Character indexing can be done using `s.chars().nth(INDEX)`.
    string_slice(&String::from("abc")[0..1]);

    string_slice("  hello there ".trim());

    string("Happy Monday!".replace("Mon", "Tues"));

    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
