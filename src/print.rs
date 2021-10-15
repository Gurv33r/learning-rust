pub fn run(){ // pub = public, allows it to be used by other files in the print module, all funcs are defaulted to private access
    //print with newline to console
    println!("this was done using println!() in the print.rs file");
    
    //println!(1); cannot print non-strings or variable values explicity
    //instead use a format string -> "{}" -> the empty brackets are substituted for the value of the non-string.
    println!("I can now print non-string values! {}", 1);
    
    //print {} by printing {{}}
    println!("I can now escape format! {{}}");
    
    // print multiple non string values by adding {} in the string for the first arg 
        // and the rest of the args are the non-string values placed in the order that they are placed in the string
    println!("I can now print multiple non-string values! {}, {}, {}", 1, true, 2.00);
    
    //you can add labels inside the {} for better readability, assign the labels as additional arguments in <label>=value format
    println!("{person} can {} now {action} {} using labels", action="print", person="I");
    // mixing empty brackets with labeled brackets is weird:
        // {label} {} -> if empty doesn't have a value, it will get the value of the label
        // however, it will only fill one {} that follows the labeled bracket. any more {} afterwards will cause an error
    // either label everything, don't label anything, or ensure that evey bracket has a value
    // labeling takes long but it much more readable

    // display the binary version of the value with the {<label>:b} format
    // display the hexademical version with {<label>:x} and octal with {<label>:o}
    println!("{number} in binary = {number:b}, in hex = {number:x}, in octal = {number:o}", number=23);

    // anonymous datatypes are created with () shorthand and can hold multiple primitive values
    // print out datatypes you constructed with {<label>:?}
    println!{"{:?}",(23,true,"i am a anon datatype")};

    //args can also be basic math expressions
    println!("10 + 10 = {}, 10 * 10 = {}, 10/5 = {}, 10 - 10 = {}, 10 % 3 = {}", 10+10, 10 * 10, 10/5, 10 - 10, 10%3);
}