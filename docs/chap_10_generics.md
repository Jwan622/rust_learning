# Generic Types, Traits, Lifetimes

Every programming language has tools for effectively handling the duplication of concepts. In Rust, one such tool is generics: abstract stand-ins for concrete types or other properties. We can express the behavior of generics or how they relate to other generics without knowing what will be in their place when compiling and running the code.

Functions can take parameters of some generic type, instead of a concrete type like i32 or String, in the same way a function takes parameters with unknown values to run the same code on multiple concrete values. In fact, we’ve already used generics in Chapter 6 with Option<T>, Chapter 8 with Vec<T> and HashMap<K, V>, and Chapter 9 with Result<T, E>. In this chapter, you’ll explore how to define your own types, functions, and methods with generics!

First, we’ll review how to extract a function to reduce code duplication. We’ll then use the same technique to make a generic function from two functions that differ only in the types of their parameters. We’ll also explain how to use generic types in struct and enum definitions.

Then you’ll learn how to use **`traits` to define behavior in a generic way**. You can combine traits with **generic types to constrain a generic type to accept only those types that have a particular behavior**, as opposed to just any type.

Finally, we’ll discuss lifetimes: a variety of generics that give the compiler information about how references relate to each other. Lifetimes allow us to give the compiler enough information about borrowed values so that it can ensure references will be valid in more situations than it could without our help.

## Removing duplication by extracting a function

**Generics allow us to replace specific types with a placeholder that represents multiple types to remove code duplication**. Before diving into generics syntax, then, let’s first look at how to remove duplication in a way that doesn’t involve generic types by extracting a function that replaces specific values with a placeholder that represents multiple values. Then we’ll apply the same technique to extract a generic function! By looking at how to recognize duplicated code you can extract into a function, you’ll start to recognize duplicated code that can use generics.

We begin with the short program in Listing 10-1 that finds the largest number in a list.


```rust
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}
// Listing 10-1: Finding the largest number in a list of numbers
```
We store a list of integers in the variable number_list and place a reference to the first number in the list in a variable named largest. We then iterate through all the numbers in the list, and if the current number is greater than the number stored in largest, replace the reference in that variable. However, if the current number is less than or equal to the largest number seen so far, the variable doesn’t change, and the code moves on to the next number in the list. After considering all the numbers in the list, largest should refer to the largest number, which in this case is 100. easy enough!

We've now been tasked with finding the largest number in two different lists of numbers. To do so, we can choose to duplicate the code in Listing 10-1 and use the same logic at two different places in the program, as shown in Listing 10-2.

```rust
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}
// Listing 10-2: Code to find the largest number in two lists of numbers
```
Although this code works, duplicating code is tedious and error prone. We also have to remember to update the code in multiple places when we want to change it.

So what can we do? To eliminate this duplication, we’ll create an abstraction by defining a function that operates on any list of integers passed in a parameter. This solution makes our code clearer and lets us express the concept of finding the largest number in a list abstractly.

In Listing 10-3, we extract the code that finds the largest number into a function named largest. Then we call the function to find the largest number in the two lists from Listing 10-2. We could also use the function on any other list of i32 values we might have in the future.

```rust
fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
}
```
The largest function has a parameter called list, which represents any concrete slice of i32 values we might pass into the function. As a result, when we call the function, the code runs on the specific values that we pass in.

In summary, here are the steps we took to change the code from Listing 10-2 to Listing 10-3:

- Identify duplicate code.
- Extract the duplicate code into the body of the function and specify the inputs and return values of that code in the function signature.
- Update the two instances of duplicated code to call the function instead.

Next, we’ll use these same steps with generics to reduce code duplication. **In the same way that the function body can operate on an abstract list instead of specific values, generics allow code to operate on abstract types.**

For example, say we had two functions: one that finds the largest item in a slice of i32 values and one that finds the largest item in a slice of char values. How would we eliminate that duplication? Let’s find out!

## Generic Data Types

