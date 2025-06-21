use std::fs;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut file = fs::File::create("hello.txt")?; // ? 연산자는 작업이 성공했을때 결과값을 자동으로 전달합니다.
    file.write_all(b"Hello, world!")?;
    Ok(())
}
// 이것은 파일을 생성하고 파일에 문자열을 쓰는 코드입니다.