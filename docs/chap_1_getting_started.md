# Installation

## Why Rust?

Take, for example, “systems-level” work that deals with low-level details of memory management, 
data representation, and concurrency. Traditionally, this realm of programming is seen as arcane, 
accessible only to a select few who have devoted the necessary years learning to avoid its infamous 
pitfalls. And even those who practice it do so with caution, lest their code be open to exploits, 
crashes, or corruption.

Rust breaks down these barriers by eliminating the old pitfalls and providing a friendly, 
polished set of tools to help you along the way. Programmers who need to “dip down” into lower-level 
control can do so with Rust, without taking on the customary risk of crashes or security holes.

Programmers who are already working with low-level code can use Rust to raise their ambitions. 
For example, introducing parallelism in Rust is a relatively low-risk operation: the compiler will 
catch the classical mistakes for you. And you can tackle more aggressive optimizations in your code 
with the confidence that you won’t accidentally introduce crashes or vulnerabilities.

## Intro

High-level ergonomics and low-level control are often at odds in programming language design; Rust challenges that conflict. Through balancing powerful technical capacity and a great developer experience, Rust gives you the option to control low-level details (such as memory usage) without all the hassle traditionally associated with such control.

Low-level code is prone to various subtle bugs, which in most other languages can be caught only through extensive testing and careful code review by experienced developers. In Rust, the compiler plays a gatekeeper role by refusing to compile code with these elusive bugs, including concurrency bugs. By working alongside the compiler, the team can spend their time focusing on the program’s logic rather than chasing down bugs.

## Tools

Cargo, the included dependency manager and build tool, makes adding, compiling, and managing dependencies painless and consistent across the Rust ecosystem.
The Rustfmt formatting tool ensures a consistent coding style across developers.

## Speed vs stability

Rust is for people who crave speed and stability in a language. By speed, we mean both how quickly Rust code can run and the speed at which Rust lets you write programs. The Rust compiler’s checks ensure stability through feature additions and refactoring. This is in contrast to the brittle legacy code in languages without these checks, which developers are often afraid to modify. By striving for zero-cost abstractions, higher-level features that compile to lower-level code as fast as code written manually, Rust endeavors to make safe code be fast code as well.


# Hello World


## Macros vs functions

```rust
fn main() {
    println!("Hello, world!");
}
```

println! calls a Rust macro. If it had called a function instead, it would be entered as println (without the !). We’ll discuss Rust macros in more detail in Chapter 19. For now, you just need to know that using a ! means that you’re calling a macro instead of a normal function and that macros don’t always follow the same rules as functions.

end the line with a semicolon (;), which indicates that this expression is over and the next one is ready to begin. Most lines of Rust code end with a semicolon.


## Compiling and running

Compiling and Running Are Separate Steps

Before running a Rust program, you must compile it using the Rust compiler by entering the rustc command and passing it the name of your source file, like this:

```rust
$ rustc main.rs
```

If you’re more familiar with a dynamic language, such as Ruby, Python, or JavaScript, you might not be used to compiling and running a program as separate steps. Rust is an ahead-of-time compiled language, meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed. If you give someone a .rb, .py, or .js file, they need to have a Ruby, Python, or JavaScript implementation installed (respectively). But in those languages, you only need one command to compile and run your program. Everything is a trade-off in language design.


Just compiling with rustc is fine for simple programs, but as your project grows, you’ll want to manage all the options and make it easy to share your code. Next, we’ll introduce you to the Cargo tool, which will help you write real-world Rust programs.


# Hello Cargo

Cargo is Rust’s build system and package manager. Most Rustaceans use this tool to manage their Rust projects because Cargo handles a lot of tasks for you, such as building your code, downloading the libraries your code depends on, and building those libraries. (We call the libraries that your code needs dependencies.)

## The Cargo.toml file

This file is in the TOML (Tom’s Obvious, Minimal Language) format, which is Cargo’s configuration format.

The first line, [package], is a section heading that indicates that the following statements are configuring a package. As we add more information to this file, we’ll add other sections.

The next three lines set the configuration information Cargo needs to compile your program: the name, the version, and the edition of Rust to use. We’ll talk about the edition key in Appendix E.

The last line, [dependencies], is the start of a section for you to list any of your project’s dependencies. In Rust, packages of code are referred to as **crates**. We won’t need any other crates for this project, but we will in the first project in Chapter 2, so we’ll use this dependencies section then.

Cargo expects your source files to live inside the src directory. The top-level project directory is just for README files, license information, configuration files, and anything else not related to your code. Using Cargo helps you organize your projects. There’s a place for everything, and everything is in its place.

to compile and run the resulting debug file:

```rust
13:26 $ cargo build
Compiling hello_cargo v0.1.0 (/Users/jwan/Desktop/programming/rust_learning/hello_cargo)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.99s
✔ ~/Desktop/programming/rust_learning/hello_cargo [master|●2✚ 1…30]
13:26 $ ./target/debug/hello_cargo
Hello, world!
```

you can simply just run `cargo run`

Using `cargo run` is more convenient than having to remember to run cargo build and then use the whole path to the binary, so most developers use cargo run.

Cargo also provides a command called `cargo check`. This command quickly checks your code to make sure it compiles but doesn’t produce an executable:

Why? Often, `cargo check` is much faster than cargo build because it skips the step of producing an executable. If you’re continually checking your work while writing the code, using cargo check will speed up the process of letting you know if your project is still compiling! As such, many Rustaceans run cargo check periodically as they write their program to make sure it compiles. Then they run cargo build when they’re ready to use the executable.


When your project is finally ready for release, you can use `cargo build --release` to compile it with optimizations. This command will create an executable in target/release instead of target/debug. The optimizations make your Rust code run faster, but turning them on lengthens the time it takes for your program to compile. This is why there are two different profiles: one for development, when you want to rebuild quickly and often, and another for building the final program you’ll give to a user that won’t be rebuilt repeatedly and that will run as fast as possible.


