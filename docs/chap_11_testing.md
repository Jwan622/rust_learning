# Writing Automated Tests

In his 1972 essay “The Humble Programmer,” Edsger W. Dijkstra said that “Program testing can be a very effective way to show the presence of bugs, but it is hopelessly inadequate for showing their absence.” That doesn’t mean we shouldn’t try to test as much as we can!

Correctness in our programs is the extent to which our code does what we intend it to do. Rust is designed with a high degree of concern about the correctness of programs, but correctness is complex and not easy to prove. Rust’s type system shoulders a huge part of this burden, but the type system cannot catch everything. As such, Rust includes support for writing automated software tests.

Say we write a function add_two that adds 2 to whatever number is passed to it. This function’s signature accepts an integer as a parameter and returns an integer as a result. When we implement and compile that function, Rust does all the type checking and borrow checking that you’ve learned so far to ensure that, for instance, we aren’t passing a String value or an invalid reference to this function. But Rust can’t check that this function will do precisely what we intend, which is return the parameter plus 2 rather than, say, the parameter plus 10 or the parameter minus 50! That’s where tests come in.

We can write tests that assert, for example, that when we pass 3 to the add_two function, the returned value is 5. We can run these tests whenever we make changes to our code to make sure any existing correct behavior has not changed.

Testing is a complex skill: although we can’t cover every detail about how to write good tests in one chapter, we’ll discuss the mechanics of Rust’s testing facilities. We’ll talk about the annotations and macros available to you when writing your tests, the default behavior and options provided for running your tests, and how to organize tests into unit tests and integration tests.

## Anatomy of a Test Function

At its simplest, a test in Rust is a function that’s annotated with the test attribute. Attributes are metadata about pieces of Rust code; one example is the derive attribute we used with structs in Chapter 5. To change a function into a test function, add `#[test]` on the line before `fn`. When you run your tests with the `cargo test` command, Rust builds a test runner binary that runs the annotated functions and reports on whether each test function passes or fails.

When we create a new lib like `cargo new adder --lib` we get this:

```rust
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

For now, let’s ignore the top two lines and focus on the function. Note the `#[test]` annotation: this attribute indicates this is a test function, so the test runner knows to treat this function as a test. We might also have non-test functions in the tests module to help set up common scenarios or perform common operations, so we always need to indicate which functions are tests.

The example function body uses the `assert_eq!` macro to assert that result, which contains the result of adding 2 and 2, equals 4. This assertion serves as an example of the format for a typical test. Let’s run it to see that this test passes.

The cargo test command runs all tests in our project, as shown in Listing 11-2.

```bash
$ cargo test
Compiling adder v0.1.0 (file:///projects/adder)
Finished test [unoptimized + debuginfo] target(s) in 0.57s
Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Listing 11-2: The output from running the automatically generated test
```

Cargo compiled and ran the test. We see the line running 1 test. The next line shows the name of the generated test function, called it_works, and that the result of running that test is ok. The overall summary test result: ok. means that all the tests passed, and the portion that reads 1 passed; 0 failed totals the number of tests that passed or failed.

Let's keep making tests.

Let’s start to customize the test to our own needs. First change the name of the `it_works` function to a different name, such as exploration, like so:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
}
```
Then run cargo test again. The output now shows exploration instead of it_works:

```bash
$ cargo test
Compiling adder v0.1.0 (file:///projects/adder)
Finished test [unoptimized + debuginfo] target(s) in 0.59s
Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)

running 1 test
test tests::exploration ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Now we’ll add another test, but this time we’ll make a test that fails! Tests fail when something in the test function panics. Each test is run in a new thread, and when the main thread sees that a test thread has died, the test is marked as failed. In Chapter 9, we talked about how the simplest way to panic is to call the panic! macro. Enter the new test as a function named another, so your src/lib.rs file looks like Listing 11-3.


```rust
// This code panics!
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}
```

Run the tests again using cargo test. The output should look like Listing 11-4, which shows that our exploration test passed and another failed.

```bash
$ cargo test
Compiling adder v0.1.0 (file:///projects/adder)
Finished test [unoptimized + debuginfo] target(s) in 0.72s
Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)

running 2 tests
test tests::another ... FAILED
test tests::exploration ... ok

failures:

---- tests::another stdout ----
thread 'tests::another' panicked at src/lib.rs:10:9:
Make this test fail
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
tests::another

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
Listing 11-4: Test results when one test passes and one test fails
```

