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
