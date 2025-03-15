# fcowsay - A Robust Rust Library for Using Cowsay on Rust

## Overview

`fcowsay` is a comprehensive Rust library designed to facilitate the generation of animal sayings in a manner reminiscent of the classic `cowsay` utility. This library provides a robust and extensible framework for creating and customizing animal-based ASCII art with associated messages. It is particularly suited for developers seeking to integrate whimsical or informative animal-based text displays into their Rust applications.

## Features

- **Multiple Animal Support**: Out-of-the-box support for various animals including cows, sheep, dragons, and cats.
- **Customizable Message Wrapping**: Automatically wraps messages to a specified width, ensuring clean and readable output.
- **Extensible Architecture**: Easily extendable to include new animals and custom ASCII art.
- **Comprehensive API**: Provides a straightforward API for generating animal sayings with minimal boilerplate.

## Installation

To integrate `fcowsay` into your Rust project, add the following dependency to your `Cargo.toml`:

```toml
[dependencies]
fcowsay = "2.0.0"
```

## Usage

### Basic Example

To generate a saying using the default cow:

```rust
use fcowsay::animalsay;

fn main() {
    let message = "Hello, world!";
    let animal = "cow";
    let output = animalsay(message, animal);
    println!("{}", output);
}
```

This will produce the following output:

```
 ----------------------------------------
| Hello, world!                          |
 ----------------------------------------
  \   ^__^
   \  (oo)\_______
      (__)\       )\/\
          ||----w |
          ||     ||
```

### Custom Animals

You can specify different animals by changing the `animal` parameter:

```rust
use fcowsay::animalsay;

fn main() {
    let message = "Rust is awesome!";
    let animal = "dragon";
    let output = animalsay(message, animal);
    println!("{}", output);
}
```

This will produce the following output:

```
 ----------------------------------------
| Rust is awesome!                       |
 ----------------------------------------
  \    \=====
   \   (o  o)
        |  ~|  
       (_____)
```

### Advanced Customization

For more advanced usage, you can directly use the `AnimalSay` struct to customize the message width and other parameters:

```rust
use fcowsay::AnimalSay;

fn main() {
    let animal_say = AnimalSay::new("Custom width and animal", "cat");
    let output = animal_say.say();
    println!("{}", output);
}
```

This will produce the following output:

```
 ----------------------------------------
| Custom width and animal                |
 ----------------------------------------
  \   /\_/\
   \  ( o.o )
        > ^ <
```

## Extending the Library

### Adding New Animals

To add a new animal, simply extend the `animal_art_map` function within the `AnimalSay` implementation:

```rust
impl AnimalSay {
    fn animal_art_map() -> HashMap<&'static str, &'static str> {
        let mut animals = HashMap::new();
        
        animals.insert("cow", "  \\   ^__^\n   \\  (oo)\\_______\n      (__)\\       )\\/\\\n          ||----w |\n          ||     ||");
        animals.insert("sheep", "  \\  (__) \n   \\ (oo)\\_______\n      (__)\\       )\\/\\\n          ||----w |\n          ||     ||");
        animals.insert("dragon", "  \\    \\=====\n   \\   (o  o)\n        |  ~|  \n       (_____)");
        animals.insert("cat", "  \\   /\\_/\\\n   \\  ( o.o )\n        > ^ <");
        animals.insert("new_animal", "  \\   New Art\n   \\  (o o)\n        |  ~|  \n       (_____)");
        
        animals
    }
}
```

Then, you can use the new animal by specifying it in the `animalsay` function or `AnimalSay` struct.

## Contributing

Contributions to `fcowsay` are welcome! Whether it's adding new animals, improving the message wrapping algorithm, or enhancing the API, your contributions can help make `fcowsay` even more robust and versatile.

1. Fork the repository.
2. Create a new branch for your feature or bugfix.
3. Submit a pull request with a detailed description of your changes.

## License

`fcowsay` is distributed under the MIT License. See the `LICENSE` file for more details.

## Contact

For any inquiries or suggestions, please contact Calestial Ashley at [calestialashley@gmail.com](mailto:calestialashley@gmail.com).
