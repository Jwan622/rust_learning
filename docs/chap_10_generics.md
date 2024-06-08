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

## Traits

A trait defines functionality a particular type has and can share with other types. We can use traits to define shared behavior in an abstract way. We can use trait bounds to specify that a generic type can be any type that has certain behavior.

> Note: Traits are similar to a feature often called interfaces in other languages, although with some differences.
> 
### Defining a trait

A type’s behavior consists of the methods we can call on that type. Different types share the same behavior if we can call the same methods on all of those types. Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.

For example, let’s say we have multiple structs that hold various kinds and amounts of text: a NewsArticle struct that holds a news story filed in a particular location and a Tweet that can have at most 280 characters along with metadata that indicates whether it was a new tweet, a retweet, or a reply to another tweet.

We want to make a media aggregator library crate named aggregator that can display summaries of data that might be stored in a NewsArticle or Tweet instance. To do this, we need a summary from each type, and we’ll request that summary by calling a summarize method on an instance. Listing 10-12 shows the definition of a public Summary trait that expresses this behavior.


```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
// Listing 10-12: A Summary trait that consists of the behavior provided by a summarize method
```

Here, we declare a trait using the trait keyword and then the trait’s name, which is Summary in this case. We’ve also declared the trait as pub so that crates depending on this crate can make use of this trait too, as we’ll see in a few examples. Inside the curly brackets, we declare the method signatures that describe the behaviors of the types that implement this trait, which in this case is fn summarize(&self) -> String.

After the method signature, instead of providing an implementation within curly brackets, we use a semicolon. Each type implementing this trait must provide its own custom behavior for the body of the method. The compiler will enforce that any type that has the Summary trait will have the method summarize defined with this signature exactly.

**A trait can have multiple methods in its body: the method signatures are listed one per line and each line ends in a semicolon.**

### Implementing a trait on a Type

Now that we’ve defined the desired signatures of the Summary trait’s methods, we can implement it on the types in our media aggregator. Listing 10-13 shows an implementation of the Summary trait on the NewsArticle struct that uses the headline, the author, and the location to create the return value of summarize. For the Tweet struct, we define summarize as the username followed by the entire text of the tweet, assuming that tweet content is already limited to 280 characters.


```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet { // Summary is the trait, then comes a for, then the Type Tweet
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
// Listing 10-13: Implementing the Summary trait on the NewsArticle and Tweet types
```

Implementing a trait on a type is similar to implementing regular methods. **The difference is that after impl, we put the trait name we want to implement, then use the `for` keyword, and then specify the name of the type we want to implement the trait for.** Within the impl block, we put the method signatures that the trait definition has defined. Instead of adding a semicolon after each signature, we use curly brackets and fill in the method body with the specific behavior that we want the methods of the trait to have for the particular type.

Now that the library has implemented the Summary trait on NewsArticle and Tweet, users of the crate can call the trait methods on instances of NewsArticle and Tweet in the same way we call regular methods. The only difference is **that the user must bring the trait into scope as well as the types**. Here’s an example of how a binary crate could use our aggregator library crate:

```rust
use aggregator::{Summary, Tweet}; // we need to bring both the trait into scope with the type to use the implementation.

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
}
```
This code prints 1 new tweet: horse_ebooks: of course, as you probably already know, people.

### Default implementations

Sometimes it’s useful to have default behavior for some or all of the methods in a trait instead of requiring implementations for all methods on every type. Then, as we implement the trait on a particular type, we can keep or override each method’s default behavior.

In Listing 10-14 we specify a default string for the summarize method of the Summary trait instead of only defining the method signature, as we did in Listing 10-12.

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
//Listing 10-14: Defining a Summary trait with a default implementation of the summarize method
```
To use a default implementation to summarize instances of NewsArticle, we specify an empty impl block with 

```rust
impl Summary for NewsArticle {}
```

Does the calling code need to chagne at all when overriding? Creating a default implementation doesn’t require us to change anything about the implementation of Summary on Tweet in Listing 10-13. The reason is that the syntax for overriding a default implementation is the same as the syntax for implementing a trait method that doesn’t have a default implementation.

Default implementations can call other methods in the same trait, even if those other methods don’t have a default implementation. In this way, a trait can provide a lot of useful functionality and only require implementors to specify a small part of it. For example, **we could define the Summary trait to have a summarize_author method whose implementation is required, and then define a summarize method that has a default implementation that calls the summarize_author** method:

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;
    
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```
To use this version of Summary, we only need to define summarize_author when we implement the trait on a type:

```rust
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
```

## Traits as parameters

