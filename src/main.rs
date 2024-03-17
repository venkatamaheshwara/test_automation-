use test_automation::add;
use test_automation::sub;
fn main() {
    println!("Hello, world!");
    let a=10;
    let b=20;
    let c=add(a,b);
    let d=sub(a,b);
    println!("addition of {a} and {b} = {c}");
    println!("subtraction of {a} and {b} = {d}");
}
