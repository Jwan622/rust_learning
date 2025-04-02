// Define a trait (similar to an interface or abstract class)
trait Animal {
    // Required method - must be implemented
    fn make_sound(&self) -> String;
    
    // Optional method with default implementation
    fn describe(&self) -> String {
        format!("I am an animal that {}", self.make_sound())
    }
}

// Define a struct for Dog
struct Dog {
    name: String,
    breed: String,
}

// Implement the Animal trait for Dog
impl Animal for Dog {
    fn make_sound(&self) -> String {
        "barks".to_string()
    }
    
    // Override the default describe method
    fn describe(&self) -> String {
        format!("{} is a {} that {}", self.name, self.breed, self.make_sound())
    }
}

// Define a struct for Cat
struct Cat {
    name: String,
    color: String,
}

// Implement the Animal trait for Cat
impl Animal for Cat {
    fn make_sound(&self) -> String {
        "meows".to_string()
    }
    
    // Override describe to use the cat's fields
    fn describe(&self) -> String {
        format!("{} is a {} cat that {}", self.name, self.color, self.make_sound())
    }
}

// A function that works with any type that implements Animal
fn print_animal_sound(animal: &impl Animal) {
    println!("The animal {}!", animal.make_sound());
}

fn main() {
    // Create instances
    let dog = Dog {
        name: String::from("Rex"),
        breed: String::from("German Shepherd"),
    };
    
    let cat = Cat {
        name: String::from("Whiskers"),
        color: String::from("orange"),
    };
    
    // Use the trait methods
    println!("Dog: {}", dog.describe());
    println!("Cat: {}", cat.describe());
    
    // Use the generic function
    print_animal_sound(&dog);
    print_animal_sound(&cat);
} 