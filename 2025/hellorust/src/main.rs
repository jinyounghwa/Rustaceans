fn main(){
    let num_vec = vec![10,9,8];

    num_vec.iter().enumerate().for_each(|(idx, &val)| {
        println!("{}: {}", idx, val);
    });
}