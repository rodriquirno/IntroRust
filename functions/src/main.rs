fn main() {
    another_function(5);

    print_labeled_measurement(5, "cm");

    println!("Function five() returns: {}", five());
}

fn another_function(x: u32) {
    println!("x value is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: &str) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

