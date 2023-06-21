use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    // panic!("crush!");

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => {
                panic!("Tried to create file but there was a problem: {:?}", e)
            }
        },
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        }
    };

    /*
       Result<T, E>型には、色々な作業をするヘルパーメソッドが多く定義されています。
       それらの関数の一つは、 unwrapと呼ばれますが、リスト9-4で書いたmatch式と同じように実装された短絡メソッドです。
       Result値がOk列挙子なら、unwrapはOkの中身を返します。ResultがErr列挙子なら、 unwrapはpanic!マクロを呼んでくれます。
       こちらが実際に動作しているunwrapの例です:
    */
    // let f = File::open("hoge.txt").unwrap();
    /*
       別のメソッドexpectは、unwrapに似ていますが、panic!のエラーメッセージも選択させてくれます。
       unwrapの代わりにexpectを使用して、いいエラーメッセージを提供すると、意図を伝え、 パニックの原因をたどりやすくしてくれます。
    */
    // let f = File::open("hoge.txt").expect("Failed to open hoge.txt");

    let f = read_username_from_file().unwrap();
}

/*fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}*/

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
