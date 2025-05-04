use std::collections::HashMap;

fn main(){
    let some_numbers = vec![1,2,3,4,5]; // 이것은 벡터입니다.
    let some_words = vec!["one", "two", "three", "four", "five"]; // 이것은 Vec<&str>입니다.

    let number_word_hasmap = some_numbers
    .into_iter() // 이 부분은 이터레이터 입니다.
    .zip(some_words.into_iter()) // .zip() 은 내부에 다른 이터레이터를 넣었습니다.
    .collect::<HashMap<_,_>>(); // 이제 이들은 함께 연결되었습니다.
    // .collect()는 이터레이터를 컬렉션으로 변환합니다.
    // HashMap은 키-값 쌍의 컬렉션입니다.

    println!("For key{} we get {}", 2, number_word_hasmap.get(&2).unwrap());
}

// fn main(){
//     let num_vec = vec![10,9,8]; // 벡터를 생성합니다.

//     num_vec.iter() // 이 부분은 이터레이터 입니다.
//     .enumerate() // 이 부분은 이터레이터 입니다.
//     .for_each(|(idx, &val)| 
//     // 이 클로저는 벡터의 각 요소에 대해 한 번씩 호출되며, 
//     // enumerate()로부터 각 요소의 인덱스와 값을 받아 처리합니다. 
//     // 즉, 벡터의 모든 요소를 순회하면서 각 요소의 인덱스와 값을 출력하는 작업을 수행합니다.

//     // Rust에서 클로저는 익명 함수로, 주변 환경의 변수를 캡처할 수 있는 특징이 있습니다. 
//     // 이 예제에서는 외부 변수를 캡처하지는 않지만, 이터레이터로부터 전달받은 값을 처리하는 간결한 방법을 제공합니다.
    
//     {
//         println!("{}: {}", idx, val);
//     });
// }