#[derive(Debug)]
struct Character{
    name:String,
    age:u8,
    height:u32,
    weight:u32,
    lifesate:LifeState,
}
#[derive(Debug)]
enum LifeState{
    Alive,
    Dead,
    NaverAlive,
    Uncertain,
}
impl Character{
    fn new(name:String,age:u8,height:u32,weight:u32, alive:bool)->Self{
        Self{
            name,
            age,
            height,
            weight,
            lifesate : if alive {
                LifeState::Alive
            }else{
                LifeState::Dead
            }
        }
    }

}
/**
여기서 Self는 Character 구조체 그 자체를 의미합니다.

즉, 위 함수의 반환 타입은 Character입니다.

Self { ... }는 Character { ... }와 동일합니다.

보통 impl Character 블록 안에서 Character를 반복하지 않기 위해 Self를 사용합니다.
*/
impl Default for Character{
    fn default()->Self{
        Self{
            name: "Billy".to_string(),
            age: 25,
            height: 180,
            weight: 70,
            lifesate: LifeState::Alive,
        }
    }
}
fn main(){
    let character = Character::default();
    println!(
        "The character {:?} is {:?} year old",
        character.name,
        character.age,
    );
}
/*
Default 트레이트는 std::default::Default에 정의되어 있는 표준 트레이트입니다.

구조체가 이 트레이트를 구현하면, .default() 메서드를 통해 기본값을 자동으로 생성할 수 있습니다.

여기서도 Self는 Character를 의미하고, Self { ... }는 Character { ... }와 같습니다.

👉 Character::default() 는 위에서 정의한 기본 속성을 갖는 Character 인스턴스를 반환합니다.
**/



/**
Character: 사람 같은 개체를 표현하는 구조체

LifeState: 생존 상태를 나타내는 열거형 (Alive, Dead 등)

Character::new(...): 생성자 함수

impl Default: Character::default()로 기본값을 만드는 기능

Self: 현재 구조체의 타입을 의미함 (Character와 동일)

*/