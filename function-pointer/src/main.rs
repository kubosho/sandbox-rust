fn main() {
    let f: fn(i32) -> i32 = plus_one;
    let six = f(5);
}

fn plus_one(i: i32) -> i32 {
    i + 1
}