Now that you know how to define and implement traits, we can explore how to use traits to define functions that accept many different types. We'll use the Summary trait we implemented on the NewsArticle and Tweet types in Listing 10-13 to define a notify function that calls the summarize method on its item parameter, which is of some type that implements the Summary trait. To do this, we use the `impl Trait` syntax, like this:
```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

Instead of a concrete type for the item parameter, we specify the `impl` keyword and the `trait` name. This parameter accepts any type that implements the specified trait. In the body of notify, we can call any methods on item that come from the Summary trait, such as summarize. We can call notify and pass in any instance of `NewsArticle` or `Tweet`. Code that calls the function with any other type, such as a String or an i32, won’t compile because those types don’t implement Summary.


### Trait Bound Syntax
The impl Trait syntax works for straightforward cases but is actually syntax sugar for a longer form known as a trait bound; it looks like this:

```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

This longer form is equivalent to the example in the previous section but is more verbose. We place trait bounds with the declaration of the generic type parameter after a colon and inside angle brackets.

The impl Trait syntax is convenient and makes for more concise code in simple cases, while the fuller **trait bound syntax can express more complexity in other cases**. For example, we can have two parameters that implement `Summary`. Doing so with the impl Trait syntax looks like this:

```rust
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
```

Using impl Trait is appropriate if we want this function to allow item1 and item2 to have different types (as long as both types implement Summary). If we want to force both parameters to have the same type, however, we must use a trait bound, like this:

```rust
pub fn notify<T: Summary>(item1: &T, item2: &T) { // the generic type T must implement Summary and item1 adn item2 must be of type T
```
The generic type T specified as the type of the item1 and item2 parameters constrains the function such that the concrete type of the value passed as an argument for item1 and item2 must be the same.

## Specifying multiple trait bounds

We can also specify more than one trait bound. Say we wanted notify to use display formatting as well as summarize on item: we specify in the notify definition that item must implement both Display and Summary. We can do so using the + syntax:

```rust
pub fn notify(item: &(impl Summary + Display)) { // & means notify takes a reference to an object. but cannot modify or take ownership of it. Using references allows for efficient passing of data without copying large objects.
```

The + syntax is also valid with trait bounds on generic types:
```rust
pub fn notify<T: Summary + Display>(item: &T) {
```

With the two trait bounds specified, the body of notify can call summarize and use {} to format item.



### Returning types that implement traits

We can also use the impl Trait syntax in the return position to return a value of some type that implements a trait, as shown here:

```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false, 
    }
}
```
By using impl Summary for the return type, we specify that the returns_summarizable function returns some type that implements the Summary trait without naming the concrete type. In this case, returns_summarizable returns a Tweet, but the code calling this function doesn’t need to know that.

The ability to specify a return type only by the trait it implements is especially useful in the context of closures and iterators, which we cover in Chapter 13. Closures and iterators create types that only the compiler knows or types that are very long to specify. The impl Trait syntax lets you concisely specify that a function returns some type that implements the Iterator trait without needing to write out a very long type.

**In dynamically typed languages, we would get an error at runtime if we called a method on a type which didn’t define the method. But Rust moves these errors to compile time so we’re forced to fix the problems before our code is even able to run. Additionally, we don’t have to write code that checks for behavior at runtime because we’ve already checked at compile time. Doing so improves performance without having to give up the flexibility of generics.**


### Using trait bou to Conditionally Implement methods

By using a trait bound with an impl block that uses generic type parameters, we can implement methods conditionally for types that implement the specified traits. For example, the type Pair<T> in Listing 10-15 always implements the new function to return a new instance of Pair<T> (recall from the “Defining Methods” section of Chapter 5 that Self is a type alias for the type of the impl block, which in this case is Pair<T>). But in the next impl block, Pair<T> **only implements the cmp_display method if its inner type T implements the PartialOrd trait that enables comparison and the Display trait that enables printing.**

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
// Listing 10-15: Conditionally implementing methods on a generic type depending on trait bounds
```

**We can also conditionally implement a trait for any type that implements another trait. Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations and are extensively used in the Rust standard library**. For example, the standard library implements the ToString trait on any type that implements the Display trait. The impl block in the standard library looks similar to this code:

```rust
impl<T: Display> ToString for T { // implement that ToString trait on any type that implements the Display Trait
    // --snip--
}
```

Because the standard library has this blanket implementation, we can call the to_string method defined by the ToString trait on any type that implements the Display trait. For example, we can turn integers into their corresponding String values like this because integers implement Display:

```rust
let s = 3.to_string();
```

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