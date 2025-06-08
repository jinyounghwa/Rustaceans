#![allow(dead_code)]
use std::mem::size_of;

trait JustATrait{}

enum EnumOfNumbers{
    I8(i8),
    AnotherI8(i8),
    OneMoreI8(i8),
}

impl JustATrait for EnumOfNumbers{}

struct StructOfNumbers{
    an_i8: i8,
    another_i8: i8,
    one_more_i8: i8,
}

impl JustATrait for StructOfNumbers{}

enum EnumOfNumberTypes{
    I8(i8),
    AnotherI8(i8),
    Collection(Vec<String>),
}

struct StructOfNumberTypes{
    an_i8: i8,
    another_i8: i8,
    collection: Vec<String>,
}

impl JustATrait for StructOfNumberTypes{}

struct ArrayAndI8{
    an_array: [i8; 10000],
    an_i8: i8,
}

impl JustATrait for ArrayAndI8{}

fn returns_just_a_trait() -> Box<dyn JustATrait>{
    let some_enum = EnumOfNumbers::I8(8);
    Box::new(some_enum)
}

fn main(){
    println!("Size of EnumOfNumbers: {} bytes", size_of::<EnumOfNumbers>());
    println!("Size of StructOfNumbers: {} bytes", size_of::<StructOfNumbers>());
    println!("Size of EnumOfNumberTypes: {} bytes", size_of::<EnumOfNumberTypes>());
    println!("Size of StructOfNumberTypes: {} bytes", size_of::<StructOfNumberTypes>());
    println!("Size of ArrayAndI8: {} bytes", size_of::<ArrayAndI8>());
    println!("Size of Box<dyn JustATrait>: {} bytes", size_of::<Box<dyn JustATrait>>());
}