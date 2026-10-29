# Simpler Rust Questions

Remeber, you are allowed google

## Data modelling

These questions are about how you represent real-world objects (dogs, cats, songs, people, products, etc.) using Rust code.

For each structure you create, also show code creating an instance of that structure

Things to pay attention to:
 - the type you are using. Often, there are many different types which will "do the job", but some are better than others. 
 - Don't be afraid to look for 3rd party libraries if you feel like "this is a common thing that people will work with, I wonder if there's a standard type to use". Check the standard library first, but remember that the standard library is deliberately missing many types. For example, there is no `Date` or `DateTime` type in the Rust standard library. The 3rd party crate `chrono` contains types for this. I won't tell you the others :p
 - You may be able to reuse structures from old questions for the later questions


### Example

Question: create some Rust data structures (structs and enums) that can represent cars.

Cars have a price, number plate, number of seats, an age (in years), and a type (either "manual", "automatic", or "electric").

Answer:
```rust
struct Car {
    number_plate: String,
    price_in_pounds: u32,
    number_of_seats: u8,
    age_in_years: u8
    kind: CarKind,  // we use `kind` because `type` is a keyword in rust
}

enum CarKind {
    Manual,
    Automatic,
    Electric,
}

fn make_car() -> Car {
    Car {
        number_plate: String::from("ab12 cde");
        price_in_pounds: 10_000,
        number_of_seats: 5,
        age_in_years: 2,
        kind: CarKind::Manual,
    }
}
```