Instead of ok, the line test tests::another shows FAILED. Two new sections appear between the individual results and the summary: the first displays the detailed reason for each test failure. In this case, we get the details that another failed because it panicked at 'Make this test fail' on line 10 in the src/lib.rs file. The next section lists just the names of all the failing tests, which is useful when there are lots of tests and lots of detailed failing test output. We can use the name of a failing test to run just that test to more easily debug it; we’ll talk more about ways to run tests in the “Controlling How Tests Are Run” section.


The summary line displays at the end: overall, our test result is FAILED. We had one test pass and one test fail.

Now that you’ve seen what the test results look like in different scenarios, let’s look at some macros other than panic! that are useful in tests.

### Checking Results with the assert! Macro


The assert! macro, provided by the standard library, is useful when you want to ensure that some condition in a test evaluates to true. We give the assert! macro an argument that evaluates to a Boolean. If the value is true, nothing happens and the test passes. If the value is false, the assert! macro calls panic! to cause the test to fail. Using the assert! macro helps us check that our code is functioning in the way we intend.

In Chapter 5, Listing 5-15, we used a Rectangle struct and a can_hold method, which are repeated here in Listing 11-5. Let’s put this code in the src/lib.rs file, then write some tests for it using the assert! macro.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

The `can_hold` method returns a Boolean, which means it’s a perfect use case for the `assert!` macro. In Listing 11-6, we write a test that exercises the `can_hold` method by creating a Rectangle instance that has a width of 8 and a height of 7 and asserting that it can hold another `Rectangle` instance that has a width of 5 and a height of 1.

```rust
// Filename: src/lib.rs
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }
}
// Listing 11-6: A test for can_hold that checks whether a larger rectangle can indeed hold a smaller rectangle
```

Note that we’ve added a new line inside the tests module: `use super::*;`. The tests module is a regular module that follows the usual visibility rules we covered in Chapter 7 in the “Paths for Referring to an Item in the Module Tree” section. Because the tests module is an inner module, we need to bring the code under test in the outer module into the scope of the inner module. We use a glob here so anything we define in the outer module is available to this `tests` module.

We’ve named our test larger_can_hold_smaller, and we’ve created the two Rectangle instances that we need. Then we called the assert! macro and passed it the result of calling larger.can_hold(&smaller). This expression is supposed to return true, so our test should pass. Let’s find out!

```bash
$ cargo test
Compiling rectangle v0.1.0 (file:///projects/rectangle)
Finished test [unoptimized + debuginfo] target(s) in 0.66s
Running unittests src/lib.rs (target/debug/deps/rectangle-6584c4561e48942e)

running 1 test
test tests::larger_can_hold_smaller ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Doc-tests rectangle

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

It does pass! Let’s add another test, this time asserting that a smaller rectangle cannot hold a larger rectangle:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        // --snip--
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
}
```

Because the correct result of the `can_hold` function in this case is false, we need to negate that result before we pass it to the assert! macro. As a result, our test will pass if can_hold returns false:

```bash
$ cargo test
Compiling rectangle v0.1.0 (file:///projects/rectangle)
Finished test [unoptimized + debuginfo] target(s) in 0.66s
Running unittests src/lib.rs (target/debug/deps/rectangle-6584c4561e48942e)

running 2 tests
test tests::larger_can_hold_smaller ... ok
test tests::smaller_cannot_hold_larger ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Doc-tests rectangle

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
Two tests that pass! Now let’s see what happens to our test results when we introduce a bug in our code. We’ll change the implementation of the can_hold method by replacing the greater-than sign with a less-than sign when it compares the widths:

```rust
// This code does not produce the desired behavior.
// --snip--
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width < other.width && self.height > other.height
    }
}
```

### Testing equality with assert_eq! and assert_ne!

A common way to verify functionality is to test for equality between the result of the code under test and the value you expect the code to return. You could do this using the assert! macro and passing it an expression using the == operator (meaning the non-test code uses == or >). However, this is such a common test that the standard library provides a pair of macros — `assert_eq!` and `assert_ne!` — to perform this test more conveniently. These macros compare two arguments for equality or inequality, respectively. They’ll also print the two values if the assertion fails, which makes it easier to see why the test failed; conversely, the `assert!` macro only indicates that it got a false value for the `==` expression, without printing the values that led to the false value.

In Listing 11-7, we write a function named add_two that adds 2 to its parameter, then we test this function using the assert_eq! macro.


```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}
// Listing 11-7: Testing the function add_two using the assert_eq! macro
```

Let’s check that it passes!

```bash
$ cargo test
Compiling adder v0.1.0 (file:///projects/adder)
Finished test [unoptimized + debuginfo] target(s) in 0.58s
Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)

