fn main() {
    println!("Hello, world!");

    // Primitive Datatypes
    // int, float, bool, char

    // int
    let variable1: i64 = 10;
    let variable2: i64 = 20;
    println!("Sum of {:?} and {:?} = {:?}", variable1, variable2, (variable1+variable2));

    // float
    let pi: f64 = 3.145;
    println!("Pi = {:?}", pi);

    // bool
    let is_snowing: bool = false;
    println!("Is Snowing now? {:?} ", is_snowing);

    // char 
    let first_letter: char = 'A';
    println!("First letter of Alphabets is {:?}", first_letter);

    // compound datatypes 
    // arrays, tuples, string slices

    // arrays
    // int array
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array values are {:?}", numbers);

    // float array
    let floatarray: [f64; 5] = [1.4, 3.14, 2.3, 5.6, 7.3];
    println!("Floating array values are {:?}", floatarray);

    //  string array
    let fruits: [&str; 3] = ["Berries", "Guava", "Pom"];
    println!("Fruits are {:?}", fruits);
    println!("First fruit in the Array is {:?}", fruits[0]);

    // tuples
    let person: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Person details is {:?}", person);

    // mix of tuples
    let mix_tuples = ("Kratos", 23, true, [1, 2, 3, 4, 5]);
    println!("Mix tuple is {:?}", mix_tuples);
}