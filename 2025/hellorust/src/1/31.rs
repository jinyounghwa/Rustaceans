use std::fmt;
use std::ops::Add;
// Add 라이브러리 구현 예제입니다. 
#[derive(Clone)]
struct Country{
    name:String,
    population:u32,
    gdp:u32,
}
impl Country{
    fn new(name:&str, population:u32, gdp:u32) -> Self{
        Self{
            name:name.to_string(),
            population,
            gdp,
        }
    }
}
impl Add for Country{
    type Output = Self;

    fn add(self, other:Self) -> Self{
        Self{
            name:format!("{} and {}", self.name, other.name),
            population:self.population + other.population,
            gdp:self.gdp + other.gdp,
        }
    }
}
impl fmt::Display for Country{
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "{}: {} people, ${} GDP", self.name, self.population, self.gdp)
    }
}

fn main(){
    let china = Country::new("China", 1_067_800_000, 16_000_000);
    let india = Country::new("India", 1_300_000_000, 3_000_000);
    let combined = china.clone() + india.clone();
    println!("{}", combined);
    println!("china: name={}, population={}, gdp={}", china.name, china.population, china.gdp);
    println!("india: name={}, population={}, gdp={}", india.name, india.population, india.gdp);
}