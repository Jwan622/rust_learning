# Managing growing projects

Rust has a number of features that allow you to manage your code’s organization, including which details are exposed, which details are private, and what names are in each scope in your programs. These features, sometimes collectively referred to as the module system, include:

- Packages: A Cargo feature that lets you build, test, and share crates
- Crates: A tree of modules that produces a library or executable
- Modules and use: Let you control the organization, scope, and privacy of paths
- Paths: A way of naming an item, such as a struct, function, or module


## Packages and Crates

The first parts of the module system we’ll cover are packages and crates.

A crate is the smallest amount of code that the Rust compiler considers at a time. Even if you run rustc rather than cargo and pass a single source code file (as we did all the way back in the “Writing and Running a Rust Program” section of Chapter 1), the compiler considers that file to be a crate. Crates can contain modules, and the modules may be defined in other files that get compiled with the crate, as we’ll see in the coming sections.

A crate can come in one of two forms: a binary crate or a library crate. Binary crates are programs you can compile to an executable that you can run, such as a command-line program or a server. Each must have a function called `main` that defines what happens when the executable runs. All the crates we’ve created so far have been binary crates.

Library crates don’t have a main function, and they don’t compile to an executable. Instead, they define functionality intended to be shared with multiple projects. For example, the rand crate we used in Chapter 2 provides functionality that generates random numbers. Most of the time when Rustaceans say “crate”, they mean library crate, and they use “crate” interchangeably with the general programming concept of a “library". So crate == library


A package is the bundle that provides a set of functionality to be shared across multiple projects. It contains a Cargo.toml file that describes how to build one or more crates.

A package can contain at most one library crate and any number of binary crates.

A package must contain at least one crate, whether a library or binary crate. 

