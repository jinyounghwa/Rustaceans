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
