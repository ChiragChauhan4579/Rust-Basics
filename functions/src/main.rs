fn main() {
    println!("Hello, world!");

    another_function();
    print_labeled_measurement(5,'h');

    let x = 1;
    let y = 2;
    let s = sum(x,y);
    println!("{s}");
}

fn another_function() {
    println!("Another function.");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}
