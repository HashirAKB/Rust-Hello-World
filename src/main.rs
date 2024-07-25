use std::fs;

fn main() {
    println!("Hello, world!");

    let x = -5; //signed int, only -ve.
    let y: u32 = 1000; //Unsigned int, only +ve.
    let z: f32 = 1000.00; //Floating point, can be both -ve and +ve.

    //By default rust'll infer the type, if you don't give one explicitly.

    println!("x:{}", x);
    println!("y:{}", y);
    println!("z:{}", z);

    // let mut a: i8 = 5; //Mutable variable

    //BOOLEANS
    let is_male: bool = true;
    let is_above_18: bool = true;

    if is_male {
        println!("You're a male");
    } else {
        println!("You're not a male");
    }

    if is_male && is_above_18 {
        println!("You're a legal male");
    }

    /*
    Primitive Types in Rust:
    1. string --> Hard
    2. integers (floats, and integers) --> Easy
    3. booleans --> Easy

    STRINGS
    Strings are a bit tricky, especially when the data/stored value changes. When a variable is declared and stored only the amount that needed to contain that amount of string/information is allocated in the memory. So if the string length get changes or becomes bigger it'll lead to memory issues. Stack and heap are the solutions here.

    Using character matching we can run this code. ie, if the char is not present at the given string it'll print the message associated with the run.

    let message = String::from("This is a string");
    println!("{}", message);

    let char1 = message.chars().nth(2);
    match char1 {
        Some(c) => println!("{}", c),
        None => println!("No characters at given index"),
    }
    println!("{}", char1);
     */

    /*CONDITIONALS & LOOPS */
    let is_even = true;
    if is_even {
        println!("The number is even.");
    } else {
        println!("The number is odd.")
    }

    for i in 0..11 {
        println!("{}", i);
    }

    //Iterating over arrays, maps, and strings.
    fn get_first_word(sentence: String) -> String {
        let mut ans = String::from("");
        for char in sentence.chars() {
            ans.push_str(char.to_string().as_str());
            if char == ' ' {
                break;
            }
        }
        return ans;
    }

    let sentence = String::from("Hashir Ahmed K B");
    let first_name = get_first_word(sentence);
    println!("The first name is {}", first_name);

    for i in 0..11 {
        println!("This is i at: {}", i);
    }

    /*MEMORY MANAGEMENT IN RUST
    Rust don't use a gargabe collector like in JS or Java. Rust has it's own ownership model for memory management. It make'sit extremely safe to memory errors. Not having a garbage collector is one of the key reasons rust is so fast. It achieves this by:
        1. Mutability
        2. Heap and Memory.
        3. Ownership Model.
        4. Borrowing and references.
        5. Lifetimes.


    MUTABILITY:
        immutable are the ones whose values can't be changed once it's assigned. By default all the variables in rust are immutable. Because mutability leads to many memory issues. tu update a varible value in rust, we need to use the keyword 'mut'.

        let x; let mut x;

        Immutable data is inherently thread safe because if no thread can alter the data , then no synchronization is needed when data accessed concurrently. Knowing that certain data won't change allows the compiler to optimize the code better.

    STACK vs HEAP:
        Stack: If rust is running a code or program in a system, it'll ask for some space in memory/RAM in which it'll store all the variables or other things of the program. For ex, if there's a variable 'a' then it'll be pushed to the bottom of the stack(The allocated memory space).

        But there's a conflict when we declare a string like variales at compile time which might get change during run time. The compiler can only allocate the memory for the given string during the compile time, but if the value changes to something big than the given memory space during the run time it'll run into issues. Here's 'Heap' comes into picture.

        Because we don't know the size of these strings, Instead of placing these dynamically changing strings or vectors directly into the Stack, we use 'Heap' to store this items and stack'll point to these heaps. The actual value of these variables'll be stored in heaps.

        Stack is much faster than the heap because the program needs to ask the OS each time to use heap to store some data. Everything that's size can be predictable during compile time goes into stack and the rest into the heap.
    */

    fn stack_fn() {
        let a = 10;
        let b = 20;
        let c = a + b;
        println!("Stack Fn: The sum of {} and {} is {}.", a, b, c);
    }

    fn heap_fn() {
        let s1 = String::from("Hello ");
        let s2 = String::from("World");
        let combined = format!("{} {}", s1, s2);
        println!("Heap Fn: The combined string is '{}'", combined);
    }

    fn update_string() {
        let mut s = String::from("Initial String.");
        println!("Before Update: {}", s);
        println!(
            "Capacity: {}, Length: {}, Pointer: {:p}",
            s.capacity(),
            s.len(),
            s.as_ptr()
        );

        for _ in 0..100 {
            s.push_str("And Some additional text");
            println!(
                "Capacity: {}, Length: {}, Pointer: {:p}",
                s.capacity(),
                s.len(),
                s.as_ptr()
            );
        }
    }

    //Calling these Fns:
    stack_fn();
    heap_fn();
    update_string();

    /*
    OWNERSHIP IN RUST:
        Each piece of data in Rust has a single owner at any given time, when the owner goes out of scope, rust automatically deallocated the memory associated with that data, preventing memory leaks.

        Dangling Pointer: A pointer that exists which points to a data which doesn't exist anymore.
        Double Free Error: A pointer getting allocated or deallocated by another program.

        Rust fix these two errors mentioned above, These errors occur due to multiple parties are owning the heap. If any one of them dies/goes out of scope, the heap get's deallocated. The variale related to the data contained in the heap owns that portion of the heap. It won't stay in allocation if the variable dies or goes out of scope.

        For ex:
            s1 = aaa;
            s2 = s1;
            On the second assignment the heap data (aaa) ownership'll come into s2 and s1'll die or go out of the scope. We can not use s1 anymore. In rust, there'll be only one owner at a time. This is how ownership works in rust. The heap'll move on with s2. if anyone try to access/print the s1 it'll get error thrown.
            This makes rust extremely memory safe.

        Another way of moving this ownership:
     */

    fn takes_ownership(some_string: String) {
        //This now owns the data.
        println!("{}", some_string);
    }

    let sample_string = String::from("This is the sample string.");
    takes_ownership(sample_string);
    // println!("{}", sample_string);
    /*
    --> Throws error 'borrowed value' as the fn above takes the ownership of the heap data associated with the sample_string value and hence the sample_string variable goes out of scope.
    But we can use .clone fn if we need to clone the same data into a separate heap and give it to the fn. But cloning is not advisable always as it's a uses more memory/space.

    We can return the value from the fn to the main fn into a mut variable, in that case ownership'll get retransfered back. But there's a better method: references.

    BORROWING/ REFERENCES:
        Borrowing means Referencing or giving the address of a string rather than the ownership of the string over to a function.
    */
    fn takes_ownership_by_borrowing(some_string: &String) {
        //This now owns the data.
        println!("This string is borrowed: {}", some_string);
    }

    let mut sample_string = String::from("Hanky Panky");
    takes_ownership_by_borrowing(&sample_string);
    println!("But I'm the owner MF! - {}", sample_string);

    /* What about changing the values or updating it after borrowing? */
    fn takes_ownership_by_borrowing2(some_string: &mut String) {
        //This now owns the data.
        some_string.push_str("Oh Yeeeeah!!");
        println!("This string is borrowed: {}", some_string);
    }
    takes_ownership_by_borrowing2(&mut sample_string);
    // This is how we can pass a variable mutably into a fn so that the other fn can update the variable.

    //Speaking about rules in borrowing:

    fn update_word(word: &mut String) {
        word.push_str(" world");
    }

    let mut s1 = String::from("hello ");
    let s2 = &mut s1;
    s2.push_str(" world");
    // update_word(&mut s1); //Throws error here,
    //You can have a owner, a reference, But you can't have a 3rd mutable or immutable reference after that. If it's a immutable reference then it's fine, can be used at multiple places.

    // println!("{}", s1); // Throws error here, uncomment and check here again to see it.
    println!("{}", s2);

    /*
       1. There can be only one mutable reference at a time.
       2. If there's a mutable reference, you can't have another immutable reference either.
       #Todo:
        - Checkout String Slices, and LifeTimes.

    THE SLICE TYPE:
        Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership.
        https://doc.rust-lang.org/book/ch04-03-slices.html

    String Slices:
        A string slice is a reference to part of a String, and it looks like this:
    */
    let s = String::from("hello world");

    let _hello = &s[0..5];
    let _world = &s[6..11];

    /*
    With Rust’s .. range syntax, if you want to start at index 0, you can drop the value before the two periods. In other words, these are equal:
     */
    let s = String::from("hello");

    let _slice = &s[0..2];
    let _slice = &s[..2];

    // By the same token, if your slice includes the last byte of the String, you can drop the trailing number. That means these are equal:
    let s = String::from("hello");

    let len = s.len();

    let _slice = &s[3..len];
    let _slice = &s[3..];

    // You can also drop both values to take a slice of the entire string. So these are equal:
    let s = String::from("hello");
    let len = s.len();

    let _slice = &s[0..len];
    let _slice = &s[..];

    //String slices, as you might imagine, are specific to strings. But there’s a more general slice type too. Consider this array:
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);

    /*
    Validating References with Lifetimes:
        Every reference in Rust has a lifetime, which is the scope for which that reference is valid. The main aim of lifetimes is to prevent dangling references, which cause a program to reference data other than the data it’s intended to reference.

    Erroneous code example:

    struct Foo<'a> {
        x: Option<&'a u32>,
    }

    let mut x = Foo { x: None };
    {
        let y = 0;
        x.x = Some(&y); // error: y does not live long enough
    }
    println!("{:?}", x.x);

    Here, y is dropped at the end of the inner scope, but it is borrowed by x until the println. To fix the previous example, just remove the scope so that y isn't dropped until after the println

    struct Foo2<'a> {
        x: Option<&'a u32>,
    }
    let mut x = Foo { x: None };
    let y = 0;
    x.x = Some(&y);

    println!("{:?}", x.x);
    */
    //EXAMPLE:
    struct Backpack<'a> {
        snack: Option<&'a str>,
    }

    /* This code doesn't work:
    fn main2() {
        let mut my_backpack = Backpack { snack: None };
        {
            let apple = String::from("Apple");
            my_backpack.snack = Some(&apple);
        } // apple is destroyed here
        println!("My snack is: {:?}", my_backpack.snack);
    }
    */

    // This code works:
    fn main3() {
        let mut my_backpack = Backpack { snack: None };
        let apple = String::from("Apple");
        my_backpack.snack = Some(&apple);
        println!("My snack is: {:?}", my_backpack.snack);
    }

    /*
    Let's break this down:

        1. We have a `Backpack` that can hold a reference to a snack (a string).

        2. In the first example (which doesn't work):
            - We create a backpack.
            - Inside a smaller scope (the `{}` brackets), we create an apple.
            - We put a reference to the apple in our backpack.
            - We leave this scope, and the apple is destroyed.
            - We try to check what snack is in our backpack, but the apple is gone!

        3. In the second example (which works):
            - We create a backpack.
            - We create an apple in the same scope as the backpack.
            - We put a reference to the apple in our backpack.
            - We can successfully check what snack is in our backpack because the apple still exists.

        The key difference is how long the apple exists. In the first example, it's like putting a real apple in your backpack, but then the apple magically disappears before you can eat it. In the second example, the apple stays around as long as your backpack does, so you can always check what snack you have.

        This is a simplified version of what's happening in the original code. The Rust compiler ensures that we don't accidentally refer to things that no longer exist, which helps prevent a whole class of common programming errors.
    */

    /*
    STRUCTS
        structs lets you structure data together, similar to objects in JS.
        #Todo:
        - Complete the rest of the coding.
        - Checkout  copy trait, and clone trait.
     */
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let user1 = User {
        active: true,
        username: String::from("HashirAKB"),
        email: String::from("hashirakb@gmail.com"),
        sign_in_count: 1,
    };
    println!("User 1 username is {}", user1.username);

    /*STRUCT IMPLEMENTATION */
    struct Rect {
        width: u32,
        height: u32,
    }

    impl Rect {
        fn area(&self) -> u32 {
            return self.width * self.height;
        }
        fn perimeter(&self) -> u32 {
            return 2 * (self.width + self.height);
        }
    }

    let rectangle = Rect {
        height: 30,
        width: 50,
    };

    println!("The area of the rectangle is {}", rectangle.area());
    println!(
        "The perimeter of the rectangle is {}",
        rectangle.perimeter()
    );

    /*ENUMS IN RUST */
    enum Direction {
        North,
        East,
        West,
        South,
    }
    let my_direction = Direction::South;
    move_arount(my_direction);
    fn move_arount(_my_direction: Direction) {
        // println!("User should move to {} direction", my_direction);
    }

    /* PATTERN MATCHING IN RUST */
    //ENUMS with values:
    enum Shape {
        Circle(f64),
        Square(f64),
        Rectangle(f64, f64),
    }

    let circle = Shape::Circle(50.0);
    let square = Shape::Square(30.0);
    let rectangle = Shape::Rectangle(25.0, 30.0);

    fn calculate_area(shape: Shape) -> f64 {
        let ans = match shape {
            Shape::Circle(radius) => 3.14 * radius * radius,
            Shape::Rectangle(width, height) => width * height,
            Shape::Square(side) => side * side,
        };
        return ans;
    }

    println!("Area of circle: {}", calculate_area(circle));
    println!("Area of square: {}", calculate_area(square));
    println!("Area of rectangle: {}", calculate_area(rectangle));

    /* ERROR HANDLING IN RUST
        Run time error might can occur with rust for example if a file is not there in the system which the program is trying to get access. The way rust does error handling is slightly different.

        Generics:
        enum Result<A, B>{
        x: A,
        y: B,
        z: B,
        }

        enum Result<A, B>
        {
            Ok(A),
            Err(B),
        }
    */

    let response = fs::read_to_string("/notAnExistingFile");
    match response {
        Ok(content) => {
            println!("File Content: {}", content);
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
    // By using this we're gracefully handling the error and the thread won't crash also.
    // The above is the implementation of try-catch method in rust.

    /* OPTION ENUM IN RUST
        This is used to get ridd of null type through out the entire codebase. Rust don't have a concept of null because it might cause a lot of runtime errors. Option enum handles the concept of nullability in rust.
    */
    pub enum Option<T> {
        None,
        Some(T),
    } // If there's a fn which returns null, return an Option instead.

    fn find_first_a(s: String) -> Option<i32> {
        for (index, character) in s.chars().enumerate() {
            if character == 'a' {
                return Some(index as i32);
            }
        }
        return None;
    }

    fn main() {
        let my_string = String::from("raman");
        match find_first_a(my_string) {
            Some(index) => println!("The letter 'a' is found at index: {}", index),
            None => println!("The letter 'a' is not found in the string."),
        }
    }

    /*
    CARGO:
        Cargo is the package manager for rust similar to what NPM is for node.
        Cargo can be used to bring 'crates' (external packages) into rust.
        What all libraries does rust have?
            A lot of them
            https://actix.rs/ - Extremely fast http server
            https://serde.rs/ - Serializing and deserialising data in rust
            https://tokio.rs/ - Asynchronous runtime in rust
            https://docs.rs/reqwest/latest/reqwest/ - Send HTTP requests
            https://docs.rs/sqlx/latest/sqlx/ - Connect to sql database
        - crates.io
     */

    /*
    Leftovers - Collections, Traits, Generics and Lifetimes, Multithreading, macros, async ops (Futures)
        https://doc.rust-lang.org/book/ch10-00-generics.html
        https://doc.rust-lang.org/book/ch19-00-advanced-features.html
        https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html
        https://doc.rust-lang.org/std/future/trait.Future.html

     */
}
