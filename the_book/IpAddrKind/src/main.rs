#[derive(Debug)]
enum IpAddr {
    V4(u8, i8),
    V6(String),
}

/*#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}*/

fn route(ip_addr: IpAddr) {
    println!("{:?}", ip_addr);
}

fn main() {
    /*let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };*/

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    route(home);
}

// 各enumの列挙子に直接データを格納して、enumを構造体内に使うというよりもenumだけを使って、 同じ概念をもっと簡潔な方法で表現することができます。
// この新しいIpAddrの定義は、 V4とV6列挙子両方にString値が紐付けられていることを述べています。

/*
   構造体よりもenumを使うことには、別の利点もあります: 各列挙子に紐付けるデータの型と量は、異なってもいいのです。
   バージョン4のIPアドレスには、常に0から255の値を持つ4つの数値があります。
   V4のアドレスは、4つのu8型の値として格納するけれども、 V6のアドレスは引き続き、単独のString型の値で格納したかったとしても、構造体では不可能です。
*/

/*
    struct IPAddress {
    ip: IPType, // これが異なるデータ型を持つ列挙子になる想定ですが、構造体では不可能です
}

enum IPType {
    IPv4(u8, u8, u8, u8), // IPv4アドレスに対応する列挙子。4つのu8型の値を持つ
    IPv6(String), // IPv6アドレスに対応する列挙子。単一のString型の値を持つ
}

fn main() {
    let ipv4 = IPType::IPv4(192, 168, 0, 1);
    let ipv6 = IPType::IPv6(String::from("2001:0db8:85a3:0000:0000:8a2e:0370:7334"));

    let ip_address1 = IPAddress { ip: ipv4 }; // OK
    let ip_address2 = IPAddress { ip: ipv6 }; // エラー: 異なるデータ型の列挙子は構造体に含めることができない
}
 */

/*
     Compiling playground v0.0.1 (/playground)
warning: unused variable: `ip_address1`
  --> src/main.rs:14:9
   |
14 |     let ip_address1 = IPAddress { ip: ipv4 }; // OK
   |         ^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_ip_address1`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: unused variable: `ip_address2`
  --> src/main.rs:15:9
   |
15 |     let ip_address2 = IPAddress { ip: ipv6 }; // エラー: 異なるデータ型の列挙子は構造体に含めることができない
   |         ^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_ip_address2`

warning: field `ip` is never read
 --> src/main.rs:2:5
  |
1 | struct IPAddress {
  |        --------- field in this struct
2 |     ip: IPType, // これが異なるデータ型を持つ列挙子になる想定ですが、構造体では不可能です
  |     ^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: `playground` (bin "playground") generated 3 warnings (run `cargo fix --bin "playground"` to apply 2 suggestions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.55s
     Running `target/debug/playground`
 */