### Questions

 1. Design a set of structures to represent dogs. 
   - Dogs have a name, an age, a color (brown, black, or white), and a tail length (long, meduim, or short).
 2. Design a set of structures to represent laptops. 
   - Laptops have a name, an ip address, and an operating system
   - An operating system is either Mac, Windows, or Linux. If it is Windows, it is either "Home Edition" or "Pro Edition"
   - An IP address is 4 numbers separated by `.`. Each number is between `0` and `255` inclusive. (e.g. `125.4.0.255`)
 3. Design a set of structures to represent people.
   - A person has a name, date of birth, "handedness" (i.e. left-handed, right-handed), height, weight
 4. Design a set of structures to represent cats.
   - Cats have a name, weight, height, whisker count, and favourite food (either "cat food", "mice", or "no preference")
 5. Design a set of structures to represent cables.
   - Cables have a length, material (either "braided", "rubber", "plastic"), and 2 connectors
   - A connector has a "standard" and a gender (male or female) (https://en.wikipedia.org/wiki/Gender_of_connectors_and_fasteners#Gender_selection_in_electronic_design)
   - A standard is either "lightning", "usb", "magsafe". "usb" can be either "usb-a" (the rectangular one), or "usb-c"
 6. Design a set of structures to represent computers.
   - A computer is either a laptop (with the same requirements as in question 2), or a desktop
   - A desktop has a CPU, GPU, a size (mini, medium, large), and a power consumption
 

## Simple Programming

These are just some simple "pure programming" questions, that don't require much knowledge of other systems:

### Numbers and loops

For these questions, try to avoid using iterators, and instead, try to use explicit loops (e.g. `for ... in ... {}`, `while ... {}` and `loop {}`)

 1. print all the numbers from 1 to 100 inclusive
 2. print all the even numbers from 1 to 100 inclusive
 3. print all the numbers that are a multiple of 3 or 5 from 1 to 100 inclusive
 4. print all the square numbers from 1 to 100 inclusive (a square number is a number that is equal to `n*n` where `n` is an integer)
 5. print all the odd square numbers from 1 to 100 inclusive
 6. print the sum of all the odd square numbers from 1 to 100 inclusive
 7. print the product of all the odd square numbers from 1 to 100 inclusive
 8. print all the numbers from 1 to 100 inclusive which have a digit sum greater than 10. The digit sum is the sum of the individual digits of a number (i.e. the digit sum of 123 is `1 + 2 + 3 = 6`)
 9. print all the numbers from 1 to 100 inclusive which have a digit sum that is even
 10. print numbers from 1 to 100 at random. It must give different results every time it's run, and should print roughly 50 numbers each time.
 11. write a function which returns the "Collatz Stopping Time" for a number. The collatz stopping time for a number is the number of steps it takes for a number to reach 1 when you repeatedly apply the following rules:
   - if the number is even, the next number is `n / 2`
   - if the numeber is odd, the next number is `(3 * n) + 1`
 11. (continued) Some examples:
   - `cst(1) = 0` (1 is already 1)
   - `cst(4) = 2` (4 -> 2 -> 1, 2 steps)
   - `cst(5) = 5` (5 -> 16 -> 8 -> 4 -> 2 -> 1, 5 steps)
   - `cst(3) = 7` (3 -> 10 -> 5 -> 16 -> 8 -> 4 -> 2 -> 1, 7 steps)
 12. print `1!` to `10!` inclusive (`n!` is `1 * 2 * 3 * ... * n`, pronounced "n factorial")
 13. (hard) write a function to print the first 100 prime numbers (a prime number is a number bigger than 1 that is divisible only by 1 and itself)
 14. write a function that plays fizzbuzz up to 100. Fizzbuzz is a game where you print numbers from 1 to 100 except
   - if the number is divisible by 3, you print "fizz" instead of the number
   - if the number is divisible by 5, you print "buzz" instead of the number
   - if the number is divisible by 3 and 5, you print "fizzbuzz" instead of the number
 15. write a function that calculates the area of a triangle with the following properties:
  - there are 3 points, A, B and C
  - the distance between A and B is `x` (one of the inputs to the function)
  - the distance between A and C is `y` (one of the inputs to the function)
  - the angle between the lines AB and AC is 90 degrees
 16. Write a function that finds the largest single digit in a number (i.e. for 251 it would be `5`)
 
Now see how many of these you can write using iterators instead of explicit loops. 
Note, Q15 doesn't really use loops, so don't try to use iterators. Just skip this one.
Do you generally prefer iterators or explicit loops?
Are there circumstances where you'd prefer one over the other?

### Lists

These questions involve writing functions that process lists.
For now, we can assume that the list will be a `Vec<T>`.
Your functions should take a list as a parameter, and the type should be a "slice" - `&[T]`.
A slice is similar to a `&str` - it's cheaper and more efficient to use if you don't need to modify the data.
You can convert a `Vec<T>` to a `&[T]` by simply putting a `&` in front of it:

```rust
fn main() {
    let vec: Vec<i32> = vec![1, 2, 3];
    takes_slice(&vec);  // &vec is a &[i32]
}

fn takes_slice(slice: &[i32]) {
    println!("i have a slice");
}
```

 1. Write a function which takes a list of `i32`s and returns the biggest element
 2. Write a function which takes a list of `i32`s and returns the smallest element
 3. Write a function which takes a list of `i32`s and calculates the sum of the two largest elements
 4. Write a function which takes a list of `i32`s and calculates the difference between the largest element and the smallest element. (Once you've written this, try writing it by using the answers to previous questions)
 5. Write a function which takes two lists of `i32`s and calculates the "dot product"
   - the dot product of two lists is calculated by:
     - calculating the product of elements in the same position
     - adding up each product
   - for example, the dot product of `[1, 2, 3]` and `[4, 5, 6]` is `(1 * 4) + (2 * 5) + (3 * 6) = 4 + 10 + 18 = 32` 
   - if the two lists have a different length, the dot product is "undefined" (i.e. there is no dot product, similar to how there is "no answer" to `1 / 0`).
 6. Write a function which takes a list of `i32`s, and another `i32` called `target`. The function should return the index of `target` in the list:
   - for example, if the list is `[1, 6, 4, 7, 2]`, and `target`is `4`, the answer should be `2` (since the number `4` is in position `2`)
   - remember, we start counting at `0`
 7. Write a function that takes a list of `f64`s and calculates the mean:
   - the mean is "the sum of each element divided by the number of elements"
 8. Write a function that takes a list of `f64`s and calculates the median:
   - the median is the "middle number"
   - you can assume the list you are given is already sorted in ascending order
 9. Write a function that takes a list of `i32`s and returns two lists, one containing all the positive numbers, the other containing all the negative numbers
   - The numbers in each returned list must be in the same order they appeared in the original list
   - For example, if the input list is `[1, 2, -4, 5, -6]` then the output should be `[1, 2, 5]` and `[-4, -6]`
 10. Write a function which takes a list of `String`s and returns the longest
 11. Write a function which takes a list of `bool`s and returns the number of `true`s
 12. Write a function which takes a list of `String`s and returns a new list with all the strings capitalized
 13. Write a function which takes a list of `String`s and returns a `String` which is formed by putting all of the first letters together
   - e.g. if the input list is `["hello", "world", "foo", "bar"]` then the string should be `"hwfb"`

Like the number questions, all of these can be rewritten to use iterators.

Many of these functions may need to account for empty lists. For example, what is the "maximum value" of the list `[]`?


### Strings

These questions deal with manipulating strings. 
Check out [the docs for the `str` type](https://doc.rust-lang.org/std/primitive.str.html) if you get stuck. 
There may be useful builtin functions that can help solve some parts of the questions.
If you want to use regex (you may want to, you may not), [use the `regex` crate](https://docs.rs/regex)

Remember to use the right string type in your function signatures - either `&str` or `String`.
Consider what happens in "edge cases":
 - what happens if the string is empty
 - what happens if the string contains only spaces or newline characters

 1. Write a function which reverses a string
 2. Write a function which counts the number of vowels in a string.
 3. Write a function which counts how many vowels there are on each line of a string, and returns the maximum number of vowels on a single line.
 4. Write a function which reverses the order of words in a sentence (i.e. "hello world from rust" should become "rust from world hello")
 5. Write a function which returns the "score" of the word with the highest score in a string:
   - the score is calculated by taking the individual letters of a word, converting them to numbers (a = 1, b = 2, ... z = 26), and adding them up
 6. (hard) Write a function which counts the number of unique words in a setence (i.e. "hello world hello" has 2 unique words)
 7. (hard) Write a function which returns the most common word in a string
 8. (extra hard) Write a function which returns a reference to the first occurrence of the most common word in a string (i.e. the function should have the signature `fn(&str) -> &str` and must not create any `String`s at any point) 
 9. write a function which replaces any digits with the corresponding word (i.e. "hello 123" would become "hello onetwothree")
 10. Write a function which replaces any occurrences of "zero", "one", "two", ... "nine", with the corresponding digit. Does this function "undo" the effects of the previous question?
 11. (hard) Write a function which returns true if a string is a valid email address (only do "syntactic checking", i.e. only check "could this possibly be a valid email", not "is this actulaly a real email that someone owns". In other words, your code doesn't need to connect to the internet)
 12. (hard) Write a function which takes a string containing Rust source code and prints the number of functions and the number of variables
 13. Write a function which takes a string containing a list of numbers separated by commas, and returns the sum
