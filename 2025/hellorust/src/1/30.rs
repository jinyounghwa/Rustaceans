use rand::*;
use rand::Rng;
use tokio;
use tokio::join;


async fn wait_and_give_u8(num:u8) -> u8 {
    let mut rng = rand:: thread_rng();
    let wait_time = rng.gen_range(0..100);
    tokio::time::sleep(std::time::Duration::from_millis(wait_time)).await;
    /***
     * 비동기적으로(wait) 지정된 시간만큼(랜덤 ms) 현재 작업을 멈춥니다.
tokio::time::sleep() 함수는 tokio 런타임에서 사용할 수 있는 비동기 타이머입니다.
std::time::Duration::from_millis(wait_time)는 wait_time 밀리초만큼의 시간(Duration 객체)을 만듭니다.
.await를 붙여서, 해당 시간만큼 비동기적으로 대기합니다.
즉, 이 함수가 실행되는 동안 다른 비동기 작업들은 계속 진행될 수 있습니다.
왜 필요한가?
여러 비동기 작업이 동시에 실행될 때, 각 작업이 서로 다른 시간만큼 랜덤하게 대기하도록 하여,
비동기 처리의 효과(동시에 여러 작업이 진행됨)를 실험할 수 있습니다.
실제로는 네트워크 요청, 파일 입출력 등에서 많이 사용됩니다.
예시
만약 wait_time이 50이라면,
이 라인은 "50밀리초 동안 대기"를 의미합니다.
하지만 이 대기는 비동기적으로 처리되므로, 다른 작업들이 블로킹되지 않습니다.
     */
    println!("Got a munber: {}", wait_time);
    num
}

#[tokio::main]
async fn main() {
    let mums = join!(
        wait_and_give_u8(1),    
        wait_and_give_u8(2),
        wait_and_give_u8(3),
        wait_and_give_u8(4),
        wait_and_give_u8(5),
    );
    println!("mums: {:?}", mums);
}