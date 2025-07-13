use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("사용법: {} [server|client]", args[0]);
        return Ok(());
    }

    match args[1].as_str() {
        "server" => run_server().await,
        "client" => run_client().await,
        _ => {
            eprintln!("알 수 없는 명령: {}", args[1]);
            eprintln!("사용법: {} [server|client]", args[0]);
            Ok(())
        }
    }
}

async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("서버가 127.0.0.1:8080에서 실행 중입니다.");

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("클라이언트 {}가 연결되었습니다.", addr);

        tokio::spawn(async move {
            let mut buf = vec![0; 1024]; // 1024바이트 버퍼
            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => { // 연결 종료
                        println!("클라이언트 {} 연결이 종료되었습니다.", addr);
                        return;
                    }
                    Ok(n) => {
                        let received_data = String::from_utf8_lossy(&buf[..n]);
                        println!("클라이언트 {}로부터 {} 바이트 수신: {}", addr, n, received_data);

                        // 받은 데이터를 다시 클라이언트로 전송 (에코)
                        if let Err(e) = socket.write_all(&buf[..n]).await {
                            eprintln!("클라이언트 {}로 데이터 전송 실패: {}", addr, e);
                            return;
                        }
                        println!("클라이언트 {}로 {} 바이트 전송 (에코).", addr, n);
                    }
                    Err(e) => {
                        eprintln!("클라이언트 {}로부터 데이터 읽기 실패: {}", addr, e);
                        return;
                    }
                }
            }
        });
    }
}

async fn run_client() -> Result<(), Box<dyn std::error::Error>> {
    println!("서버에 연결 중...");
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    println!("서버에 연결되었습니다.");

    let messages = vec!["Hello, Tokio!", "How are you?", "Goodbye!"];

    for msg in messages {
        println!("서버로 메시지 전송: {}", msg);
        stream.write_all(msg.as_bytes()).await?;

        let mut buffer = vec![0; 1024];
        match stream.read(&mut buffer).await {
            Ok(0) => {
                println!("서버가 연결을 종료했습니다.");
                break;
            }
            Ok(n) => {
                let echoed_msg = String::from_utf8_lossy(&buffer[..n]);
                println!("서버로부터 에코된 메시지 수신: {}", echoed_msg);
            }
            Err(e) => {
                eprintln!("서버로부터 데이터 읽기 실패: {}", e);
                break;
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await; // 1초 대기
    }

    Ok(())
}
/***
1. use 선언을 통해 가져온 것들
tokio::io::{AsyncReadExt, AsyncWriteExt}: 이들은 비동기 읽기/쓰기 작업을 위한 트레이트입니다.

AsyncReadExt는 read() 같은 비동기 읽기 함수를 제공합니다.

AsyncWriteExt는 write_all() 같은 비동기 쓰기 함수를 제공합니다.

tokio::net::TcpListener: TCP 연결을 수신 대기하고 받아들이는 데 사용되는 구조체입니다.

2. main 함수와 관련된 것들
#[tokio::main]: 이것은 Rust 매크로로, main 함수를 비동기 컨텍스트에서 실행할 수 있도록 Tokio 런타임을 초기화하고 실행합니다.

async fn main() -> Result<(), Box<dyn std::error::Error>>:

async fn: 이 함수가 비동기 함수임을 나타냅니다. await 키워드를 사용할 수 있게 해줍니다.

Result<T, E>: Rust의 표준 라이브러리에 있는 열거형으로, 성공(Ok) 또는 실패(Err)를 나타냅니다. 에러 처리에 사용됩니다.

Box<dyn std::error::Error>: Rust의 표준 라이브러리에 있는 스마트 포인터이자 트레이트 객체입니다.

Box: 데이터를 힙에 할당하고 소유권을 가집니다.

dyn std::error::Error: 다양한 종류의 에러를 일반적인 Error 트레이트 객체로 처리할 수 있도록 합니다. 즉, 다형성을 위해 사용됩니다.

3. 서버 로직 내에서 사용된 주요 함수 및 트레이트
TcpListener::bind("127.0.0.1:8080").await:

TcpListener::bind(): TcpListener 구조체의 연관 함수(정적 메서드)로, 지정된 주소에 서버를 바인딩합니다. 이 함수 자체는 Future를 반환합니다.

.await: 비동기 연산의 결과를 기다리게 하는 키워드입니다. Future를 폴링하여 완료될 때까지 실행을 일시 중지합니다.

listener.accept().await:

listener.accept(): TcpListener의 메서드로, 들어오는 연결을 비동기적으로 기다립니다. 이 또한 Future를 반환합니다.

.await: 위와 동일하게 비동기 연산 완료를 기다립니다.

tokio::spawn(async move { ... }):

tokio::spawn(): tokio 크레이트의 함수로, 주어진 비동기 블록(async move { ... })을 새로운 비동기 태스크로 스케줄링하여 병렬로 실행되게 합니다.

async move { ... }: 이 클로저는 비동기적으로 실행될 태스크를 정의하며, move 키워드는 클로저 내에서 외부 변수(socket, addr)의 소유권을 가져오도록 합니다.

socket.read(&mut buf).await:

socket.read(): TcpStream (더 정확히는 AsyncReadExt 트레이트)의 메서드로, 소켓으로부터 비동기적으로 데이터를 읽어 buf에 채웁니다.

.await: 비동기 읽기 작업 완료를 기다립니다.

String::from_utf8_lossy(&buf[..n]):

String::from_utf8_lossy(): String 구조체의 연관 함수로, 바이트 슬라이스를 UTF-8 문자열로 변환합니다. 유효하지 않은 UTF-8 바이트 시퀀스는 대체 문자로 변환됩니다.

socket.write_all(&buf[..n]).await:

socket.write_all(): TcpStream (더 정확히는 AsyncWriteExt 트레이트)의 메서드로, 제공된 바이트 슬라이드(&buf[..n])의 모든 내용을 비동기적으로 소켓에 씁니다.

.await: 비동기 쓰기 작업 완료를 기다립니다.

eprintln!(): Rust의 매크로로, 표준 에러 스트림에 메시지를 출력합니다.

/