We use generics to create definitions for items like function signatures or structs, which we can then use with many different concrete data types. Let’s first look at how to define functions, structs, enums, and methods using generics. Then we’ll discuss how generics affect code performance.

When defining a function that uses generics, we place the generics in the signature of the function where we would usually specify the data types of the parameters and return value. Doing so makes our code more flexible and provides more functionality to callers of our function while preventing code duplication.

Continuing with our largest function, Listing 10-4 shows two functions that both find the largest value in a slice. We'll then combine these into a single function that uses generics.

```rust
fn largest_i32(list: &[i32]) -> &i32 {
let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}
// Listing 10-4: Two functions that differ only in their names and the types in their signatures
```

The largest_i32 function is the one we extracted in Listing 10-3 that finds the largest i32 in a slice. The largest_char function finds the largest char in a slice. The function bodies have the same code, so let’s eliminate the duplication by introducing a generic type parameter in a single function.

To parameterize the types in a new single function, we need to name the type parameter, just as we do for the value parameters to a function. **You can use any identifier as a type parameter name. But we’ll use T because, by convention, type parameter names in Rust are short, often just a letter, and Rust’s type-naming convention is UpperCamelCase. Short for “type,” T is the default choice of most Rust programmers.**


When we use a parameter in the body of the function, we have to declare the parameter name in the signature so the compiler knows what that name means. Similarly, when we use a type parameter name in a function signature, **we have to declare the type parameter name before we use it. To define the generic largest function, place type name declarations inside angle brackets, <>, between the name of the function and the parameter list**, like this:
```rust
fn largest<T>(list: &[T]) -> &T {...
```

We read this definition as: **the function largest is generic over some type T. This function has one parameter named list, which is a slice of values of type T. The largest function will return a reference to a value of the same type T.**

isting 10-5 shows the combined largest function definition using the generic data type in its signature. The listing also shows how we can call the function with either a slice of i32 values or char values. Note that this code won’t compile yet, but we’ll fix it later in this chapter.

Filename: src/main.rs

