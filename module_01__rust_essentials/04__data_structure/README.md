# Data Strucutre in Rust


## Struct for Custom Data Types
Structs in Rust allow you to define custom data types by combining existing types. This is useful for grouping related data together.
```rust
struct Animal {
    age: u8,
    animal_type: AnimalType,
}
```

## Enum Type
Enums provide a way to define a type by enumerating its possible variants. This is particularly useful when a value can be one of several different kinds.
```rust
enum AnimalType {
    Cat,
    Dog,
}
```

## Impl Keyword
The impl keyword is used to define methods on structs and enums. These methods can be regular methods that take self (or &self or &mut self) or associated functions that do not take self.
```rust
impl Animal {
    fn new() -> Self {
        Self {
            age: 10,
            animal_type: AnimalType::Cat,
        }
    }
}
```

## Methods
Methods are functions that are associated with a particular struct or enum. They can be defined within an impl block and can take self, &self, or &mut self as parameters.
```rust
impl Animal {
    fn change_to_dog(&mut self) {
        self.animal_type = AnimalType::Dog;
    }
}
```


## References
- Rust book - structs https://doc.rust-lang.org/book/ch05-00-structs.html 
- Rust book - defining an enum https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html 

