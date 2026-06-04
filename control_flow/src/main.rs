fn main() {
    // if statemnt
    let number = 3;

    if number < 5 {
        println!("number is less than 5");
    } else if number > 5 {
        println!("number is greater than 5");
    } else {
        println!("number is 5")
    }
    
    // Using if in a let Statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    // Looping
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // Loop labels
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining: u8 = 10;
      
        loop { 
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // while loops
    let mut number: u8 = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a: [i32; 5] = [10, 20, 30, 40, 50];
    let mut i = 0;

    while i < a.len() {
        println!("the {}-th value is: {}", i+1, a[i]);

        i += 1;
    }

    // for loops
    for element in a {
        println!("the value is : {element}")
    }

    for n in (1..4).rev() {
        println!("{n}!") 
    }
    println!("LIFTOFF!!!")
    
}

