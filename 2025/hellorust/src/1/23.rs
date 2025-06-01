use std::thread;

fn main() {
    let numbers = vec![10, 20, 30]; // 외부 환경의 변수

    let handle = thread::spawn(move || {
        for n in numbers { // numbers는 외부 환경에서 가져온 변수
            println!("숫자: {}", n);
        }
    });

    handle.join().unwrap();
}
//move는 클로저가 외부 환경의 변수를 캡처할 때, 그 변수를 소유권을 가져오도록(move) 지정합니다.
//이렇게 하지 않으면 numbers가 main함수가 끝나고 소멸되기 때문에 thread에서 numbers를 사용할 수 없습니다.
//따라서 move를 사용하여 numbers의 소유권을 thread로 이전합니다.
//handle.join().unwrap();은 thread가 완료될 때까지 main함수를 대기합니다.
//unwrap은 Result를 반환하는 함수에서 에러가 발생할 경우 프로그램을 중단합니다.
//이렇게 하지 않으면 thread가 완료되지 않았을 때 프로그램이 중단됩니다.
//이렇게 하면 thread가 완료될 때까지 main함수가 대기하지 않고 프로그램이 종료됩니다.

