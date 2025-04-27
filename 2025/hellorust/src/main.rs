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