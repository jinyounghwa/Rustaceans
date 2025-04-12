struct Circle {
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        3.14 * self.radius * self.radius
    }
}

impl Circle {
    fn circumference(&self) -> f64 {
        2.0 * 3.14 * self.radius
    }
}

fn main() {
    let circle = Circle { radius: 10.0 };
    println!("Area: {}", circle.area());
    println!("Circumference: {}", circle.circumference());
}

// 구조체 Circle을 정의하고, area와 circumference 메서드를 구현합니다.
// main 함수에서 Circle 구조체의 인스턴스를 생성하고, area와 circumference 메서드를 호출하여 결과를 출력합니다.
// Circle 구조체는 원의 반지름을 나타내며, area 메서드는 원의 면적을 계산하고, circumference 메서드는 원의 둘레를 계산합니다.

// impl 키워드를 사용하여 Circle 구조체에 메서드를 추가합니다.
// Circle 구조체의 인스턴스를 생성할 때, radius 필드에 값을 할당합니다.
// area 메서드는 self.radius를 사용하여 면적을 계산하고, circumference 메서드는 self.radius를 사용하여 둘레를 계산합니다.

// main 함수에서 Circle 구조체의 인스턴스를 생성하고, area와 circumference 메서드를 호출하여 결과를 출력합니다.
// Circle 구조체는 원의 반지름을 나타내며, area 메서드는 원의 면적을 계산하고, circumference 메서드는 원의 둘레를 계산합니다.
// impl 키워드를 사용하여 Circle 구조체에 메서드를 추가합니다. 함수나 구조체를 정의할 때, impl 키워드를 사용하여 메서드를 추가할 수 있습니다.
// trait 키워드를 사용하여 Circle 구조체에 메서드를 추가합니다. trait 키워드를 사용하여 Circle 구조체에 메서드를 추가할 수 있습니다.
// struct 와 impl 키워드를 사용하여 Circle 구조체에 메서드를 추가합니다. struct와 impl 키워드를 사용하여 Circle 구조체에 메서드를 추가할 수 있습니다.