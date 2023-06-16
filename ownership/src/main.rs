fn main() {
    let s1 = String::from("hello");
    // takes_ownership(s.clone()); deep copy
    takes_ownership(s1);

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

    println!("{}", s3);

    // i32はCopy
    let x = 5;

    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
