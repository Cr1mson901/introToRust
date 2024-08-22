struct Person {
    age: u32,
    weight: u64,
    name: String,
}

struct Color(i32,i32,i32);
struct Point(i32,i32,i32);

fn main() {
    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    let mut person1 = Person{
        age: 25,
        name: String::from("Kyle"),
        weight: 180,
    };

    // let person2 = Person{32, 195, String::from("Mike"), true}; Must call out what you are assigning values to
    let person2 = build_person(32, String::from("Mike"), 190); //This works because I am calling a function that makes a person

    let person3 = Person {
        name: String::from("Kevin"),
        ..person2 //Reuses the attributes from person 2 except for name
        //Must come last as it says all remaining will be the same
        //Since I didn't transfer ownership of the strings I can still use person2
    };

    let person4 = Person {
        age: 88,
        weight: 104,
        name: person1.name.clone() //Only way I got this to work so far
    };

    println!("{} is {} and weighs {}", person1.name, person1.age, person1.weight);
    println!("{} is {} and weighs {}", person2.name, person2.age, person2.weight);
    println!("{} is {} and weighs {}", person3.name, person3.age, person3.weight);
    println!("{} is {} and weighs {}", person4.name, person4.age, person4.weight);
}

fn build_person(age: u32, name:String, weight:u64) -> Person {
    Person {
        age,
        name,
        weight,//Shorthand so I don't have to put x: x, for every value
    }
}