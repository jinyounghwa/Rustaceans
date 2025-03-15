// 오늘 공부한 부분은 역참조 

// fn main() {
//     let x = 5;
//     let y = &x; // x에 대한 참조

//     println!("x: {}", x);
//     println!("y: {}", *y); // 역참조로 실제 값에 접근
// }

// fn main() {
//     let b = Box::new(10); // 힙에 10을 저장
//     println!("b: {}", *b); // 역참조로 값에 접근
// }

use std::ops::Deref;

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let m = MyBox(String::from("Rust"));
    hello(&m); // `&MyBox<String>` → `&String` → `&str` 자동 변환
}
