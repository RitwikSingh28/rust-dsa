enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // Vectors are implemented using generics
    let _vector : Vec<i32> = Vec::new();

    // or use a macro
    let mut vec2 = vec![1, 2, 3];

    vec2.push(20);
    vec2.push(22);
    vec2.push(10);

    // accessing the elements of an array
    let mut v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2]; // Panics when index out of bounds
    println!("The third element is: {third}");

    let third: Option<&i32> = v.get(2); // Lets you decide how to handle the out of bounds case 
    match third {
        Some(val) => println!("Third element via get is: {val}"),
        None => println!("Third element doesn't exist"),
    }

    let _first = &v[0]; // Here, we have an immutable reference to the zeroth element
    v.push(23); // If the following line is uncommented, results an error due to having
                // both mutable and immutable references together... 
    // println!("The first element is: {first}");
    // Remember that vector is dynamically allocated....
    
    // Iterating over the values of vectors
    let v = [1, 2, 3, 4, 5];
    for i in &v {
        println!("{i}\t");
    }

    // Iterating mutably over the vector
    let mut v = [1, 2, 3, 4, 5];
    for i in &mut v {
        *i += 50;
    }

    // enums to hold different datatypes in vec
    // The following method works only when the type of elements within the vector are known
    // beforehand
    

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    // use vector as an efficient stack
    let mut stack : Vec<i32> = vec![];
    
    stack.push(2);
    stack.push(3);
    stack.push(1);

    while let Some(top) = stack.pop() {
        println!("{top} ");
    }

    // In Rust, it's more common to pass slices as arguments rather than vectors when you just want
    // to provide read access
    let _u: &[usize] = &v;

    // Vec is, and always will be a (pointer, capacity, length) triplet 
    // If you wish to free up unused memory, use shrink_to_fit or shrink_to
}
