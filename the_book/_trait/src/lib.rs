pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more..."))
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// デフォルト実装は、自らのトレイトのデフォルト実装を持たない他のメソッドを呼び出すことができます。 
// このようにすれば、トレイトは多くの有用な機能を提供しつつ、実装者は僅かな部分しか指定しなくて済むようになります。


/*
    where句を使ったより明確なトレイト境界
あまりたくさんのトレイト境界を使うことには欠点もあります。 
それぞれのジェネリック（な型）がそれぞれのトレイト境界をもつので、複数のジェネリック型の引数をもつ関数は、関数名と引数リストの間に大量のトレイト境界に関する情報を含むことがあります。 
これでは関数のシグネチャが読みにくくなってしまいます。 このため、Rustはトレイト境界を関数シグネチャの後のwhere句の中で指定するという別の構文を用意しています。

fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{...

*/