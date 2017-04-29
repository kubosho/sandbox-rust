fn main() {
    shadowing1();
    shadowing2();
}

fn shadowing1() {
    println!("Shadowing1");

    let x = 8;
    {
        println!("{}", x);
        let x = 12;
        println!("{}", x);
    }
    println!("{}", x);
    let x = 42;
    println!("{}", x);
}

fn shadowing2() {
    println!("Shadowing2");

    let mut x = 8;
    x = 6;
    let x = x; // to immutable

    let y = 4;
    let y = "aaa"; // different type
}
