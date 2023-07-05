fn main() {
    let s = String::from("hello world");

    let word = first_word(&s);

    // s.clear();

    // sがもう存在しないのに，wordが使えるのは危険!
    println!("{}", word);

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{}{}", hello, world);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

/*fn second_word(s: &String) -> &str {

}*/

// 文字列スライスを意味する型は、&strと記述します
