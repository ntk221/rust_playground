fn main() {
    let _v: Vec<i32> = Vec::new();

    let v = vec![1, 2, 3];
    println!("{:?}", v);

    {
        let mut v = Vec::new();
        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
    } // <- ここで，ドロップされる
    println!("{:?}", v);

    let v = vec![1, 2, 3, 4, 5];

    // let third: &i32 = &v[5]; <- 配列外参照できる
    // println!("The third element is {}", third);

    match v.get(2) {
        // get method は Result を返すので，安全に使える
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    /*
    プログラムに有効な参照がある場合、借用チェッカー (borrow checker) は、（第4章で解説しましたが）所有権と借用規則を強制し、ベクタの中身へのこの参照や他のいかなる参照も有効であり続けることを保証してくれます。
    同一スコープ上では、可変と不変な参照を同時には存在させられないというルールを思い出してください。
     */

    let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0]; // ここでvを不変借用している
    // let first = &mut v[0]; <- これもダメだ...

    v.push(6); // ここでvの可変借用を取得している

    println!("the first element is: {:?}", v);

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in &row {
        match i {
            (SpreadsheetCell::Int(n)) => println!("{}", n),
            (SpreadsheetCell::Text(str)) => println!("{:?}", str),
            (SpreadsheetCell::Float(f)) => println!("{}", f),
            _ => println!("hoge"),
        }
    }

    match row.get(0) {
        Some(SpreadsheetCell::Int(n)) => println!("{}", n),
        _ => println!("hoge"),
    }
}
