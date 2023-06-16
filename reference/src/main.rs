fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    change(&mut s1);

    let _r1 = &mut s1;
    let _r2 = &mut s1;

    // println!("{}, {}", r1, r2);

    let reference_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// 借用した値を変更しようとする
fn change(some_sting: &mut String) {
    some_sting.push_str(", world");
}

// ぶら下がりポインタを作る関数(コンパイルエラーになる)
fn dangle() -> &String {
    let s = String::from("hello");

    &s
    // s が スコープを抜けるからドロップされて，&sはダングリングポインタになる？
}

// 値の所有権をもらう代わりに引数としてオブジェクトへの参照を取る
// 値の所有権がムーブしないから，s1をそのまま使える

// これらのアンド記号が参照であり、これのおかげで所有権をもらうことなく値を参照することができるのです。

// この&s1という記法により、s1の値を参照する参照を生成することができますが、これを所有することはありません。
// 所有してないということは、指している値は、参照がスコープを抜けてもドロップされないということです。

// 借用した値を変更するには，可変な参照(&mut)を使うことができる
// ところが、可変な参照には大きな制約が一つあります: 特定のスコープで、ある特定のデータに対しては、 一つしか可変な参照を持てないことです。

// この制約がある利点は、コンパイラがコンパイル時にデータ競合を防ぐことができる点です。
