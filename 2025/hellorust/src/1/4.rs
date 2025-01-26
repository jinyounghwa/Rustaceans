/*Rust에서 flat_map은 Iterator 트레이트의 메서드로 구현되어 있습니다. 이 메서드는 각 항목을 반복 가능한 값으로 매핑한 다음, 이 반복 가능한 값들을 평탄화(flatten)합니다.

아래는 flat_map의 기본적인 사용 방법과 예제입니다: */

fn main() {
    let numbers = vec![1, 2, 3];

    // 각 숫자를 두 배로 늘려서 새로운 벡터로 만들고 평탄화
    let flattened: Vec<i32> = numbers
        .into_iter()
        .flat_map(|x| vec![x, x * 2])
        .collect();

    println!("{:?}", flattened); // 출력: [1, 2, 2, 4, 3, 6]
}
// option
fn main() {
    let values = vec![Some(1), None, Some(3), Some(4)];

    // Option을 평탄화하여 None은 제거하고 Some의 값을 추출
    let flattened: Vec<i32> = values
        .into_iter()
        .flat_map(|x| x) // or .flat_map(Option::into_iter)
        .collect();

    println!("{:?}", flattened); // 출력: [1, 3, 4]
}

//Result
fn main() {
    let results = vec![Ok(10), Err("Error"), Ok(20)];

    // Result를 평탄화하여 Ok 값만 추출
    let flattened: Vec<i32> = results
        .into_iter()
        .flat_map(Result::ok)
        .collect();

    println!("{:?}", flattened); // 출력: [10, 20]
}
