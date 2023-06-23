#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // rect1にrect2ははまり込む？
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("rect1 is {:#?}", rect1);

    println!("square {:?}", Rectangle::square(4));

    println!("長方形の面積は，{} 平方ピクセルです", rect1.area());
}

// 第4章で触れたように、構造体の所有権を奪うよりも借用する必要があります。こうすることでmainは所有権を保って、 rect1を使用し続けることができ、そのために関数シグニチャと関数呼び出し時に&を使っているわけです。

// 波括弧内に:?という指定子を書くと、println!にDebugと呼ばれる出力整形を使いたいと指示するのです。
