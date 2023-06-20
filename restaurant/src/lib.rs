mod front_of_house {
    mod hosting {
        fn add_to_wailist() {}
    }
}

pub fn eat_At_restaurant() {
    crate::front_of_house::hosting::add_to_wailist();

    front_of_house::hosting::add_to_wailist();
}

/* 
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
} */


#![allow(unused)]
fn main() {
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    // 夏 (summer) にライ麦 (Rye) パン付き朝食を注文
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    // やっぱり別のパンにする
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // 下の行のコメントを外すとコンパイルできない。食事についてくる
    // 季節のフルーツを知ることも修正することも許されていないので
    // meal.seasonal_fruit = String::from("blueberries");
}
}

/*

Rustにおけるプライバシーは、「あらゆる要素（関数、メソッド、構造体、enum、モジュールおよび定数）は標準では非公開」というやり方で動いています。
親モジュールの要素は子モジュールの非公開要素を使えませんが、子モジュールの要素はその祖先モジュールの要素を使えます。

*/

/*

親モジュールから始まる相対パスなら、superを最初につけることで構成できます。 ファイルシステムパスを..構文で始めるのに似ています。 
どのようなときにこの機能が使いたくなるのでしょう？

fix_incorrect_order関数はback_of_houseモジュールの中にあるので、superを使ってback_of_houseの親モジュールにいけます。
親モジュールは、今回の場合ルートであるcrateです。 そこから、serve_orderを探し、見つけ出します。
もしクレートのモジュールツリーを再編成することにした場合でも、back_of_houseモジュールとserve_order関数は同じ関係性で有り続け、一緒に動くように思われます。
そのため、superを使うことで、将来このコードが別のモジュールに移動するとしても、更新する場所が少なくて済むようにしました。

 */

// 構造体定義の前にpubを使うと、構造体は公開されますが、構造体のフィールドは非公開のままなのです。
// 一方で、enumを公開すると、そのヴァリアントはすべて公開されます。