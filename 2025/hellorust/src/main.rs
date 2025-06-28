use rand::*;
use rand::Rng;
use tokio;
use tokio::join;


async fn wait_and_give_u8(num:u8) -> u8 {
    let mut rng = rand:: thread_rng();
    let wait_time = rng.gen_range(0..100);
    tokio::time::sleep(std::time::Duration::from_millis(wait_time)).await;
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