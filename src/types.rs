/*
Rust is statically-typed, meaning that you MUST declare the datatype of a variable when you instantiate it
BUT compiler can actually infer the type of the variable based on the value and/or how we use it.
Statically-typed declaration = let <name>: <type> = <value>;
inferred declaration = let <name> = <value>; <- this is what we've been using so far
Primitives:
    Ints: u8, i8 (byte), u16, i16 (short), u32, i32 (standard ints, i.e. most common integer represenation), u64, i64 (long), u128, i128
    Floats: f32 (float), f64 (double)
    Booleans: bool
    Characters: char
    Arrays: fixed length and called vectors
    Tuples
 */

pub fn run() {
    let x = 1; // default number type is i32    
    let y = 2.5; // default decimal type is f64
    // inferred/default types are called implicit types, while explicit types follow the statically-typed declaration format
    let z: i64 = 454545454545;
    println!("i32 = {}, f64 = {}, explicit f64 = {}", x, y, z);
    // all float and int types have a max and min value value
    let(
        intMAX: i32 = std::i32::MAX;
        longMAX: i64 = std::i64::MAX;
        floatMAX: f32 = std::f32::MAX;
        doubleMAX: f64 = std::f32::MAX;
    )
}