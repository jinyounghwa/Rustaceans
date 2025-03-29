fn main() {
    // loop 예시: 무한 루프 후 break
    let mut count = 0;
    loop {
        count += 1;
        if count > 5 {
            break;
        }
        println!("loop count: {}", count);
    }

    // while 예시: 조건 기반 루프
    let mut num = 1;
    while num <= 5 {
        println!("while num: {}", num);
        num += 1;
    }
}