fn main() {
    let v: Vec<i32> = Vec::new();

    let v = vec![1, 2, 3];

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let v = vec![1, 2, 3, 4, 5];
    // Don't use;
    // let does_not_exist = &v[100];
    // Use .get, will return an Option<T>
    let does_not_exist = v.get(100);

    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);

    // Won't compile:
    //println!("The first element is: {}", first);

    // why should a reference to the first element care about what changes at the end of the vector? This error is due to the way vectors work: adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space, if there isn’t enough room to put all the elements next to each other where the vector currently is. In that case, the reference to the first element would be pointing to deallocated memory. The borrowing rules prevent programs from ending up in that situation.

    let mut v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    v.push(23);

    let mut v = vec![100, 32, 57];
    // We can also iterate over mutable references to each element in a mutable vector in order to make changes to all the elements.
    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{}", i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // Storing "different" types in a Vec:
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
