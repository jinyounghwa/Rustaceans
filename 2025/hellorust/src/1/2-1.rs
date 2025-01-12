// 면적을 계산할 수 있는 도형들을 위한 trait
trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &str;
}

struct Circle {
    radius: f64
}

struct Rectangle {
    width: f64,
    height: f64
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    
    fn name(&self) -> &str {
        "원"
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    
    fn name(&self) -> &str {
        "직사각형"
    }
}

// Shape trait을 구현한 어떤 타입이든 매개변수로 받을 수 있습니다
fn print_shape_info(shape: &impl Shape) {
    println!("도형: {}", shape.name());
    println!("면적: {:.2}", shape.area());
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle { width: 4.0, height: 6.0 };
    
    print_shape_info(&circle);
    print_shape_info(&rectangle);
}

/*
Trait 구현
impl Trait for Type 구문을 사용해 특정 타입에 대해 trait을 구현합니다
필수 메서드는 반드시 구현해야 합니다
기본 구현이 있는 메서드는 선택적으로 오버라이드할 수 있습니다
*/