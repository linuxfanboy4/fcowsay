use std::collections::HashMap;

pub struct AnimalSay {
    message: String,
    animal: String,
}

impl AnimalSay {
    pub fn new(message: &str, animal: &str) -> Self {
        AnimalSay {
            message: message.to_string(),
            animal: animal.to_string(),
        }
    }

    pub fn say(&self) -> String {
        let wrapped_message = self.wrap_message(40);
        let border = self.create_border(&wrapped_message);
        let animal_art = self.get_animal_art();
        
        format!("{}\n{}\n{}", border, wrapped_message, animal_art)
    }

    fn wrap_message(&self, width: usize) -> String {
        let words: Vec<&str> = self.message.split_whitespace().collect();
        let mut lines = Vec::new();
        let mut current_line = String::new();
        
        for word in words {
            if current_line.len() + word.len() + 1 > width {
                lines.push(current_line);
                current_line = String::new();
            }
            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        }
        
        if !current_line.is_empty() {
            lines.push(current_line);
        }

        lines
            .into_iter()
            .map(|line| format!("| {:width$} |", line, width = width))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn create_border(&self, wrapped_message: &str) -> String {
        let width = wrapped_message.lines().next().unwrap_or("").len();
        format!(" {}", "-".repeat(width - 2))
    }

    fn get_animal_art(&self) -> &'static str {
        let animals = AnimalSay::animal_art_map();
        animals.get(self.animal.as_str()).unwrap_or(&animals["cow"])
    }

    fn animal_art_map() -> HashMap<&'static str, &'static str> {
        let mut animals = HashMap::new();
        
        animals.insert("cow", "  \\   ^__^\n   \\  (oo)\\_______\n      (__)\\       )\\/\\\n          ||----w |\n          ||     ||");
        animals.insert("sheep", "  \\  (__) \n   \\ (oo)\\_______\n      (__)\\       )\\/\\\n          ||----w |\n          ||     ||");
        animals.insert("dragon", "  \\    \\=====\n   \\   (o  o)\n        |  ~|  \n       (_____)");
        animals.insert("cat", "  \\   /\\_/\\\n   \\  ( o.o )\n        > ^ <");

        animals
    }
}

pub fn animalsay(message: &str, animal: &str) -> String {
    let animal_say = AnimalSay::new(message, animal);
    animal_say.say()
                    }
