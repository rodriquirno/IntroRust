fn main() {
    // Variable
    let x = 5;
    println!("The value of x is: {x}");

    // type annotation
    let x = 5; // default type aca es i32
    let x: u8 = 5; 
    let x = 5_u8;
    

    // Mutability
    let mut y = 5;
    y = 6;
    println!("The value of y is: {y}");

    // Shadowing 
    let x = x + 1; 
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    // The value of x in the outer scope is still 6
    println!("The value of x is: {x}");

    // Constant
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds is: {THREE_HOURS_IN_SECONDS}");

    // Tuple
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let six_point_four = x.1;
    println!("The value of x_0 is: {}", x.0);
    println!("The value of x_1 is: {six_point_four}");
    println!("The value of x_2 is: {}", x.2);

    // Array
    let a = [1, 2, 3, 4, 5]; // let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("The first element of the array a is: {first}");
    println!("The second element of the array a is: {second}");

    let b = [3; 5];  // let b: [i32; 5] = [3, 3, 3, 3, 3];
    println!("The first element of the array b is: {}", b[0]);

    // Vector
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    println!("The first element of the vector is: {}", v[0]);

    // .pop() returns an Option<T>, which is Some(value) if the vector is not empty, and None if it is empty.
    // .unwrap() will return the value inside the Some variant, or panic if it is None.
    let last_in = v.pop().unwrap_or(0); // .unwrap_or(value) will return value if Option<T> is None.
    println!("The last element of the vector is: {last_in}");
}