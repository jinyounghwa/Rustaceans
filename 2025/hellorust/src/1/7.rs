/**
 * Rust 트레잇의 연관 타입 (Associated Type) 예제 설명
Rust 트레잇의 연관 타입은 트레잇을 구현하는 타입과 관련된 타입을 지정할 수 있도록 해주는 기능입니다. 
이를 통해 코드를 더욱 유연하고 재사용 가능하게 만들 수 있습니다.

연관 타입의 필요성
트레잇을 설계할 때 특정 타입을 사용해야 하는 경우가 있습니다. 
예를 들어, 반복자 (Iterator) 트레잇은 next() 메서드를 통해 다음 값을 반환해야 합니다. 
이때 반환되는 값의 타입은 반복자마다 다를 수 있습니다. 
연관 타입을 사용하면 트레잇을 구현하는 각 타입에 대해 반환 값의 타입을 지정할 수 있습니다.
 * 
 */

trait Iterator {
    type Item; // 연관 타입 Item 정의

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32; // Counter의 Item은 u32

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count > 5 {
            None
        } else {
            Some(self.count)
        }
    }
}

fn main() {
    let mut counter = Counter { count: 0 };
    for i in counter {
        println!("{}", i);
    }
}
/**
 * Iterator 트레잇은 Item이라는 연관 타입을 정의합니다.
Counter 구조체는 Iterator 트레잇을 구현하고, Item 타입을 u32로 지정합니다.
next() 메서드는 Option<Self::Item> 타입을 반환합니다. 여기서 Self::Item은 Counter에 대해 지정된 u32 타입입니다.
 * /