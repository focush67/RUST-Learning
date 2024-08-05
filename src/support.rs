#![allow(unused_variables, dead_code)]

pub fn issue_greeting(name: &str) -> String {
    format!("Hello {} ", name)
}

// Variables can be mutable in one scope but immutable in another. Once a mutable variable goes out of scope, any re-declaration of that variable will need a new mut declaration if mutability is required.

pub fn shadowing_and_scope() {
    let mut a = 5;
    {
        let a = a; // Shadowing, a is now immutable
                   // a = 6; // Error: cannot assign to `a` because it is immutable
        println!("Here is a inside immutable scope {}", a);
    }
    a = 6; // This is allowed because we are back to the outer mutable scope
    println!("The value of a is: {}", a);
}

//By default, function parameters are immutable. If you want to change the value of a parameter, you must make it mutable within the function's scope.

pub fn mutability_and_dereferencing() {
    let mut z = 10;
    println!("The value of z before change is {}", z);
    change_value(&mut z);
    println!("The value of z after change is: {}", z);
}

pub fn change_value(val: &mut i32) {
    *val = 20; // Dereferencing and changing the value
}

// Basic increment function

pub fn increment(n: &mut i32) {
    *n += 2;
}

// &str is immutable, but String is mutable

pub fn mutable_and_immutable_string() {
    let mut string: String = String::from("This is an mutable sentence");
    println!("{}", string);

    string.push_str(" Yup Yup");
    println!("New ,{}", string);
}

// Destructuring

pub fn tuple_destructuring() {
    let emp_info: (u8, bool, &str) = (12, true, "Lokesh");
    // destructuring
    let (age, status, name) = emp_info;

    println!("Details = {} , {}, {}", age, status, name);
}

// Write a program to add two numbers

pub fn add(a: &mut i32, b: &mut i32) -> i32 {
    let mut result: i32 = *a + *b;
    println!("Result before values were modified {}", result);

    *a += 2;
    *b += 2;

    result = *a + *b;
    println!("Result after values were modified {}", result);

    return result;
}

// See how the ownership of a is transferred to b, due to which any attempts to access 'a' after b=a will result in an error

pub fn copy_numbers() {
    let a = 100;
    let b = a;
    println!("a is {}", a);
    println!("b is {}", b);
}

// In RUST, whenever we use a dynamic memory paradigm, (like in this case, String uses the heap memory unlike the normal integer which uses stack memory), and assign the value of a variable to a different variable, there is a transfer of onwership , as there can only be one owner of a value. Now if like in this example, we try to access s1 after transfer of ownership, it shows an error as s1 is now invalidated.

// Ownership of Functions

pub fn functional_ownership() {
    let x = String::from("Sparsh");
    process_string(x); // Now ownership of x is transferred to function process_integer, and hence x is invalidated now. So if we try the following statement, it will give an error

    // println!("Value of x in main {}", x);
}

pub fn process_string(x: String) {
    println!("Value of x in process_string {}", x);
}

// Another example

pub fn ownership_example() {
    let s1: String = get_string();
    println!("This is s1 {}", s1); // s1 gets the ownership of new_string variable

    let s2: String = String::from("world");
    let s3: String = send_get_string(s2); // Now s2 has relinquished the ownership as it is being transferred to s3 over here

    println!("This is s3 {}", s3);

    let s4 = get_string();
    println!("Here is s4 {}", s4);

    let s5 = get_string();
    println!("Here is s5 {}", s5);
}

pub fn get_string() -> String {
    let new_string = String::from("Hello"); // new_string owns "Hello"
    return new_string; // Ownership of new_string is returned
}

pub fn send_get_string(received_string: String) -> String {
    return received_string; // Onwership of received_string returned
}

// Referencing and getting back ownership, example

pub fn get_back_ownership_through_tuple() {
    let s1: String = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The string and its length {} {}", s2, len);
}

pub fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

// In this example, since ownership will be transferred from s1 to s, we transfer the ownership back by modifying what we return from the function. This is a tedious method

// This is a much better way. Here we pass the reference to the String, which doesnt transfer the ownership, but also allows read-only operations on the said argument. This is the concept of borrowing.

pub fn borrowing_and_referencing() {
    let s1: String = String::from("Siddharamaiya");
    let mut s2: String = String::from("Drake");
    let length = calculate_length_with_reference(&s1);
    println!("Here is the length of s1 {}", length);
    let another_length = manipulate_length(&mut s2);
    println!("Here is the length of s2 {} {}", another_length, s1);
}

