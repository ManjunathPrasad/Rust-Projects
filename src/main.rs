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

    // slices
    let number_slices :&[i32] = &[1, 2, 3, 4, 5];
    println!("Number slices are {:?}", number_slices);

    // string slices
    let animal_slices :&[&str] = &["elephant", "tiger", "ox"];
    println!("Animal slices are {:?}", animal_slices);

    let book_slices :&[&str] = &[&"Fiction".to_string(), &"nonfiction".to_string(), &"Mystery".to_string()];
    println!("Book slices are {:?}", book_slices);

    // string vs string slices
    // strings [ growable, mutable, owned string type ]
    let mut stone_cold: String = String::from("cold fire ");
    stone_cold.push_str("cubes "); 
    println!("Stone cold slices are {}", stone_cold);
    stone_cold.push_str("water");
    println!("Stone cold slices are {}", stone_cold);

    // string slices [ fixed size, immutable, borrowed string type ]
    let string_variable: String = String::from("Hello world!");
    let slice_variable: &str = &string_variable;
    println!("Slice is {}", slice_variable);
}

