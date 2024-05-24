# Notes

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

