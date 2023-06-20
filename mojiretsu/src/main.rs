fn main() {
    let mut _s = String::new();

    let data = "initial contents";

    let _s = data.to_string();

    let s = "initial contents".to_string();

    println!("{}", s);

    // 文字列は UTF-8エンコードされる
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
    println!("{}", hello);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); // push_strはs2のスライスをとる。
    println!("{}", s1);

    println!("{}", s2); // push_strはs2のスライスをとっているから，s2の所有権はs2にある

    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2; // s1 は ムーブされて，使用できない
                       // fn add(self, s: &str) -> String {
    println!("{}", s3);
    // &s2 の 型は &String であるのに，add を呼んだ時なぜコンパイルエラーにならないのか？
    // add呼び出しで&s2を使える理由は、コンパイラが&String引数を&strに型強制してくれるためです。
    // ここでは，&s2 を &s2[..] に変えている

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    // let s1 = String::from("hello");
    // let h = s1[0]; <- コンパイルエラー！

    // String は Vec<u8>のラッパーである

    // Rust の 文字列はUTF-8エンコードをサポートしている
    // 故に，"Здравствуйте" の様な文字列を作ることができる
    // しかし，この様に1つの文字が，1バイト分の情報でできているか，複数バイト分の情報でできているか判定することはできない
    // 故に，Rustの文字列は添字でアクセスすることができない。
    /*
    幸いなことに、他の方法でも文字列の要素にアクセスすることができます。
    もし、個々のUnicodeスカラー値に対して処理を行う必要があったら、最適な方法はcharsメソッドを使用するものです。
    “नमस्ते”に対してcharsを呼び出したら、分解して6つのchar型の値を返すので、各要素にアクセスするには、 その結果を走査すればいいわけです:
     */
}
