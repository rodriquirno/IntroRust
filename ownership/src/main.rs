fn main() {
    // Ownership rules:
    // Each value in Rust has an owner.
    // There can only be one owner at a time.
    // When the owner goes out of scope, the value will be dropped.


    // Memory and Allocation
    {   // stack-only data: copy
        let x = 5; // stack-allocated integer
        let y = x; // copy of x is made and stored in y

        // heap data: move, clone
        let s = "hello"; // s is a string literal
        let mut s = String::from("hello"); // s is now a String, which is a growable, heap-allocated data structure
    
        let s2 = s // s is moved to s2, and s is no longer valid
        .clone(); // clone() creates a deep copy of the String, so s is still valid (performance cost)
    
        s = String::from("ahoy"); // s is now "ahoy", and the previous String "hello" is dropped (memory is freed)

        s.push_str(", world!"); // push_str() appends a literal to a String
        println!("{s}"); // this will print `ahoy, world!`
        println!("{s2}"); // this will print `hello`
    }


    // Ownership and functions
    {
        let s = String::from("hello"); // s comes into scope
        takes_ownership(s);         // s's value moves into the function...
                                    // ... and so is no longer valid here

        let x = 5;                  // x comes into scope

        makes_copy(x);              // Because i32 implements the Copy trait,
                                    // x does NOT move into the function,
                                    // so it's okay to use x afterward.
    }// Here, x goes out of scope, then s. However, because s's value was moved,
    // nothing special happens.

    fn takes_ownership(some_string: String) { // some_string comes into scope
        println!("{some_string}");
    } // Here, some_string goes out of scope and `drop` is called. The backing
    // memory is freed.

    fn makes_copy(some_integer: i32) { // some_integer comes into scope
        println!("{some_integer}");
    } // Here, some_integer goes out of scope. Nothing special happens.


    // Return Values and Scope
    {
        let s1 = gives_ownership();        // gives_ownership moves its return
                                           // value into s1

        let s2 = String::from("hello");    // s2 comes into scope

        let s3 = takes_and_gives_back(s2); // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
    } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
    // happens. s1 goes out of scope and is dropped.

    fn gives_ownership() -> String {    // gives_ownership will move its
                                        // return value into the function
                                        // that calls it

        let some_string = String::from("yours"); // some_string comes into scope

        some_string                     // some_string is returned and
                                        // moves out to the calling
                                        // function
    }

    // This function takes a String and returns a String.
    fn takes_and_gives_back(a_string: String) -> String {
        // a_string comes into scope

        a_string  // a_string is returned and moves out to the calling function
    }


    // Returning Multiple Values Using Tuples
    {
        let s1 = String::from("hello");
        
        let (s2, len) = calculate_length(s1); 
        // let len = calculate_length(s1); would
        // give an error because s1 moved to the function
        println!("The length of '{s2}' is {len}.");
    }

    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len(); // len() returns the length of a String

        (s, length)
    }


    // References and Borrowing (&: ampersand)
    {
        let s1 = String::from("hello");

        let len = calculate_length_ref(&s1);        // pass a reference to s1
                                                    // s1 still owns the value
        println!("The length of '{s1}' is {len}."); // s1 is still valid here
    }

    fn calculate_length_ref(s: &String) -> usize {  // input is a reference to a String
        s.len()
    }// Here, s goes out of scope. But because s does not have ownership of what
    // it refers to, the String is not dropped.

    // Note: opposite of referencing (&) is dereferencing (*)

    // Just as variables are immutable by default, so are references.


    // Mutable References
    {
        let mut s = String::from("hello");  // s must be mutable for its
                                            // reference to be mutable

        change(&mut s);                     // mutable reference
    }

    fn change(some_string: &mut String) {
        // this function mutates the original value without the need of
        // having ownership.                                 
        some_string.push_str(", world"); // without *, why?
    }   // when you call a method with ".", Rust auto-dereferences.
        // more on dereferencig later...

    // PROP: You cannot borrow as mutable more than once at a time
    // let r1 = &mut s; // no problem
    // let r2 = &mut s; // BIG PROBLEM

    // PROP: you can have either one mutable reference (&mut) or 
    // any number of immutable references (&), but never both at once.
    // let r1 = &s;     // no problem
    // let r2 = &s;     // no problem
    // let r3 = &mut s; // BIG PROBLEM


}