Packages are the units of software distribution, managed by Cargo (Rust's package manager).
Packages can depend on other packages, forming a dependency graph.

Cargo follows conventions like src/main.rs for a binary crate and src/lib.rs for a library crate within a package.

A Rust package can have at most one src/lib.rs file, which represents the single library crate allowed within that package. If you need multiple libraries, you should use a workspace with separate packages for each library, rather than trying to include multiple src/lib.rs files within a single package.

### Binary crate

Binary Crate
A binary crate is a crate that contains a main function and compiles to an executable program. The source code for a binary crate is typically located in the src/main.rs file.
Here's an example of what src/main.rs might look like:

```rust
fn main() {
    println!("Hello, world!");
}
```

When you run cargo build or cargo run in a package containing a binary crate, Cargo will compile the code in src/main.rs into an executable binary that you can run.


### Library crate

A library crate does not contain a main function and is meant to be used as a dependency by other crates. The source code for a library crate is typically located in the src/lib.rs file.

Here's an example of what `src/lib.rs` might look like:
```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

This library crate defines a public add function that adds two integers. Other crates can include this library as a dependency and use the add function.

To create a new library crate, you can run `cargo new --lib my_library`. This will create a new package with a `src/lib.rs` file containing a minimal library crate.

To compile the library crate into a library artifact (e.g., a static or dynamic library), you can use the `cargo build` command. By default, cargo build will compile your library in debug mode, creating the library artifact in the `target/debug` directory.
For example, if you run `cargo build` in the my_library directory, Cargo will compile the code in src/lib.rs and create a library artifact (e.g., libmy_library.rlib for a static library or libmy_library.so for a dynamic library on Unix-like systems) in the target/debug directory.

If you want to create a release build (optimized for performance), you can use the `--release` flag:
`cargo build --release`

This will create the library artifact in the `target/release` directory instead of target/debug.

To use the library crate in another crate or project, you need to specify it as a dependency in the Cargo.toml file of the other crate. For example, if you have another crate called my_app, you can add my_library as a dependency in my_app/Cargo.toml:
```toml
[dependencies]
my_library = { path = "../my_library" }
```

This tells Cargo to look for the `my_library` crate in the `../my_library` directory relative to the `my_app` crate.

Once you've added the dependency, you can use the library crate in your code by importing it with the extern crate statement (for Rust 2015 edition) or by using the `use` statement (for Rust 2018 edition and later).


### How do use a library

In a Rust project, you can have multiple crates, each serving a different purpose. One crate can be a binary crate (an executable program), while another can be a library crate (a reusable collection of code).

The Cargo.toml file is the manifest file that describes the metadata and dependencies of your crate (either binary or library). This is top level and you can link to libraries.

When you want to use a library crate in your project, you need to specify it as a dependency in the `Cargo.toml` file of the crate that needs to use that library.

In the example you provided:
```toml
[dependencies]
my_library = { path = "/path/to/my_library" }
```

This line is added under the [dependencies] section of your Cargo.toml file. It specifies that your crate depends on a library crate named `my_library`.

The `{ path = "/path/to/my_library" }` part tells Cargo where to find the `my_library` crate. In this case, it's a relative path from your crate's directory to the directory containing the my_library crate.
So, let's say you have the following project structure:

```rust
my_project/
├── Cargo.toml
├── src/
│   └── main.rs
└── libraries/
└── my_library/
├── Cargo.toml
└── src/
└── lib.rs
```

In this structure, `my_project` is your main crate (likely a binary crate with a main.rs file), and `my_library` is a separate library crate located in the libraries directory.

To use the `my_library` crate in your my_project crate, you would add the following line to the Cargo.toml file in the my_project directory:

```toml
[dependencies]
my_library = { path = "libraries/my_library" }
```

This tells Cargo to look for the my_library crate in the libraries/my_library directory relative to the my_project crate.

After specifying the dependency, you can then import and use the my_library crate in your main.rs file (or any other source file in your my_project crate) using the extern crate or `use` statements, as explained in my previous response.

If your library code grows large, it should not all reside in `src/lib.rs`. Instead, you should modularize your library code into multiple modules and source files.

The `src/lib.rs` file serves as the crate root and the main entry point for the library crate. It can import and re-export modules from other source files.


### Modules

Modules are the way you organize code within a crate (the unit of compilation). They allow you to control the organization, scope, and privacy of paths, allowing you to create a hierarchical namespace for better code organization.

A module is a collection of items like functions, structs, traits, impl blocks, and even nested modules.

The `mod` keyword is used to define a module.

Modules can be defined in the same file using mod module_name { ... } or in a separate file (module_name.rs or module_name/mod.rs).

Modules allow controlling visibility with the pub keyword, making items public or private.

## Defining modules to control scope and privacy

About modules and other parts of the module system, namely `paths` that allow you to name items; the `use` keyword that brings a `path` into scope; and the `pub` keyword to make items public. We’ll also discuss the as keyword, external packages, and the glob operator.

First, we’re going to start with a list of rules for easy reference when you’re organizing your code in the future. Then we’ll explain each of the rules in detail.

### Modules Cheat Sheet

Here we provide a quick reference on how modules, paths, the use keyword, and the pub keyword work in the compiler, and how most developers organize their code. We’ll be going through examples of each of these rules throughout this chapter, but this is a great place to refer to as a reminder of how modules work.

1. Start from the crate root: When compiling a crate, the compiler first looks in the crate root file (usually src/lib.rs for a library crate or src/main.rs for a binary crate) for code to compile.

2. Declaring modules: In the crate root file, you can declare new modules; say, you declare a “garden” module with `mod garden;`. The compiler will look for the module’s code in these places:

- Inline, within curly brackets that replace the semicolon following mod garden
- In the file `src/garden.rs`
- In the file `src/garden/mod.rs`

3. Declaring submodules: In any file other than the crate root, you can declare submodules. For example, you might declare `mod vegetables`; in `src/garden.rs`. The compiler will look for the submodule’s code within the directory named for the parent module in these places:

- Inline, directly following mod vegetables, within curly brackets instead of the semicolon
- In the file `src/garden/vegetables.rs`
- In the file src/garden/vegetables/mod.rs

4. Paths to code in modules: Once a module is part of your crate, you can refer to code in that module from anywhere else in that same crate, as long as the privacy rules allow, using the path to the code. For example, an Asparagus type in the garden vegetables module would be found at `crate::garden::vegetables::Asparagus.`

5. Private vs public: Code within a module is private from its parent modules by default. To make a module public, declare it with pub mod instead of mod. To make items within a public module public as well, use pub before their declarations.

6. The `use` keyword: Within a scope, the `use` keyword creates shortcuts to items to reduce repetition of long paths. In any scope that can refer to crate::garden::vegetables::Asparagus, you can create a shortcut with `use crate::garden::vegetables::Asparagus;` and from then on you only need to write Asparagus to make use of that type in the scope.


### Paths in a module tree

To show Rust where to find an item in a module tree, we use a path in the same way we use a path when navigating a filesystem. To call a function, we need to know its path.

A path can take two forms:

- An absolute path is the full path starting from a crate root; for code from an external crate, the absolute path begins with the crate name, and for code from the current crate, it starts with the literal crate.
- A relative path starts from the current module and uses self, super, or an identifier in the current module.

Both absolute and relative paths are followed by one or more identifiers separated by double colons `(::)`.

Choosing whether to use a relative or absolute path is a decision you’ll make based on your project, and depends on whether you’re more likely to move item definition code separately from or together with the code that uses the item. For example, if we move the front_of_house module and the eat_at_restaurant function into a module named customer_experience, we’d need to update the absolute path to add_to_waitlist, but the relative path would still be valid. However, if we moved the eat_at_restaurant function separately into a module named dining, the absolute path to the add_to_waitlist call would stay the same, but the relative path would need to be updated. Our preference in general is to specify absolute paths **because it’s more likely we’ll want to move code definitions and item calls independently of each other.**


Let’s try to compile Listing 7-3 and find out why it won’t compile yet!

```rust
$ cargo build
   Compiling restaurant v0.1.0 (file:///projects/restaurant)
error[E0603]: module `hosting` is private
 --> src/lib.rs:9:28
  |
9 |     crate::front_of_house::hosting::add_to_waitlist();
  |                            ^^^^^^^  --------------- function `add_to_waitlist` is not publicly re-exported
  |                            |
  |                            private module
  |
note: the module `hosting` is defined here
 --> src/lib.rs:2:5
  |
2 |     mod hosting {
  |     ^^^^^^^^^^^

error[E0603]: module `hosting` is private
  --> src/lib.rs:12:21
   |
12 |     front_of_house::hosting::add_to_waitlist();
   |                     ^^^^^^^  --------------- function `add_to_waitlist` is not publicly re-exported
   |                     |
   |                     private module
   |
note: the module `hosting` is defined here
  --> src/lib.rs:2:5
   |
2  |     mod hosting {
   |     ^^^^^^^^^^^

For more information about this error, try `rustc --explain E0603`.
error: could not compile `restaurant` (lib) due to 2 previous errors
```

The error messages say that module hosting is private. In other words, we have the correct paths for the hosting module and the add_to_waitlist function, but Rust won’t let us use them because it doesn’t have access to the private sections. In Rust, all items (functions, methods, structs, enums, modules, and constants) are private to parent modules by default. If you want to make an item like a function or struct private, you put it in a module.

Items in a parent module can’t use the private items inside child modules, but items in child modules can use the items in their ancestor modules. This is because child modules wrap and hide their implementation details, but the child modules can see the context in which they’re defined. To continue with our metaphor, think of the privacy rules as being like the back office of a restaurant: what goes on in there is private to restaurant customers, but office managers can see and do everything in the restaurant they operate.

Rust chose to have the module system function this way so that hiding inner implementation details is the default. **That way, you know which parts of the inner code you can change without breaking outer code.** However, Rust does give you the option to expose inner parts of child modules’ code to outer ancestor modules by using the pub keyword to make an item public.



### Exposing using pub keyword

Let’s return to the error in Listing 7-4 that told us the hosting module is private. We want the `eat_at_restaurant` function in the parent module to have access to the `add_to_waitlist` function in the child module, so we mark the hosting module with the pub keyword, as shown in Listing 7-5.

```rust
mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
// Listing 7-5: Declaring the hosting module as pub to use it from eat_at_restaurant
```
Unfortunately, the code in Listing 7-5 still results in an error, as shown in Listing 7-6.

```bash
$ cargo build
Compiling restaurant v0.1.0 (file:///projects/restaurant)
error[E0603]: function `add_to_waitlist` is private
--> src/lib.rs:9:37
|
9 |     crate::front_of_house::hosting::add_to_waitlist();
|                                     ^^^^^^^^^^^^^^^ private function
|
note: the function `add_to_waitlist` is defined here
--> src/lib.rs:3:9
|
3 |         fn add_to_waitlist() {}
|         ^^^^^^^^^^^^^^^^^^^^

error[E0603]: function `add_to_waitlist` is private
--> src/lib.rs:12:30
|
12 |     front_of_house::hosting::add_to_waitlist();
|                              ^^^^^^^^^^^^^^^ private function
|
note: the function `add_to_waitlist` is defined here
--> src/lib.rs:3:9
|
3  |         fn add_to_waitlist() {}
|         ^^^^^^^^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0603`.
error: could not compile `restaurant` (lib) due to 2 previous errors
// Listing 7-6: Compiler errors from building the code in Listing 7-5
```

What happened? Adding the pub keyword in front of mod hosting makes the module public. With this change, if we can access front_of_house, we can access hosting. But the contents of hosting are still private; making the module public doesn’t make its contents public. The pub keyword on a module only lets code in its ancestor modules refer to it, not access its inner code. Because modules are containers, there’s not much we can do by only making the module public; we need to go further and choose to make one or more of the items within the module public as well. We need to make the function public too.

The errors in Listing 7-6 say that the add_to_waitlist function is private. The privacy rules apply to structs, enums, functions, and methods as well as modules.

Let’s also make the add_to_waitlist function public by adding the pub keyword before its definition, as in Listing 7-7.


```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
// Listing 7-7: Adding the pub keyword to mod hosting and fn add_to_waitlist lets us call the function from eat_at_restaurant
```
Now the code will compile! Why?

In the absolute path, we start with `crate`, the root of our crate’s module tree. The `front_of_house` module is defined in the crate root. While `front_of_house` isn’t public, because the `eat_at_restaurant` function is defined in the same module as `front_of_house` (that is, `eat_at_restaurant` and `front_of_house` are siblings), we can refer to `front_of_house` from `eat_at_restaurant`. Next is the hosting module marked with pub. We can access the parent module of hosting, so we can access hosting. Finally, the add_to_waitlist function is marked with pub and we can access its parent module, so this function call works!

In the relative path, the logic is the same as the absolute path except for the first step: rather than starting from the crate root, the path starts from front_of_house. The `front_of_house` module is defined within the same module as `eat_at_restaurant`, so the relative path starting from the module in which eat_at_restaurant is defined works. Then, because `hosting` and `add_to_waitlist` are marked with pub, the rest of the path works, and this function call is valid!


### Defining paths using super

We can construct relative paths that begin in the parent module, rather than the current module or the crate root, by using `super` at the start of the path. This is like starting a filesystem path with the .. syntax. Using super allows us to reference an item that we know is in the parent module, which can make rearranging the module tree easier when the module is closely related to the parent, but the parent might be moved elsewhere in the module tree someday.

Consider the code in Listing 7-8 that models the situation in which a chef fixes an incorrect order and personally brings it out to the customer. The function fix_incorrect_order defined in the back_of_house module calls the function deliver_order defined in the parent module by specifying the path to deliver_order starting with super:


```rust
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    
    fn cook_order() {}
}
// Listing 7-8: Calling a function using a relative path starting with super
```

The fix_incorrect_order function is in the back_of_house module, so we can use super to go to the parent module of back_of_house, which in this case is crate, the root. From there, we look for deliver_order and find it. Success! We think the back_of_house module and the deliver_order function are likely to stay in the same relationship to each other and get moved together should we decide to reorganize the crate’s module tree. Therefore, we used super so we’ll have fewer places to update code in the future if this code gets moved to a different module.

### Making structs and enums public

We can also use `pub` to designate structs and enums as public, but there are a few details extra to the usage of pub with structs and enums. If we use pub before a struct definition, we make the struct public, but the struct’s fields will still be private. We can make each field public or not on a case-by-case basis. In Listing 7-9, we’ve defined a public back_of_house::Breakfast struct with a public toast field but a private seasonal_fruit field. This models the case in a restaurant where the customer can pick the type of bread that comes with a meal, but the chef decides which fruit accompanies the meal based on what’s in season and in stock. The available fruit changes quickly, so customers can’t choose the fruit or even see which fruit they’ll get.



```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
// Listing 7-9: A struct with some public fields and some private fields
```

Because the toast field in the back_of_house::Breakfast struct is public, in eat_at_restaurant we can write and read to the toast field using dot notation. Notice that we can’t use the seasonal_fruit field in eat_at_restaurant because seasonal_fruit is private. Try uncommenting the line modifying the seasonal_fruit field value to see what error you get!

Also, note that because back_of_house::Breakfast has a private field, the struct needs to provide a public associated function that constructs an instance of Breakfast (we’ve named it summer here). If Breakfast didn’t have such a function, we couldn’t create an instance of Breakfast in eat_at_restaurant because we couldn’t set the value of the private seasonal_fruit field in eat_at_restaurant.

**In contrast, if we make an enum public, all of its variants are then public. We only need the pub before the enum keyword**, as shown in Listing 7-10.


```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
// Listing 7-10: Designating an enum as public makes all its variants public
```

Because we made the Appetizer enum public, we can use the Soup and Salad variants in eat_at_restaurant.

Enums aren’t very useful unless their variants are public; it would be annoying to have to annotate all enum variants with pub in every case, so the default for enum variants is to be public. Structs are often useful without their fields being public, so struct fields follow the general rule of everything being private by default unless annotated with pub.

There’s one more situation involving pub that we haven’t covered, and that is our last module system feature: the use keyword. We’ll cover use by itself first, and then we’ll show how to combine pub and use.

## Bringing paths into scope using use keyword

Having to write out the paths to call functions can feel inconvenient and repetitive. In Listing 7-7, whether we chose the absolute or relative path to the add_to_waitlist function, every time we wanted to call add_to_waitlist we had to specify front_of_house and hosting too. Fortunately, there’s a way to simplify this process: we can create a shortcut to a path with the use keyword once, and then use the shorter name everywhere else in the scope.

In Listing 7-11, we bring the crate::front_of_house::hosting module into the scope of the eat_at_restaurant function so we only have to specify hosting::add_to_waitlist to call the add_to_waitlist function in eat_at_restaurant.


```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
// Listing 7-11: Bringing a module into scope with use
```

Adding `use` and a path in a scope is similar to creating a symbolic link in the filesystem. By adding `use crate::front_of_house::hosting` in the crate root, hosting is now a valid name in that scope, just as though the hosting module had been defined in the crate root. Paths brought into scope with use also check privacy, like any other paths.

Note that use only creates the shortcut for the particular scope in which the use occurs. Listing 7-12 moves the `eat_at_restaurant` function into a new child module named customer, which is then a different scope than the `use` statement, so the function body won’t compile:

```rust
// This code does not compile!
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
// Listing 7-12: A use statement only applies in the scope it’s in
```

The compiler error shows that the shortcut no longer applies within the customer module:

```bash
$ cargo build
Compiling restaurant v0.1.0 (file:///projects/restaurant)
error[E0433]: failed to resolve: use of undeclared crate or module `hosting`
--> src/lib.rs:11:9
|
11 |         hosting::add_to_waitlist();
|         ^^^^^^^ use of undeclared crate or module `hosting`
|
help: consider importing this module through its public re-export
|
10 +     use crate::hosting;
|

warning: unused import: `crate::front_of_house::hosting`
--> src/lib.rs:7:5
|
7 | use crate::front_of_house::hosting;
|     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
|
= note: `#[warn(unused_imports)]` on by default

For more information about this error, try `rustc --explain E0433`.
warning: `restaurant` (lib) generated 1 warning
error: could not compile `restaurant` (lib) due to 1 previous error; 1 warning emitted
```
Notice there’s also a warning that the use is no longer used in its scope! **To fix this problem, move the use within the customer module too, or reference the shortcut in the parent module with super::hosting within the child customer module.**


### Creating Idiomatic use paths

Creating Idiomatic use Paths
In Listing 7-11, you might have wondered why we specified use crate::front_of_house::hosting and then called hosting::add_to_waitlist in eat_at_restaurant rather than specifying the use path all the way out to the `add_to_waitlist` function to achieve the same result, as in Listing 7-13.


```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
}
// Listing 7-13: Bringing the add_to_waitlist function into scope with use, which is unidiomatic
```
Although both Listing 7-11 and 7-13 accomplish the same task, Listing 7-11 is the idiomatic way to bring a function into scope with use. Bringing the function’s parent module into scope with use means we have to specify the parent module when calling the function. **Specifying the parent module when calling the function makes it clear that the function isn’t locally defined while still minimizing repetition of the full path**. The code in Listing 7-13 is unclear as to where `add_to_waitlist` is defined.

On the other hand, when bringing in structs, enums, and other items with use, it’s idiomatic to specify the full path. Listing 7-14 shows the idiomatic way to bring the standard library’s HashMap struct into the scope of a binary crate.


```rust
use std::collections::HashMap;

fn main() {
let mut map = HashMap::new();
map.insert(1, 2);
}
// Listing 7-14: Bringing HashMap into scope in an idiomatic way
```

There’s no strong reason behind this idiom: it’s just the convention that has emerged, and folks have gotten used to reading and writing Rust code this way.

### Reexporting names using pub use

When we bring a name into scope with the use keyword, the name available in the new scope is private. To enable the code that calls our code to refer to that name as if it had been defined in that code’s scope, we can combine pub and use. This technique is called re-exporting because we’re bringing an item into scope but also making that item available for others to bring into their scope.

Listing 7-17 shows the code in Listing 7-11 with use in the root module changed to pub use.

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;
// Listing 7-17: Making a name available for any code to use from a new scope with pub use
```

Before this change, external code would have to call the add_to_waitlist function by using the path `restaurant::front_of_house::hosting::add_to_waitlist()`. Now that this pub use has re-exported the hosting module from the root module, external code can now use the path `restaurant::hosting::add_to_waitlist()` instead. 

Re-exporting is useful when the internal structure of your code is different from how programmers calling your code would think about the domain. For example, in this restaurant metaphor, the people running the restaurant think about “front of house” and “back of house.” But customers visiting a restaurant probably won’t think about the parts of the restaurant in those terms. With pub use, we can write our code with one structure but expose a different structure. External users won't see `front_of_house` with `pub use` here.

## Example of modularizing a package

This work is in the `restaurant2` folder

Note that you only need to load a file using a mod declaration once in your module tree. Once the compiler knows the file is part of the project (and knows where in the module tree the code resides because of where you’ve put the mod statement), other files in your project should refer to the loaded file’s code using a path to where it was declared, as covered in the “Paths for Referring to an Item in the Module Tree” section. In other words, mod is not an “include” operation that you may have seen in other programming languages.

Next, we’ll extract the hosting module to its own file. The process is a bit different because hosting is a child module of front_of_house, not of the root module. We’ll place the file for hosting in a new directory that will be named for its ancestors in the module tree, in this case src/front_of_house/.

To start moving hosting, we change src/front_of_house.rs to contain only the declaration of the hosting module:

```rust
// Filename: src/front_of_house.rs
pub mod hosting;
```

Then we create a `src/front_of_house` directory and a file `hosting.rs` to contain the definitions made in the hosting module:

```rust
// Filename: src/front_of_house/hosting.rs
pub fn add_to_waitlist() {}
```

If we instead put `hosting.rs` in the `src` directory, the compiler would expect the `hosting.rs` code to be in a hosting module declared in the crate root, and not declared as a child of the front_of_house module (`we wrote pub mod hosting in the src/front_of_house.rs file so it's a child of front_of_house module`). The compiler’s rules for which files to check for which modules’ code means the directories and files more closely match the module tree.



We’ve moved each module’s code to a separate file, and the module tree remains the same. The function calls in eat_at_restaurant will work without any modification, even though the definitions live in different files. This technique lets you move modules to new files as they grow in size.

Note that the pub use crate::front_of_house::hosting statement in src/lib.rs also hasn’t changed, nor does use have any impact on what files are compiled as part of the crate. The mod keyword declares modules, and Rust looks in a file with the same name as the module for the code that goes into that module.

### When would you just use curlies?
The file systems and `pub mod` are ways to extract the curly braces code:

You would use curly braces to define an inline module (also known as an anonymous module or a module inline with its parent) when you want to keep related items together within a specific scope without creating a separate file or directory for that module. Here's when you might use curly braces to define an inline module:

Organizing Related Items:
You want to group related functions, structs, enums, or traits together under a common namespace within a specific module or function.
This is useful for keeping the code organized and easier to understand, especially when the items are closely related and used only within that module or function.

Avoiding File or Directory Overhead:
- You want to avoid creating separate files or directories for small or closely related pieces of code that are not intended to be reused or exposed outside the parent module or function.



## Summary
Rust lets you split a package into multiple crates and a crate into modules so you can refer to items defined in one module from another module. You can do this by specifying absolute or relative paths. These paths can be brought into scope with a use statement so you can use a shorter path for multiple uses of the item in that scope. Module code is private by default, but you can make definitions public by adding the pub keyword.



## Topics

- crates (binary and library crates)
- packages
- main
- private vs public
- modules
- submodules
- use keyword
- pub keyword