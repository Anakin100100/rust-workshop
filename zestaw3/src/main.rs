fn x(a: i32) -> impl Fn() -> i32 {
    return move || a + 5;
}

fn main() {
    let c = x(7);

    println!("{}", c());
}
