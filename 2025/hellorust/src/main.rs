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
