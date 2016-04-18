fn main() {
    let y = 8;              // y is of type 'i32'
    let z = 9;              // z is an i32 as well
    let mut x : &i32 = &y;  // x is of type 'reference to i32'
                            // 'mut' in this case makes 'x' mutable
                            // meaning the reference can be changed to point to a different i32

    println!("{},{}", x,y); // This print is just to avoid warnings about unused variables


    x = &z;                 // this is an error, because variable scopes are LIFO - z goes out of
                            // scope before x does. This means, this line is defining a reference
                            // that would end up living beyond the scope of the pointed-to item

    println!("{},{},{}", x,y,z); // This print is just to avoid warnings about unused variables
}

