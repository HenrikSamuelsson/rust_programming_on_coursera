fn main() {
    let mut v = vec![1, 2, 3];
    println!("At start of main, v = {:?}", v);

    v.push(4);
    println!("Push 4 onto v, v = {:?}", v);

    // Extend adds each element of the given slice to the vector.
    let more_numbers = vec![5, 6];
    v.extend(more_numbers);
    println!("Extend v with more numbers, v = {:?}", v);

    // Append adds the given vector to the vector, requires the vector to be mutable.
    let mut other_numbers = vec![7, 8];
    v.append(&mut other_numbers);
    println!("Append another vector to v, v = {:?}", v);

    // Insert items at a given index.
    v.insert(0, 0);
    println!("Insert 0 at index 0, v = {:?}", v);
}
