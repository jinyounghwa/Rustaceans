
# Rust 초급 코딩 교재: 자판기 만들기 게임

## 소개

이 교재는 Rust 프로그래밍 언어를 처음 배우는 사람들을 위해 만들어졌습니다. 자판기 만들기 게임 프로젝트를 통해 Rust의 기본적인 문법과 개념을 익힐 수 있습니다.

## 준비물

*   Rust 개발 환경 (Rust 설치 방법은 [Rust 공식 웹사이트](https://www.rust-lang.org/tools/install)를 참고하세요.)
*   텍스트 편집기 또는 IDE (Visual Studio Code, IntelliJ IDEA 등)

## 1. 프로젝트 설정

### 1.1 새 프로젝트 생성

터미널을 열고 다음 명령어를 입력하여 새 Rust 프로젝트를 생성합니다.

### 1.2 Cargo.toml 설정

`Cargo.toml` 파일을 열고 다음과 같이 수정합니다.

```toml
[package]
name = "vending_machine"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8" # 랜덤 숫자 생성을 위한 라이브러리
```

`rand`는 랜덤한 값을 생성하기 위해 사용하는 외부 라이브러리입니다.

## 2. 기본 구조

### 2.1 `main.rs` 파일 수정

`src/main.rs` 파일을 열고 다음과 같이 코드를 작성합니다.

```rust
use std::io;
use rand::Rng;

// 자판기 구조체
struct VendingMachine {
items: Vec<Item>,
money: i32,
}

// 자판기 아이템 구조체
#[derive(Clone)]
struct Item {
name: String,
price: i32,
quantity: i32,
}

impl VendingMachine {
// 새 자판기 생성
fn new() -> VendingMachine {
    VendingMachine {
        items: Vec::new(),
        money: 0,
    }
}

// 아이템 추가
fn add_item(&mut self, name: String, price: i32, quantity: i32) {
    let item = Item {
        name: name,
        price: price,
        quantity: quantity,
    };
    self.items.push(item);
}

// 아이템 출력
fn display_items(&self) {
    println!("==========================");
    for (i, item) in self.items.iter().enumerate() {
        println!("{}. {} ({}원) - {}개", i + 1, item.name, item.price, item.quantity);
    }
    println!("==========================");
}

// 돈 투입
fn insert_money(&mut self, money: i32) {
    self.money += money;
    println!("현재 투입 금액: {}원", self.money);
}

// 아이템 구매
fn purchase_item(&mut self, item_index: usize) {
    if item_index >= self.items.len() {
        println!("잘못된 상품 번호입니다.");
        return;
    }

    let selected_item = self.items[item_index].clone();

    if selected_item.price > self.money {
        println!("잔액이 부족합니다.");
        return;
    }

    if selected_item.quantity == 0 {
        println!("해당 상품은 품절되었습니다.");
        return;
    }

    self.money -= selected_item.price;
    self.items[item_index].quantity -= 1;

    println!("{}을(를) 구매하셨습니다. 남은 금액: {}원", selected_item.name, self.money);
}

// 잔돈 반환
fn return_change(&mut self) {
    println!("잔돈 {}원을 반환합니다.", self.money);
    self.money = 0;
}
}

fn main() {
// 자판기 생성 및 아이템 추가
let mut vending_machine = VendingMachine::new();
vending_machine.add_item("콜라".to_string(), 1500, 5);
vending_machine.add_item("사이다".to_string(), 1200, 5);
vending_machine.add_item("커피".to_string(), 1000, 5);

loop {
    println!("자판기 메뉴:");
    vending_machine.display_items();
    println!("0. 잔돈 반환");
    println!("원하는 상품 번호를 입력하세요 (또는 0: 잔돈 반환):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("입력 실패");

    let choice: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("잘못된 입력입니다.");
            continue;
        }
    };

    match choice {
        0 => vending_machine.return_change(),
        _ => {
            if choice <= vending_machine.items.len() {
                println!("{}번 상품을 선택하셨습니다.", choice);

                println!("돈을 넣어주세요:");
                let mut money_input = String::new();
                io::stdin().read_line(&mut money_input).expect("입력 실패");

                let money: i32 = match money_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("잘못된 입력입니다.");
                        continue;
                    }
                };

                vending_machine.insert_money(money);
                vending_machine.purchase_item(choice - 1);
            } else {
                println!("잘못된 상품 번호입니다.");
            }
        }
    }
}
}
```

### 2.2 코드 설명

*   `struct VendingMachine`: 자판기 구조체로, 상품 목록(`items`)과 투입된 돈(`money`)을 관리합니다.
*   `struct Item`: 상품 구조체로, 이름(`name`), 가격(`price`), 수량(`quantity`)을 저장합니다.
*   `VendingMachine::new()`: 새로운 자판기를 생성합니다.
*   `VendingMachine::add_item()`: 자판기에 새로운 상품을 추가합니다.
*   `VendingMachine::display_items()`: 자판기의 상품 목록을 화면에 출력합니다.
*   `VendingMachine::insert_money()`: 돈을 투입합니다.
*   `VendingMachine::purchase_item()`: 상품을 구매합니다.
*   `VendingMachine::return_change()`: 잔돈을 반환합니다.
*   `main()` 함수: 프로그램의 메인 루프를 담당하며, 사용자 입력을 받아 자판기를 조작합니다.

## 3. 실행

터미널에서 다음 명령어를 입력하여 프로그램을 실행합니다.

```bash
cargo run
```

## 4. 추가 기능 (챌린지)

*   **상품 재고 관리**: 상품의 수량이 0이 되면 "품절" 메시지를 출력하고, 더 이상 구매할 수 없도록 합니다.
*   **잔돈 반환 기능 개선**: 투입된 금액이 상품 가격보다 많을 경우, 잔돈을 계산하여 반환합니다.
*   **관리자 모드**: 비밀번호를 입력하여 관리자 모드로 진입하고, 상품 추가/삭제, 가격 변경 등의 기능을 구현합니다.
*   **UI 개선**: 터미널 UI를 더 보기 좋게 꾸미거나, GUI 라이브러리를 사용하여 그래픽 인터페이스를 구현합니다.

## 5. 결론

이 교재를 통해 Rust 프로그래밍의 기본적인 문법과 개념을 익히고, 자판기 만들기 게임 프로젝트를 완성할 수 있습니다. 추가 기능 구현에 도전하여 더욱 깊이 있는 학습을 해보세요.