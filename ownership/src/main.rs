fn main() {
    let my_string = String::from("hello world");
    // first_wordは`String`のスライスに対して機能する
    let word = first_word(&my_string[..]);
    println!("word = {}", word);
    let my_string_literal = "hello world";
    // first_wordは文字列リテラルのスライスに対して機能する
    let word = first_word(&my_string_literal[..]);
    println!("word = {}", word);
    // 文字列リテラルは、すでに文字列スライスなので、
    // スライス記法なしでも機能するのだ！
    let word = first_word(my_string_literal);
    println!("word = {}", word);
}



fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