running 1 test
test tests::it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

We pass `4` as the argument to `assert_eq!`, which is equal to the result of calling `add_two(2)`. The line for this test is `test tests::it_adds_two ... ok`, and the ok text indicates that our test passed!

Let’s introduce a bug into our code to see what assert_eq! looks like when it fails. Change the implementation of the add_two function to instead add 3:

```rust
pub fn add_two(a: i32) -> i32 {
    a + 3
}
```

Run the tests again:

```bash
$ cargo test
Compiling adder v0.1.0 (file:///projects/adder)
Finished test [unoptimized + debuginfo] target(s) in 0.61s
Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)

running 1 test
test tests::it_adds_two ... FAILED

failures:

---- tests::it_adds_two stdout ----
thread 'tests::it_adds_two' panicked at src/lib.rs:11:9:
assertion `left == right` failed
left: 4
right: 5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
tests::it_adds_two

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
```

Our test caught the bug! The it_adds_two test failed, and the message tells us that the assertion that fails was assertion failed: `(left == right)` and what the left and right values are. This message helps us start debugging: the left argument was 4 but the right argument, where we had add_two(2), was 5. You can imagine that this would be especially helpful when we have a lot of tests going on.

Note that in some languages and test frameworks, the parameters to equality assertion functions are called expected and actual, and the order in which we specify the arguments matters. However, in Rust, they’re called left and right, and the order in which we specify the value we expect and the value the code produces doesn’t matter. We could write the assertion in this test as assert_eq!(add_two(2), 4), which would result in the same failure message that displays assertion failed: `(left == right)`. This is nice!

The assert_ne! macro will pass if the two values we give it are not equal and fail if they’re equal. This macro is most useful for cases when we’re not sure what a value will be, but we know what the value definitely shouldn’t be. For example, if we’re testing a function that is guaranteed to change its input in some way, but the way in which the input is changed depends on the day of the week that we run our tests, the best thing to assert might be that the output of the function is not equal to the input.

Under the surface, the assert_eq! and assert_ne! macros use the operators == and !=, respectively. When the assertions fail, these macros print their arguments using debug formatting, which means the values being compared must implement the PartialEq and Debug traits. All primitive types and most of the standard library types implement these traits. For structs and enums that you define yourself, you’ll need to implement PartialEq to assert equality of those types. You’ll also need to implement Debug to print the values when the assertion fails. Because both traits are derivable traits, as mentioned in Listing 5-12 in Chapter 5, this is usually as straightforward as adding the #[derive(PartialEq, Debug)] annotation to your struct or enum definition. 

### Adding Custom Error Messages

You can also add a custom message to be printed with the failure message as optional arguments to the assert!, assert_eq!, and assert_ne! macros. Any arguments specified after the required arguments are passed along to the format! macro (discussed in Chapter 8 in the “Concatenation with the + Operator or the format! Macro” section), so you can pass a format string that contains {} placeholders and values to go in those placeholders. Custom messages are useful for documenting what an assertion means; when a test fails, you’ll have a better idea of what the problem is with the code.

For example, let’s say we have a function that greets people by name and we want to test that the name we pass into the function appears in the output.

```rust
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }
}
```


### Checking for pancis with `should_panic`

In addition to checking return values, it’s important to check that our code handles error conditions as we expect. For example, consider the `Guess` type that we created in Chapter 9, Listing 9-13. Other code that uses Guess depends on the guarantee that `Guess` instances will contain only values between 1 and 100. We can write a test that ensures that attempting to create a Guess instance with a value outside that range panics.



