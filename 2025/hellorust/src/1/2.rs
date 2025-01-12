// Animal이라는 trait을 정의합니다
trait Animal {
    // 반드시 구현해야 하는 메서드를 선언합니다
    fn make_sound(&self) -> String;
    
    // 기본 구현을 제공하는 메서드도 만들 수 있습니다
    fn description(&self) -> String {
        String::from("이것은 동물입니다")
    }
}

// Dog 구조체를 정의합니다
struct Dog {
    name: String
}

// Dog에 대해 Animal trait을 구현합니다
impl Animal for Dog {
    fn make_sound(&self) -> String {
        String::from("멍멍!")
    }
    
    fn description(&self) -> String {
        format!("{}라는 이름의 강아지입니다", self.name)
    }
}

// Cat 구조체를 정의합니다
struct Cat {
    name: String
}

// Cat에 대해 Animal trait을 구현합니다
impl Animal for Cat {
    fn make_sound(&self) -> String {
        String::from("야옹!")
    }
    
    // description을 오버라이드 할 수도 있습니다
    fn description(&self) -> String {
        format!("{}라는 이름의 고양이입니다", self.name)
    }
}

fn main() {
    let dog = Dog { name: String::from("멍멍이") };
    let cat = Cat { name: String::from("나비") };
    
    println!("강아지 소리: {}", dog.make_sound());
    println!("강아지 설명: {}", dog.description());
    
    println!("고양이 소리: {}", cat.make_sound());
    println!("고양이 설명: {}", cat.description());
}

/*
Trait 정의 (인스턴스와 비슷한 개념)
trait 키워드를 사용해 새로운 trait을 정의합니다
필수로 구현해야 하는 메서드는 본문 없이 선언만 합니다
기본 구현을 제공하고 싶은 메서드는 본문을 작성할 수 있습니다.
*/
