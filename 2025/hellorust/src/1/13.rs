

// fn main() {
//     let number = 5;

//     match number {
//         1 => println!("One"),
//         2 => println!("Two"),
//         3..=5 => println!("Three to Five"), // 범위 패턴
//         _ => println!("Other"), // 기본 케이스
//     }
// }
use std::io;

fn main() {
    // 입력 받기
    let mut input = String::new();
    println!("숫자를 입력하세요:");

    io::stdin().read_line(&mut input).unwrap();  //.unwrap(); 실패 시 바로 종료

    // 숫자로 변환
    let number: i32 = match input.trim().parse() { //입력받은 문자열을 i32, f64 등으로 변환.
        Ok(n) => n,
        Err(_) => {
            println!("유효한 숫자를 입력하세요.");
            return;
        }
    };

    // 홀수/짝수 판별
    match number % 2 {
        0 => println!("{}는 짝수입니다.", number),
        1 | -1 => println!("{}는 홀수입니다.", number),
        _ => unreachable!(), // 이 부분은 실행되지 않음
    }
}
// 🤔 왜 바로 숫자로 입력받을 수 없을까?
// 표준 입력(stdin)으로 받는 데이터는 기본적으로 텍스트(String) 형태로 전달돼.

// Rust에서는 입력값을 직접 숫자로 받을 수 있는 메서드를 제공하지 않아.

// 그래서 문자열로 입력받은 후 parse() 메서드를 사용해 숫자로 변환해야 해.