pub fn calculate_length_with_reference(s: &String) -> usize {
    let length = s.len();
    length
}

pub fn manipulate_length(s: &mut String) -> usize {
    s.push_str("Amother One");
    let length: usize = s.len();
    length
}

// So basically we can have as many immutable references to a variable as possible, so long as there is no mutable reference to the same variable.

// Also we can have only one mutable reference to a variable, but while it is there, there should be no immutable reference to the same variable.

// Attempting to borrow mutable reference

pub fn borrow_mutable_reference() {
    let mut str = String::from("Hello");
    append_string(&mut str);
    println!("New changed string {}", str);
}

pub fn append_string(str: &mut String) {
    str.push_str(" World");
}

// How is this not an error, isnt there atmost one mutable reference allowed for a variable

pub fn possible_error_or_not() {
    let mut s = String::from("Hello");
    let ref_1 = &mut s;
    println!("Ref 1 {}", ref_1);
    let ref_2 = &mut s;
    println!("Ref 2 {}", ref_2);
}

// The above code will not give an error because before a new mutable reference is assigned to ref_2, ref_1 has already gone out of scope. So within a given scope, there still is only 1 mutable reference. However the following code gives error for the same reason, its just that ref_1 hasnt gone out of scope and we have assigned another mutable reference as ref_2, creating an error.

// pub fn definitely_an_error() {
//     let mut s = String::from("Hello");
//     let ref_1 = &mut s;
//     let ref_2 = &mut s;

//     println!("Ref 1 {}", ref_1);
//     println!("Ref 2 {}", ref_2);
// }

pub fn address_and_references() {
    let x = 32;
    println!("Here is the value of x {}", x);
    let y = &x;
    println!("Here is the value referenced in y {}", y);
    println!("Here is the address y refers to {:p}", y);
}

pub fn auto_dereferencing() {
    let x = 32;
    let y = &x;

    println!("Here is the value of x {}", x);
    println!("Here is the auto dereferenced value of y {}", y); // Note that over here, technically y has the address of x, not its value. Still when we run the above print statement, RUST auto-dereferences for us. This is the concept of auto dereferencing.
}

// Following example is just an illustration of the potential error that will occur if we resemble the code written over here.

pub fn auto_dereferencing_example() {
    let s1 = String::from("hello");
    let length = calculate_auto_dereferenced_length(&s1);
    println!("Here is the auto dereferenced length {} ", length);
}

pub fn calculate_auto_dereferenced_length(s: &String) -> usize {
    // return *s.len();   This gives an error, because s.len() is eavluated first, due to which s.len() becomes 5 (for hello). Now the * operator comes in, and now it infers that you are dereferencing a number 5, which is not correct.

    // However doing this in the following manner doesnt allow for an error due to operator precedence

    return (*s).len();
}

// Dangling Reference.

// In RUST, when we return a reference to a variable (local variable) from a function, that reference becomes invalid as soon as the function exists because the local variable is dropped when the function scope ends. This results in a dangling reference, which points to a memory location that has been freed. Due to this concept, the following code gives an error and an undefined behaviour.

// pub fn dangling_reference() {
//     let reference = create_string_reference();
//     println!("Here is the string reference {}", reference);
// }

// pub fn create_string_reference() -> &String {
//     let s: String = String::from("hello");
//     return &s;
// }

// Array in RUST

pub fn array_introduction() {
    // let arr1:[u8;5]; ----  Array declaration
    let mut arr1;
    arr1 = [1, 2, 3, 4, 5];

    println!("Here is your array {:?}", arr1); // You cannot print an array with using simply {}
    arr1[2] = 69;
    println!("Here is the changed array {:?}", arr1);
}

// Pasing array as an argument to a function.

/* Here are the problems with the following code

1. Arrays in RUST are immutable and fixed in size by default when passed to a function. So if we want to modify or mutate an array, we can't pass it directly. Instead, we need to pass a mutable reference of that array to the said function.

2. Also when we pass an entire array to a function, it is passed by value, and we cannot alter the original array, so a copy is made. So in short we need to use mutable reference to alter the said original array
*/

// pub fn playing_with_array() {
//     let arr: [&str; 4] = ["John", "Kevin", "Bruce", "Robert"];
//     write_array(arr);
//     println!("After function was called, {:?}", arr);
// }

// pub fn write_array(arr: [&str; 4]) {
//     arr[0] = "Karter";
//     println!("Array inside write_array() {:?}", arr);
// }

