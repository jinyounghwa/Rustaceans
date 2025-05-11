fn main(){
    let new_vec = vec![1,2,3];

    let double_vec = new_vec
    /*
    *- `iter`는 소유권 이전 반복자를 생성합니다.
    *- `inspect`는 디버깅용 메서드입니다.
    *- 각 요소를 클로저로 받아 **변형 없이 중간에서 무언가를 수행**할 수 있습니다.
    *- 이 클로저의 인자는 `&i32` → `first_item`의 타입은 `&&i32`입니다.
    **/
    .iter()
    .inspect(|first_item| {
        println!("The item is:{first_item}");
        /**
        *- `inspect`는 디버깅용 메서드입니다.
        *- 각 요소를 클로저로 받아 **변형 없이 중간에서 무언가를 수행**할 수 있습니다.
        *- 이 클로저의 인자는 `&i32` → `first_item`의 타입은 `&&i32`입니다.
        */
        match **first_item*2{
            /*
            *- `match`는 패턴 매칭을 사용하여 값을 비교합니다.
            *- `_`는 모든 다른 값을 나타냅니다.
            *- `first_item`은 `&i32` 타입이므로 `**first_item`을 사용하여 `i32`로 변환합니다.
            *- `*first_item`을 사용하여 `i32`로 변환합니다.
            *- **은 참조를 제거합니다.
            **/
            0 => println!("The item is 0"),
            1 => println!("The item is 1"),
            2 => println!("The item is 2"),
            _ => println!("The item is not 0, 1, or 2"),
        }
        println!("The item is:{first_item}");
    })
    .map(|x| x*2)
    .collect::<Vec<_>>();
    /*
    *- `map`은 각 요소를 변환하는 메서드입니다.
    *- `collect`는 결과를 벡터로 수집합니다.
    */
}