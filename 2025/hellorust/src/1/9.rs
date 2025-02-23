//상황 1: 작업 스케줄링 (Task Scheduling)
//설명: 작업 요청이 들어온 순서대로 처리해야 할 때 큐를 사용합니다. 
//예를 들어, 프린터가 여러 문서를 순서대로 인쇄하거나 서버가 요청을 처리할 때 유용합니다.
//활용 사례: 메시지 큐, 작업 대기열.

use std::collections::VecDeque;

fn main() {
    let mut task_queue: VecDeque<&str> = VecDeque::new();

    // 작업 추가
    task_queue.push_back("이메일 보내기");
    task_queue.push_back("보고서 작성");
    task_queue.push_back("회의 준비");

    println!("작업 대기열: {:?}", task_queue);

    // 작업 처리
    while let Some(task) = task_queue.pop_front() {
        println!("처리 중: {}", task);
    }

    println!("남은 작업: {:?}", task_queue); // 빈 큐
}