This code does not compile!
fn largest<T>(list: &[T]) -> &T {
let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
Listing 10-5: The largest function using generic type parameters; this doesn’t yet compile

If we compile this code right now, we’ll get this error:

```bash
$ cargo run
Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0369]: binary operation `>` cannot be applied to type `&T`
--> src/main.rs:5:17
|
5 |         if item > largest {
|            ---- ^ ------- &T
|            |
|            &T
|
help: consider restricting type parameter `T`
|
1 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
|             ++++++++++++++++++++++

For more information about this error, try `rustc --explain E0369`.
error: could not compile `chapter10` (bin "chapter10") due to 1 previous error
```

The help text mentions `std::cmp::PartialOrd`, which is a trait, and we’re going to talk about traits in the next section. For now, know that this error states that the body of largest won’t work for all possible types that T could be. Because we want to compare values of type T in the body, we can only use types whose values can be ordered.

To enable comparisons, the standard library has the std::cmp::PartialOrd trait that you can implement on types (see Appendix C for more on this trait). By following the help text's suggestion, we restrict the types valid for T to only those that implement PartialOrd and this example will compile, because the standard library implements PartialOrd on both i32 and char.


## Generics in Struct definitions

We can also define structs to use a generic type parameter in one or more fields using the <> syntax. Listing 10-6 defines a Point<T> struct to hold x and y coordinate values of any type.



```rust
struct Point<T> { // notice the generic type here
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```
The syntax for using generics in struct definitions is similar to that used in function definitions. **First, we declare the name of the type parameter inside angle brackets just after the name of the struct. Then we use the generic type in the struct definition where we would otherwise specify concrete data types.**


Note that because we’ve used only one generic type to define Point<T>, this definition says that the Point<T> struct is generic over some type T, and the fields x and y are both that same type, whatever that type may be. If we create an instance of a Point<T> that has values of different types, as in Listing 10-7, our code won’t compile.


```rust
// This code does not compile because POint is instantiated with 2 different types (int and float)
struct Point<T> {
    x: T, // but these are the same type
    y: T,
}

fn main() {
let wont_work = Point { x: 5, y: 4.0 };
}
```

In this example, when we assign the integer value 5 to x, we let the compiler know that the generic type T will be an integer for this instance of Point<T>. Then when we specify 4.0 for y, which we’ve defined to have the same type as x, we’ll get a type mismatch error like this:

```bash
$ cargo run
Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0308]: mismatched types
--> src/main.rs:7:38
|
7 |     let wont_work = Point { x: 5, y: 4.0 };
|                                      ^^^ expected integer, found floating-point number

For more information about this error, try `rustc --explain E0308`.
error: could not compile `chapter10` (bin "chapter10") due to 1 previous error
```

To define a Point struct where x and y are both generics but could have different types, we can use multiple generic type parameters. For example, in Listing 10-8, we change the definition of Point to be generic over types T and U where x is of type T and y is of type U.


```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
// Listing 10-8: A Point<T, U> generic over two types so that x and y can be values of different types. or the same type still!
```

Now all the instances of `Point` shown are allowed! You can use as many generic type parameters in a definition as you want, but using more than a few makes your code hard to read. If you're finding you need lots of generic types in your code, it could indicate that your code needs restructuring into smaller pieces.

**When you recognize situations in your code with multiple struct or enum definitions that differ only in the types of the values they hold, you can avoid duplication by using generic types instead.**

## Performance of using generics

You might be wondering whether there is a runtime cost when using generic type parameters. The good news is that using generic types won't make your program run any slower than it would with concrete types.

Rust accomplishes this by performing monomorphization of the code using generics at compile time. Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled. In this process, the compiler does the opposite of the steps we used to create the generic function in Listing 10-5: the compiler looks at all the places where generic code is called and generates code for the concrete types the generic code is called with. Basically, it creates more code so at runtime it calls already-compiled code at no performance cost. It's compiled at compile time.

## Generic enums

We've seen Result<T,E> already. that's an example of Enums  using multiple generic types as well. The definition of the Result enum that we used in Chapter 9 is one example:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

The Result enum is generic over two types, T and E, and has two variants: Ok, which holds a value of type T, and Err, which holds a value of type E. This definition makes it convenient to use the Result enum anywhere we have an operation that might succeed (return a value of some type T) or fail (return an error of some type E). In fact, this is what we used to open a file in Listing 9-3, where T was filled in with the type std::fs::File when the file was opened successfully and E was filled in with the type std::io::Error when there were problems opening the file.

## Example

Let's see what's going on here:

```rust
fn print_value<T: Printable>(value: &T) {
    value.print();
}
// Function Definition
```


- `fn print_value<T: Printable>(value: &T) { ... }:` This line defines a function named print_value. The function takes a single parameter value of type &T, which is a reference to a value of any type that implements the Printable trait.
- `<T: Printable>`: This is a generic type parameter declaration. It specifies that `T` can be any type that implements the `Printable` trait. In other words, **T must satisfy the constraint of implementing the `Printable` trait.** T is also in the parameter.


`value.print();:` This line calls the print method on the value parameter. Since value is of type &T, and T implements the Printable trait, this line is valid if T implements a method named print as defined by the Printable trait.

Now, let's say we have a type MyType that implements the Printable trait:

```rust
trait Printable {
    fn print(&self);
}


struct MyType {
    data: i32,
}

// Implement the Printable trait for MyType
impl Printable for MyType {
    fn print(&self) {
        println!("Printing MyType: {}", self.data);
    }
}
```

We can then use the `print_value` function with MyType objects:

```rust
fn main() {
    let my_value = MyType { data: 123 }; // this is an instance of a Struct
    print_value(&my_value); // Calls print method for MyType
}
```
In this usage:

- MyType implements the Printable trait by providing an implementation for the print method.
- We create an instance my_value of MyType.
- We call the print_value function with a reference to my_value, which is of type &MyType.
- Inside print_value, the print method of MyType is called on my_value, printing "Printing MyType: 123" to the console.

In summary, the print_value function is a generic function that accepts any type T that implements the Printable trait and calls its print method. 