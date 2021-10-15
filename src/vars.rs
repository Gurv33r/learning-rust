// variables are only block-scope (meaning they exist in the function they are declared in)
// variables are immutable, meaning they cant be changed to a different value once declared
pub fn run(){
    // declare a variable with let <var_name> = <value>
    let name = "Gurveer Singh";
    println!("Start of the vars file");
    println!("The author of this file is {name}", name=name);
    
    // variables are immutable by default, meaning they CANNOT be changed to a different value once declared
    // so you can redeclare them using the shadowing method (i.e. redeclaring the same variable)
    let var = "first";
    println!("var = {}", var);
    let var = "second";
    println!("var = {}", var);
    let var = "second";
    println!("var = {}", var);
    
    // if redeclaring is inevitable, it becomes a pain to type. 
    // so just enable the mutability of the variable on declaration by adding the mut keyword between let and var_name
    let mut m = 1;
    println!("m starts at {}", m);
    m+=1; // no ++ or --, just use +=1 and -=1
    println!("m was incremented to {}", m);
    m-=1;
    println!("m was decremented to {}", m);

    // constants can also be declared but cannot be redeclared or mutated without causing an error
    // create a constant with the const keyword
    // constants have to be explicity declared, meaning the type has to be defined in the declaration
    // const format = const <const_name>: <type> = <value>;
    const PI: f64 = 3.14159265;
    println!("pi = {}", PI);

    // variable decalaration blocks format =  
    // let (var_name1, ..., var_nameN) = (value1, ..., valueN);
    let (id,age) = (2, 12);
    println!("if ID = {}, then age = {}", id, age);
}