// fn main() {
//     println!("First Program in RUST!");
// }

// Studying Data Types

// Variables can be mutable in one scope but immutable in another. Once a mutable variable goes out of scope, any re-declaration of that variable will need a new mut declaration if mutability is required.

// fn main() {
//     let mut a = 5;
//     {
//         let a = a; // Shadowing, a is now immutable
//                    // a = 6; // Error: cannot assign to `a` because it is immutable
//     }
//     a = 6; // This is allowed because we are back to the outer mutable scope
//     println!("The value of a is: {}", a);
// }

// By default, function parameters are immutable. If you want to change the value of a parameter, you must make it mutable within the function's scope.

// fn main() {
//     let mut z = 10;
//     change_value(&mut z);
//     println!("The value of z after change is: {}", z);
// }

// fn change_value(val: &mut i32) {
//     *val = 20; // Dereferencing and changing the value
// }

// fn main() {
//     print!("Hello World");
//     let sentence = "This is a sentence in string";
//     print!("{} ", sentence);
// }

// fn increment(n: &mut i32) {
//     *n += 2;
// }

// fn main() {
//     let mut v = 14;
//     increment(&mut v);
//     println!("New value after increment {}", v);
// }

// &str is immutable, but String is mutable

// fn main() {
//     let mut string: String = String::from("This is an mutable sentence");
//     println!("{}", string);

//     string.push_str(" Yup Yup");
//     println!("New ,{}", string);
// }

// July 30th . Started Tuple Data Type

// fn main() {
//     let emp_info: (u8, bool, &str) = (12, true, "Lokesh");

//     // destructuring

//     let (age, status, name) = emp_info;

//     println!("Details = {} , {}, {}", age, status, name);
// }

// Write a program to add two numbers

fn add(a: &mut i32, b: &mut i32) -> i32 {
    let mut result: i32 = *a + *b;
    println!("Result before values were modified {}", result);

    *a += 2;
    *b += 2;

    result = *a + *b;
    println!("Result after values were modified {}", result);

    return result;
}

fn main() {
    let mut a = 12;
    let mut b = 18;

    let _res = add(&mut a, &mut b);
}
