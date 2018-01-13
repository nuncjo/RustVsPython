#[macro_use(c)]
extern crate cute;
#[macro_use]
extern crate maplit;

use std::collections::{HashMap, HashSet};


fn hashmaps() {

    // alternetive to dicts in python
    let mut persons = HashMap::new();
    persons.insert("Ruben Rybnik", 89);
    persons.insert("Olga Ryba", 55);
    persons.insert("Jan Rybak", 33);
    persons.insert("Roman Rybka", 18);

    println!("Check length: {}", persons.len());

    persons.remove("Roman Rybka");

    println!("Check length after remove: {}", persons.len());
    println!("Contains key Ruben Rybnik: {}", persons.contains_key("Ruben Rybnik"));
    println!("Age of Ruben Rybnik is : {}", persons.get("Ruben Rybnik").unwrap());

    for (name, age) in &persons {
        println!("{} is {} years old.", name, age);
    }

    for item in persons.keys() {
        println!("{}", persons.get(item).unwrap())
    }

    persons.entry("John Nonexistent").or_insert(100);
    println!("{:?}", persons);

    let names: HashMap<&str, &str> = [
        ("Guido", "VanPossum"),
        ("Kenneth", "Ratz"),
        ("David", "Veazley"),
        ("Mark", "Burtz")
    ].iter().cloned().collect();
    println!("{:?}", names);

    //dict comprehension alternative thanks to "cute" create
    let squares = c!{key => key*key, for key in 0..10};
    println!("{:?}", squares);

    //create hashmap macro thanks to "maplit" crate
    let who = hashmap!{
        "Jack" => "Harkness",
        "Martha" => "Jones"
    };
    println!("{:?}", who);

}

fn hashsets() {

    // alternetive to sets in python, items in sets are also unique
    let mut chars = HashSet::new();
    chars.insert("A");
    chars.insert("A");
    chars.insert("B");
    chars.insert("C");
    println!("{:?}", chars);

    println!("Check length: {}", chars.len());

    chars.remove("A");

    println!("Check length after remove: {}", chars.len());
    println!("Contains key A: {}", chars.contains("A"));

    for char in &chars {
        println!("{}", char);
    }

    let new_chars: HashSet<&str> = vec!["A", "Y", "B", "X"].iter().cloned().collect();
    println!("{:?}", new_chars);


    //create hashmap macro thanks to "maplit" crate
    let small_chars = hashset!{"a", "b", "c"};
    println!("{:?}", small_chars);

    //operations
    println!("Difference {:?}", chars.difference(&new_chars));
    println!("Symmetric difference {:?}", chars.symmetric_difference(&new_chars));
    println!("Intersection {:?}", chars.intersection(&new_chars));
    println!("Union {:?}", chars.union(&new_chars));

}


fn lists() {
    // array has fixed size
    let numbers = [1, 2, 3, 4, 5, 6, 7];
    println!("{:?}", numbers);

    //vector's size can be modified
    let mut numbers_vec = vec![1, 2, 3, 4, 5, 6, 7];
    println!("Check length: {}", numbers_vec.len());
    numbers_vec.push(99);
    println!("{:?}", numbers_vec);

    //removes element with index 1
    numbers_vec.remove(1);

    //removes last element
    numbers_vec.pop();

    numbers_vec[0] = 789;
    numbers_vec.extend([11, 12, 13].iter().cloned());
    println!("{:?}", numbers_vec);
    println!("Value at index 3 {}", numbers_vec[3]);

    for num in &numbers_vec {
        println!("{}", num);
    }

    let mut zeroes = vec![0; 5];
    println!("{:?}", zeroes);

    while let Some(val) = zeroes.pop() {
        println!("{}", val);
    }

    let mut new_vec = vec![99, 98, 97];

    //moves values from new_vec to numbers_vec, new_vec is empty at the end
    numbers_vec.append(&mut new_vec);
    println!("{:?}", numbers_vec);
    println!("{:?}", new_vec);

    numbers_vec.truncate(6);
    println!("{:?}", numbers_vec);

    let mut tmp_vec = vec![1, 2, 3, 4];
    //leave only values where match is true
    tmp_vec.retain(|&x| x%2 == 0);
    println!("{:?}", tmp_vec);

    // make vec of numbers in range
    let mut range_vec: Vec<i32> = (0..20).collect();
    range_vec.reverse();
    println!("{:?}", range_vec);

    // slices are immutable, often used as arguments passed to functions
    let slc = &range_vec[1..4];
    println!("{:?}", slc);

    fn print_slice(slice: &[usize]) {
        println!("{:?}", slice);
    }
    print_slice(&numbers_vec);

    let names = vec!["Mark", "Zosia", "Kasia", "Marysia"];
    for (i, name) in names.iter().enumerate() {
        println!("{} at {}", i, name);
    }

    //list comprehension thanks to "cute" crate
    let even_squares = c![x*x, for x in 0..10, if x % 2 == 0];
    println!("{:?}", even_squares);

    //the same efect in raw rust
    let even_squares_raw: Vec<i32> = (0..10).filter(|x| x % 2 == 0).collect();
    println!("{:?}", even_squares_raw);

}

fn tuples() {
    let thor = ("Thor", true, 345);
    println!("{:?}", thor);
    println!("{}, {}, {}", thor.0, thor.1, thor.2);

    let (name, _, power) = thor; //tuple unpacking
    println!("{} has {} points of power", name, power);

    let one = (1,);
    println!("single element tuple: {:?}", one);

    let (god, strength) = return_tuple(thor.0, thor.2);
    println!("This god {} has now {} strength", god, strength);
}

fn return_tuple(name: &str, power: i32) -> (&str, i32) {
    if power > 1000 {
        return (name, power * 3);
    } else {
        return (name, power * 2);
    }
}

fn main() {
    hashmaps();
    hashsets();
    lists();
    tuples();
}
