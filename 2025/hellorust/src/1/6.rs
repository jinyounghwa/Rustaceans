trait Animal {
    fn make_sound(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn make_sound(&self) {
        println!("멍멍!");
    }
}

impl Animal for Cat {
    fn make_sound(&self) {
        println!("야옹!");
    }
}

// &dyn Animal 또는 Box<dyn Animal> 사용
fn animal_sound(animal: &dyn Animal) { // 동적 디스패치
    animal.make_sound();
}

fn main() {
    let dog = Dog;
    let cat = Cat;

    animal_sound(&dog); // 참조 사용
    animal_sound(&cat);
}
