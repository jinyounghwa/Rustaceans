use std::convert::From;
// 표준 라이브러리에서 From 트레이트를 가져옵니다.
// From 트레이트는 한 타입에서 다른 타입으로 변환을 정의하는 트레이트입니다.
struct EvenOddVec(Vec<Vec<i32>>);
// 튜플 구조체 EvenOddVec을 정의합니다.
// 이 구조체는 Vec<Vec<i32>> 타입의 단일 필드를 가집니다.
// 내부적으로 벡터의 벡터를 저장하는 구조입니다.
impl From<Vec<i32>> for EvenOddVec {
    fn from(input:Vec<i32>) -> Self {
        let mut even_odd_vec: Vec<Vec<i32>> = vec![vec![], vec![]];
        for item in input {
            if item % 2 == 0 {
                even_odd_vec[0].push(item);
            } else {
                even_odd_vec[1].push(item);
            }
        }
        Self(even_odd_vec)
    }
}
// 이 부분이 코드의 핵심으로, From 트레이트를 구현합니다:
// Vec<i32>에서 EvenOddVec으로 변환하는 방법을 정의합니다.
// from 메서드는 From 트레이트의 필수 메서드입니다.
// 구현 내용:
// even_odd_vec라는 빈 벡터 두 개를 포함하는 벡터를 생성합니다.
// 입력 벡터의 각 항목을 순회하면서:
// 짝수는 첫 번째 벡터(even_odd_vec[0])에 추가
// 홀수는 두 번째 벡터(even_odd_vec[1])에 추가
// 최종적으로 Self(even_odd_vec)를 반환하여 EvenOddVec 인스턴스를 생성합니다.
fn main(){
    let bunch_of_numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let new_vec = EvenOddVec::from(bunch_of_numbers);
    println!("Even numbers:{:?}\nOdd numbers:{:?}", new_vec.0[0], new_vec.0[1]);
}
// 1부터 10까지의 숫자를 포함하는 벡터를 생성합니다.
// EvenOddVec::from()을 호출하여 벡터를 EvenOddVec 타입으로 변환합니다.
// 결과를 출력합니다: 짝수 목록과 홀수 목록을 각각 표시합니다. 
