// fn main() {
//     println!("Hello, world!");
// }
use std::any::type_name;
use std::mem::size_of_val;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    let x: i32 = 1;
    println!("{}", x);
    println!("{x}");

    let is_male: bool = false;
    let is_above_18: bool = true;

    println!("{is_male}, {is_above_18}");

    let greeting: String = String::from("hello world");
    println!("{}", greeting);

    let z: char = 'Z';
    println!("{z}");
    
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred: i32 = tup.0;
    println!("{five_hundred}");

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}",a[0]);
    let b: [i32; 5] = [3; 5];
    println!("{}",b[0]);

    println!("{tup:?}");

    let mut fees: i32 = 10000;
    println!("fees is {fees}");
    fees = 15000;
    println!("Fees increased to {fees}");

    let x: i32 = 5;
    println!("{}", type_of(x));

    let range_r: std::ops::Range<i32> = 1..5;
    println!("{range_r:?}");

    let size_range_r = size_of_val(&range_r);
    println!("{size_range_r}");

}


