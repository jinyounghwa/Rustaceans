fn main(){
    let bool_vec = vec![true, false, true, false, true];

    let option_vec = bool_vec
    .iter()
    .map(|item|{
        item.then(||{
            println!("Got a {}!", item);
            "It's true!, you got it!"
        })
    })
    .collect::<Vec<_>>();
/**
bool_vec의 각 요소를 반복(iter)합니다.
각 요소에 대해, item이 true면 then 블록이 실행되어 println!으로 "Got a true!"를 출력하고, 문자열 "It's true!, you got it!"을 반환합니다.
item이 false면 None이 반환됩니다.
결과적으로 option_vec은 Option<&str> 타입의 벡터가 됩니다. (true면 Some("It's true!, you got it!"), false면 None)
*/

    println!("{:?}", option_vec);

    let filter_vec = option_vec
    .iter()
    .filter(|item|{
        item.is_some()
    })
    .collect::<Vec<_>>();
/*
option_vec을 반복하면서, 값이 Some인 것만 필터링합니다.
결과적으로 filter_vec은 Some("It's true!, you got it!")만 모은 Vec<&Option<&str>>가 됩니다.
*/

    println!("{:?}", filter_vec);

}