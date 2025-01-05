fn main() {
    let numbers = [1, 2, 3, 4, 5];

    for num in numbers.iter() {
        println!("The number is: {}", num);
    }
}

// numbers는 정수 배열입니다.
// for num in numbers.iter()는 배열의 각 요소를 순회하며 num 변수에 할당합니다.
//println! 매크로를 사용하여 각 요소를 출력합니다.