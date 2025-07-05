macro_rules! check{ // 매크로 이름
    ($input1:ident, $input2:expr) => {  
        if $input1 > $input2 {
            println!("{} is greater than {}", $input1, $input2);
        } else {
            println!("{} is less than {}", $input1, $input2);
        }
    };
}

fn main() {
    let a = 10;
    let b = 20;
    check!(a, b);
}