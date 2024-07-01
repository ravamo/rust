use std::borrow::Cow;

#[derive(Debug, Clone)]
struct WordList<'a> {
    words: Cow<'a, Vec<String>>,
}

impl<'a> WordList<'a> {
    fn new(words: Vec<String>) -> Self {
        WordList {
            words: Cow::Owned(words),
        }
    }

    fn from_shared(words: &'a Vec<String>) -> Self {
        WordList {
            words: Cow::Borrowed(words),
        }
    }

    fn add_word(&mut self, word: String) {
        // Si estamos compartiendo, necesitamos clonar la lista antes de modificar
        self.words.to_mut().push(word);
    }

    fn print(&self) {
        for word in self.words.iter() {
            println!("{}", word);
        }
    }
}

fn main() {
    let shared_words = vec![
        "Hello".to_string(),
        "World".to_string(),
        "Rust".to_string(),
    ];

    let mut word_list = WordList::from_shared(&shared_words);

    println!("Initial shared list:");
    word_list.print();

    println!("\nAdding a new word...");
    word_list.add_word("Programming".to_string());

    println!("\nList after adding a word:");
    word_list.print();

    println!("\nShared list remains unchanged:");
    for word in &shared_words {
        println!("{}", word);
    }
}
