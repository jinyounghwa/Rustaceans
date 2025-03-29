enum ThingInTheSky {
    Sun(String),
    Stars,
}
fn create_skystate(time:i32) -> ThingInTheSky {
    match time {
        6..18 => ThingInTheSky::Sun(String::from("The sun is shining!")),
        _ => ThingInTheSky::Stars(String::from("The stars are shining!")),
    }
}
fn check_skystarte(state:&ThingInTheSky) {
    match state{
        ThingInTheSky::Sun(msg) => println!("{}", msg),
        ThingInTheSky::Stars => println!("I can see the stars!"),
    }
}
fn main() {
    let time = 8;
    let state = create_skystate(time);
    check_skystarte(&state);
}