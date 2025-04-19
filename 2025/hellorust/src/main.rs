use std::collections::VecDeque;

fn check_remaining(input: &VecDeque<(&str, bool)>) {
    for item in input {
        if !item.1 {
            println!("You must: {}", item.0);
        }
    }
}

fn done(input: &mut VecDeque<(&str, bool)>) {
    if let Some(mut task_done) = input.pop_back() { // 뒤쪽에서 꺼냅니다.
        task_done.1 = true; // 완료 상태로 변경합니다.
        input.push_front(task_done); // 완료된 작업을 앞쪽에 다시 추가합니다.
    }
}

fn main() {
    let mut input = VecDeque::new();
    let things_to_do = vec!["send email to customer", "call supplier", "write report"];
    for thing in things_to_do {
        input.push_back((thing, false)); // false는 완료되지 않은 상태를 나타냅니다.
    }

    done(&mut input); // 완료된 작업을 처리합니다.
    done(&mut input); // 완료된 작업을 처리합니다.
    check_remaining(&input); // 남은 작업을 확인합니다.

    for task in &input {
        println!("Task: {}, Done: {}", task.0, task.1);
    }
}