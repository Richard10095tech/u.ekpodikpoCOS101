fn main() {
    let (x, y) = (3, 5);
    let s: i32 = sum(x, y);

    assert_eq!(s, 8);
    println!("success!");
}

fn sum(x: i32,y: i32) -> i32{
    x + y
}