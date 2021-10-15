// Welcome to Rust!
// rust files have the .rs extension
// think of rust as a mix between go and c++
// for starters:
    // func = fn
    // main is still main
    // fmt.Println() = println!(), go to print.rs for more info on how printing and formatting works in rust
    // end all instructions with a ; just like C and Java
    // int = i32
    // arrays = slices
// run rust files in 2 ways: (rust is translate-only, meaning that the only way to run a rust file is to translate it into an executable first and then run the executable)
    //rustc <filename>.rs, and then ./filename
    //create a cargo project (kind of like a react project) 
        // run cargo init in the folder you want to develop in (you could also do cargo new <dev folder>) 
        // edit rust files in the src directory. (by default, src file will only have 1 rs file in it: main.rs)
        // run the entire project with cargo run 
            // this will build the entire project into a directory called target 
            // executable of main.rs is found in target/debug under the name of the root directory of the project
            // the executable will be run
        // you can also just build the project with cargo build, and optimize it for production by adding the --release flag to the command
// import rs files with mod <filename>;
//mod print; 
mod vars;
fn main() {
    println!("Hello, world!");
   // print::run(); //access public contents of imported modules with <mod>::<fn> notation
   vars::run()
}
