fn main() {
    // ── Defining and Instantiating Structs ────────────────────────────────
    // Struct example
    struct User {
        // Fields:
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
    {
        // We create an instance of that struct
        let mut user1 = User {
            // We don’t have to specify the fields in the same order in which 
            // we declared them in the struct
            active: true,
            email: String::from("someone@example.com"), 
            username: String::from("someusername123"), 
            sign_in_count: 1,
        };
        
        // Note that the entire instance must be mutable to change a value
        user1.email = String::from("anotheremail@example.com");

        // Function that returns a struct instance
        fn build_user(email: String, username: String) -> User {
            User {
                active: true,
                username: username,
                email: email,
                sign_in_count: 1,
            }
        }
    }

    // Using the Field Init Shorthand 
    {
        // to avoid repetition when writing parameter names and the struct 
        // field names we can use the field init shorthand
        fn build_user2(email: String, username: String) -> User {
            User {
                active: true,
                username,
                email,
                sign_in_count: 1,
            }
        }
    }

    // Creating Instances with Struct Update Syntax 
    // to create a new instance of a struct that includes most of the 
    // values from another instance of the same type

    #[cfg(any())]
    {
        let user2 = User {
            active: user1.active,
            username: user1.username,
            email: String::from("another@example.com"),
            sign_in_count: user1.sign_in_count,
        };

        // With struct update syntax, we can achieve the same with less code
        let user2 = User {
            email: String::from("another@wexample.com"),
            ..user1 // must come last to specify that any remaining fields 
                // should get their values from the corresponding fields 
                // in user1
        }; // we can no longer use user1 after creating user2 because the 
        // String in the username field of user1 was moved into user2
    }

    // Creating Different Types with Tuple Structs 
    {
        // Tuple structs have the added meaning the struct name provides but 
        // don’t have names associated with their fields
        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);
        // Each struct you define is its own type

        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);

        let x = origin.0; // you can use a . followed by the index

        // tuple structs require you to name the type of the struct when 
        // you destructure them.
        let Point(x, y, z) = origin;
    }

    // Defining Unit-Like Structs
        // can be useful when you need to implement a trait on some type 
        // but don’t have any data that you want to store in the type 
        // itself
    {   
        struct AlwaysEqual;
        let subject = AlwaysEqual;
    }

    // It’s also possible for structs to store references to data owned by
    // something else, but to do so requires the use of lifetimes.
    // More on lifetimes in Chapter 10


    // ── An Example Program Using Structs ───────────────────────────────────
    {
        // let’s write a program that calculates the area of a rectangle.a
        #[derive(Debug)] // see line 135
        struct Rectangle{
            width: u32,
            height: u32,
        }

        fn area(rectangle: &Rectangle) -> u32 {
            rectangle.width * rectangle.height
        }

        let rect1 = Rectangle{
            height: 30,
            width: 50,
        };

        println!(
            "The area of the recctangle is {} square pixels",
            area(&rect1)
        );

        // Adding Functionality with Derived Traits / Debuging
        #[cfg(any())]
        println!("rect1 is {rect1}"); 
        // `Rectangle` doesn't implement `std::fmt::Display`

        // We have to explicitly opt in to print out debugging info
        // available for our struct. We add the outer attribute #[derive(Debug)]
        // before the struct
        println!("rect1 is {rect1:?}"); // :? output format called Debug}

        // Another way to print out a value using the Debug format is to use 
        // the dbg! macro
        dbg!(&rect1); // prints to the stderr as opposed to println! which 
                      // prints to the stdout. More info in I/O.

        let scale = 2;
        let rect2 = Rectangle {
            height: 50,
            width: dbg!(30 * scale), // because dbg! returns ownership of 
                                    // the expression’s value, the width 
                                    // field will get the same value as if
                                    // we didn’t have the dbg! call there.
        }; 
        // More Derivable Traits in Appendix C
        // More Attributes in Attributes section of the Rust Reference.
    }


    // ── Methods ──────────────────────────────────────────────────────────
    // Similar to functions, defined within the context of a struct, and
    // their first parameter is always self, which represents the instance
    // of the struct the method is being called on.
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    { 
                            // Everything within this impl block will be
        impl Rectangle {    // asociated with the Rectangle type
            fn area(&self) -> u32 {  // &self is actually short for self: &Self
                self.width * self.height
            } // We chose &self here because we don’t want to take ownership.
        }     // If we wanted to change the instance, we’d use &mut self.
              // A method that takes ownership using just self is rare;
              // usually used when the method transforms self into something 
              // else and you want to prevent the caller from using the 
              // original instance

        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        println!(
            "The area of the rectangle is {} square pixels.",
            rect1.area() // we can use method syntax to call the area method
        );

        // The main reason for using methods is for organization.
        
        // Note that we can choose to give a method the same name as one 
        // of the struct’s fields.
        impl Rectangle {
            fn width(&self) -> bool {
            self.width > 0
            }
        }

        if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
        }

        // We can aslo give a method the same name as a field and to 
        // return the value in the field. This are called "getters", which
        // are useful because you can make the field private but the 
        // method public.
    } 

    // Note: When you call a method with object.something(), Rust 
    // automatically adds in &, &mut, or * so that object matches the 
    // signature of the method. For example:
    // p1.distnace(&p2); is the same as (&p1).distance(&p2);


    // Methods with More Parameters
    {
        impl Rectangle {
            fn can_hold(&self, other: &Rectangle) -> bool {
                self.area() >= other.area()
            }
        }

        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        let rect2 = Rectangle {
            width: 20,
            height: 40,
        };

        println!("Its {} that Rectangle 2 fits in Rectangle 1", rect1.can_hold(&rect2));


    }

}
