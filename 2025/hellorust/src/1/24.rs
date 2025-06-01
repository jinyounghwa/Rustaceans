use std::fmt::Display;
// impl 과 제네렉은 타입 추상화는 같지만, 여러타입은 impl을 사용한다. 복수타입 인자가 필요할때는 제네릭
// impl 사용
fn print_twice_impl(x: impl Display, y: impl Display) {
    println!("impl 방식: {} {}", x, y);
}

// 제네릭 사용
fn print_twice_generic<T: Display>(x: T, y: T) {
    println!("제네릭 방식: {} {}", x, y);
}

fn main() {
    let a = 42;
    let b = "hello";

    // impl 방식: 서로 다른 타입도 허용
    print_twice_impl(a, b);

    let x = "hi";
    let y = "there";

    // 제네릭 방식: 같은 타입만 허용
    print_twice_generic(x, y);

    // 아래는 컴파일 에러 발생 (다른 타입이므로)
    // print_twice_generic(10, "oops");
}