fn main() {
    // Ownership rules:
    // Each value in Rust has an owner.
    // There can only be one owner at a time.
    // When the owner goes out of scope, the value will be dropped.


    // ── Memory and Allocation ────────────────────────────────────────────────
    {   // stack-only data: Copy 
        let x = 5;        // stack-allocated integer
        let y = x;        // x is COPIED into y; both valid (i32 is Copy)
                        // a bit-for-bit duplicate yields two independent values

        // read-only data: &'static str
        let lit: &str = "hello"; // the binding `lit` (a fat pointer: ptr + len)
                            // lives on the STACK; the bytes "hello" live in
                            // the binary's .rodata section (read-only). The
                            // real type is &'static str: the bytes exist for
                            // the whole program, with no local owner.

        // heap data: Move / Clone
        let mut s = String::from("hello"); // String: a growable, heap-owning
                                        // structure. The header (ptr/len/cap)
                                        // lives on the stack; the bytes live
                                        // on the heap.
        s.push_str(" world");   // mutable, can grow

        // Analogies:
        // &str = &[u8]       str is [u8] with the UTF-8 invariant on top.
        // String = Vec<u8>   String is a Vec<u8> with additional methods and safety checks.
        // &String = &Vec<u8> pointer to a String's header in the stack.
    
        let s2 = s  // s is no longer valid; s2 is the new owner
            .clone();  // clone() creates a deep copy of the String, 
                       // so s is still valid (performance cost)
    
        s = String::from("ahoy"); // s is now "ahoy", and the previous String
                                  // "hello" is dropped (memory is freed)

        s.push_str(", world!"); // push_str() appends a literal to a String
        println!("{s}"); // this will print `ahoy, world!`
        println!("{s2}"); // this will print `hello`
    }


    // ── Ownership and functions ───────────────────────────────────────────
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

        // takes_and_gives_back, which also
        let s3 = takes_and_gives_back(s2); // s2 is moved into
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
        // Passing s1 by value moves it INTO the function, so s1 is no
        // longer usable out here. To get the String back, the function
        // hands it to us inside the tuple — s2 is now the owner.
        println!("The length of '{s2}' is {len}.");
    }

    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len(); // len() returns the length of a String

        (s, length)
    }


    // ── References and Borrowing (&: ampersand) ──────────────────────────────
    {
        let s1 = String::from("hello");

        let len = calculate_length_ref(&s1);        // pass a reference to s1
                                                    // s1 still owns the value
        println!("The length of '{s1}' is {len}."); // s1 is still valid here
    }

    fn calculate_length_ref(s: &String) -> usize {  // input is a reference to
        s.len()                                     // a String
    }// Here, s goes out of scope. But because s does not have ownership of
     //  what it refers to, the String is not dropped.

    // Note: opposite of referencing (&) is dereferencing (*)

    // Just as variables are immutable by default, so are references.


    // Mutable References
    {
        let mut s = String::from("hello");  // s must be mutable for its
                                            // reference to be mutable

        change(&mut s);     // pass mutable reference of s
                        // This makes it very clear that the change() 
                        // function will mutate the value it borrows.
        
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

    // PROP: you can have either only one mutable reference (&mut) or 
    // any number of immutable references (&), but never both at once.
    // let r1 = &s;     // no problem
    // let r2 = &s;     // no problem
    // let r3 = &mut s; // BIG PROBLEM

    // this PROPs prevent data races


    // Dangling References
    // If you have a reference to some data, the compiler will ensure
    // that the data will not go out of scope before the reference to
    // the data does.
    #[cfg(any())]  // conditional compilation to skip borrow checker
    {
        let reference_to_nothing = dangle();

        fn dangle() -> &String {    // dangle returns a reference to a String
            let s = String::from("hello");     // s is a new String
            &s     // we return a reference to the String, s
        } // Here, s goes out of scope and is dropped, so its memory goes away.
      // Danger! this reference would be pointing to an invalid String.
      // Rust won’t let us do this.
    }

    // The solution here is to return the String directly:
    fn no_dangle() -> String {
        let s = String::from("hello");
        s
    }

    // The Rules of References:
    // - At any given time, you can have either one mutable reference or 
    // any number of immutable references.
    // - References must always be valid.


    // The Slice Type
    // Slices let you reference a contiguous sequence of elements in a 
    // collection (heap data structures).

    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            // bytes.iter()  -> yields &u8 for each byte
            // .enumerate()  -> wraps it, yields (index, value): (usize, &u8)
            // (i, &item)    -> i is the usize index; &item destructures the
            //                  &u8 so item is a plain u8 (not a reference)
            // more info in more detail in Chapter 13.

            if item == b' ' {
                return i;
            }
        }

        s.len()
    }

    {
        let mut s = String::from("hello world");

        let word = first_word(&s); // word will get the value 5

        s.clear(); // this empties the String, making it equal to ""
    }
    // word still has the value 5 here, but s no longer has any content
    // that we could meaningfully use with the value 5, so word is now
    // totally invalid! word isn’t connected to the state of s at all!

    // Rust has a solution to this problem: 

    // String Slices
    // A string slice is a reference to a contiguous sequence of the 
    // elements of a String (&s[starting_index..ending_index])

    {
        let s = String::from("hello world");

        let hello = &s[0..5];  // equivalent to &s[..5]
        let world = &s[6..11]; // equivalent to &s[6..]
        // the slice data structure stores the starting position and 
        // the length of the slice, which corresponds to ending_index 
        // minus starting_index

        // Note: If you attempt to create a string slice in the middle 
        // of a multibyte character, your program will exit with an error.
        let s = String::from("café"); // 'é' requires 2 bytes in UTF-8

        match s.get(0..4) {
            Some(slice) => println!("valid index: {slice}"),
            None => println!("invalid index"),
        }
        println!("{}", s.len());     // 5, not 4 (counts BYTES)

        // let bad = &s[0..4];  // PANIC: index 4 falls in the middle of
                                // 'é' (bytes 3..5), not char boundary

        // safe slicing: returns Option<&str>, no panic
        match s.get(0..4) {
            Some(slice) => println!("ok: {slice}"),
            None => println!("invalid: index 4 splits 'é' (no panic)"),
        }
    }

    // The type that signifies “string slice” is written as &str.

    // the correct function to get the first word as a string slice:
    fn first_word_correct(s: &String) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    fn second_word(s: &String) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' { 
                for (j, &jtem) in bytes[i+1..].iter().enumerate() {
                    if jtem == b' ' {
                        return &s[i+1..i+1+j];
                    }
                }

                return &s[i+1..];
            }
        }

        &s[..]
    }

    // pro version:
    fn second_word_pro(s: &str) -> Option<&str> {
        s.split_whitespace().nth(1) // word in pos 1 (second one)
    }

    {
        let s = String::from("hola gordo puto");
        println!("sos un {}", second_word(&s));
    }


    // String Literals as Slices
    {
        let s = "Hello, world!";
        // The type of s here is &str: It’s a slice pointing to that 
        // specific point of the binary; &str is an immutable reference.
    }


    // String Slices as Parameters
    // Improving the first_word function by using a string slice for the 
    // type of the s parameter
    fn first_word_2(s: &str) -> &str {
        s // same as before
    }
    // Defining a function to take a string slice instead of a reference 
    // to a String makes our API more general and useful without losing 
    // any functionality
    {
        fn good(s: &str) { /* accepts String, literal, slice, everything */ }
        fn limited(s: &String) { /* ONLY accepts &String */ }
        let owned = String::from("hola");
        good(&owned);    // ok
        good("literal"); // ok
        limited(&owned); // ok
        // limited("literal"); // ERROR: a literal is not a &String
    }
    // In a nutshel, use &str, it's more general.


    // Other Slices
    {
        let a = [1, 2, 3, 4, 5];
        let slice = &a[1..3];
        assert_eq!(slice, &[2, 3]);
    }

    
    // ──Summary────────────────────────────────────────────────────────
    /* The concepts of ownership, borrowing, and slices ensure memory 
    safety in Rust programs at compile time. The Rust language gives 
    you control over your memory usage in the same way as other systems 
    programming languages. But having the owner of data automatically 
    clean up that data when the owner goes out of scope means you don’t 
    have to write and debug extra code to get this control. */
}   

