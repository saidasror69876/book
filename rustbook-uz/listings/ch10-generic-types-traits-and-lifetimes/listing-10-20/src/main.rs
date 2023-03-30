fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let natija = eng_uzun(string1.as_str(), string2);
    println!("Eng uzun satr {}", natija);
}

// ANCHOR: here
fn eng_uzun(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// ANCHOR_END: here