// Here is the corrected version of the said code

pub fn playing_with_array() {
    let arr: [&str; 4] = ["John", "Kevin", "Bruce", "Robert"];
    println!("Original array {:?}", arr);
    write_array(arr);
    println!("After function was called, {:?}", arr);
}

pub fn write_array(mut arr: [&str; 4]) {
    arr[0] = "Karter";
}

// Second example

pub fn second_game() {
    let arr: [u8; 3] = [1, 2, 4];
    write_number(arr);
    println!("Inside second_game() = {:?}", arr);
    println!("Its address {:p} ", &arr);
}

pub fn write_number(mut arr: [u8; 3]) {
    arr[2] = 3;
    println!("Inside write_number() {:?}", arr);
    println!("Its address {:p} ", &arr);
}

// arr inside the write_number and the second_game function are entirely different. arr's copy is passed inside the write_number function, not arr itself.

// Btw for the playing_with_array(), this code works the same as the above code

// pub fn playing_with_array() {
//     let arr: [&str; 4] = ["John", "Kevin", "Bruce", "Robert"];
//     println!("Original array {:?}", arr);
//     write_array(arr);
//     println!("After function was called, {:?}", arr);
// }

// pub fn write_array(mut arr: [&str; 4]) {
//     arr[0] = "Karter";
// }

// Here is a more optimised way of doing what was being done in playing_with_array()

pub fn optimised_playing_with_array() {
    let mut arr: [&str; 4] = ["Tom", "Dick", "Harry", "Kayden"];
    println!("Inside optimised_playing_with_array() array is {:#?} and its address is {:p} before being passed to optimised_write()",arr,&arr);
    optimised_write(&mut arr);
}

pub fn optimised_write(arr: &mut [&str; 4]) {
    println!(
        "Inside optimised_write() array is {:#?} and its address is {:p} before being changed",
        arr, &arr
    );
    arr[0] = "Tony";
    println!(
        "Inside optimised_write() array is {:#?} and its address is {:p} after being changed",
        arr, &arr
    );
}

// Dynamic Arrays or Vectors

pub fn dynamic_array() {
    let mut v: Vec<i32> = Vec::new();

    /*  Another way to declare the vector

    let mut vec = Vec::<i32>::new();

    */
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);

    /*
    Initialising with a set of values

     let v = vec![1,2,3,4,5];

     */

    println!("Here is the dynamic array after insertion {:?}", v);
}

// The following code will give an error for the reasons of ownership

// pub fn playing_with_vectors() {
//     let vrr: Vec<&str> = vec!["Hello", "World", "Coders"];
//     write_vrr(vrr);
//     println!("Here is vrr {:?}", vrr);
// }

// pub fn write_vrr(vrr: Vec<&str>) {
//     println!("Inside write vector {:?}", vrr);
// }

// Shadowing

pub fn shadowing() {
    let a = 23;
    let a = "Hello";
    let a = a.len();

    println!("Here is a {}", a);

    /*

    We cannot do something like

    let a = 32;
    a = "Hello";

    This will give an error

     */
}

// Control Flow Statements

pub fn control_flow() {
    let number = 17;
    if number >= 18 {
        println!("You can vote");
    } else if number >= 16 && number < 18 {
        println!("You can vote under provision");
    } else if number < 16 {
        println!("You cannot vote");
    }
}

pub fn loops() {
    /* loop {
        println!("Hello Sparsh");
    }  ---- Infinite Loop*/

    let mut c = 0;
    while c < 11 {
        println!("C {}", c);
        c += 1;
    }

    /* for i in 0..10 {
        println!("i {}", i);
    }   --- Here 10 is excluded */

    /* for i in 0..=10 {
        println!("i {}", i);
    }   --- Here 10 is included */
}

// Analogue of switch case, except fall through absolutely does not occur
pub fn matching_cases() {
    let number = 1;
    match number {
        1 => println!("Number is One"),
        2 => println!("Number is Two"),
        3 => println!("Number is Three"),
        _ => println!("Whoa !"),
    }
}

pub enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}

pub fn match_enum(message: Message) {
    match message {
        Message::Quit => println!("Quit message"),
        Message::ChangeColor(r, g, b) => println!("Change color to R:{} G:{} B:{}", r, g, b),
        Message::Move { x, y } => println!("Move to x:{} y:{}", x, y),
        Message::Write(text) => println!("Write message: {}", text),
    }
}
