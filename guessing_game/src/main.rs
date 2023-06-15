use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("数を当ててね！");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("秘密の数字は...{}", secret_number);

    println!("予想を入力してみよう！");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("あなたの予想は{}です", guess);

    loop {
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小さすぎ"),
            Ordering::Greater => println!("大きすぎ"),
            Ordering::Equal => println!("やったね！"),
        }
    }
}

// とりあえず知っておいてほしいのは、変数のように参照もデフォルトで不変であることです。
// したがって、&guessではなく&mut guessと書いて可変にする必要があります。

// Result型の値にも、他の型と同様にメソッドが定義されています。 io::Resultのインスタンスにはexpectメソッドがありますので、これを呼び出せます。
// このio::ResultインスタンスがErrの値の場合、expectメソッドはプログラムをクラッシュさせ、引数として渡されたメッセージを表示します。

// しかし待ってください、このプログラムには既にguessという名前の変数がありませんでしたか？  たしかにありますが、Rustではguessの前の値を新しい値で覆い隠す（shadowする）ことが許されているのです。
