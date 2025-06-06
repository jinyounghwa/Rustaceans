// Arc 는 쓰레드에서 데이터를 공유할 수 있도록 해주는 타입

use std::sync::{Arc, Mutex};

fn main(){
    let my_number = Arc::new(Mutex::new(0));

    let my_number1 = Arc::clone(&my_number);
    let my_number2 = Arc::clone(&my_number);

    let thread1 = std::thread::spawn(move|| {
        // 복제본만 스레드 1로 이동합니다.
        for _ in 0..10{
            *my_number1.lock().unwrap() += 1; // Mutex를 사용하여 데이터를 안전하게 접근합니다. 역참조를 사용하여 (*) 접근합니다.
        }
    });
    let thread2 = std::thread::spawn(move|| {
        // 복제제본만 스레드 2로 이동합니다.
        for _ in 0..10{
            *my_number2.lock().unwrap() += 1; // Mutex를 사용하여 데이터를 안전하게 역참조를 사용하여 (*) 접근합니다.
        }
    });
    thread1.join().unwrap();
    thread2.join().unwrap();
    println!("Value is :{my_number:?}");
    println!("Exiting the program");
}