fn choose_favorite<'a>(first: & str, second: &'a str) -> &'a str {
    println!("{first}");
    second
}

fn main() {}