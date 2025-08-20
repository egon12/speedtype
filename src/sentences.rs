
use std::time::Instant;

use crate::my_logger::MyLogger;

#[derive(Debug, Clone)]
pub struct TypingSession {
    pub sentence: String,
    pub words: Vec<Word>,
    pub current_word_index: usize,
    pub typed_text: String,
    pub cursor_position: usize,
    pub start_time: Option<Instant>,
    pub stop_time: Option<Instant>,
    pub errors: usize,
    pub total_characters: usize,
    pub logger: MyLogger,
}

#[derive(Debug, Clone)]
pub struct Word {
    pub text: String,
    pub typed: String,
    pub is_completed: bool,
    pub is_correct: bool,
    pub has_error: bool,
}

impl TypingSession {
    pub fn new(sentence: String) -> Self {
        let words: Vec<Word> = sentence
            .split_whitespace()
            .map(|word| Word {
                text: word.to_string(),
                typed: String::new(),
                is_completed: false,
                is_correct: false,
                has_error: false,
            })
            .collect();

        Self {
            sentence,
            words,
            current_word_index: 0,
            typed_text: String::new(),
            cursor_position: 0,
            start_time: None,
            stop_time: None,
            errors: 0,
            total_characters: 0,
            logger: MyLogger::new(),
        }
    }

    pub fn start(&mut self) {
        if self.start_time.is_none() {
            self.start_time = Some(Instant::now());
        }
    }

    pub fn add_character(&mut self, ch: char) {
        if self.start_time.is_none() {
            self.start();
        }

        self.typed_text.push(ch);
        self.total_characters += 1;
        
        if self.current_word_index < self.words.len() {
            let current_word = &mut self.words[self.current_word_index];
            current_word.typed.push(ch);
            
            // Check if character is correct
            let expected_char = current_word.text.chars().nth(current_word.typed.len() - 1);
            if expected_char != Some(ch) {
                current_word.has_error = true;
                self.errors += 1;
            }
            
            // Check if word is completed
            if current_word.typed.len() == current_word.text.len() {
                current_word.is_completed = true;
                current_word.is_correct = !current_word.has_error;
            }
        }
        
        self.cursor_position += 1;
    }

    pub fn handle_space(&mut self) {
        if self.current_word_index < self.words.len() {
            let current_word = &mut self.words[self.current_word_index];
            
            // Mark current word as completed if not already
            if !current_word.is_completed {
                current_word.is_completed = true;
                current_word.is_correct = current_word.typed == current_word.text && !current_word.has_error;
                
                // If word is incomplete, mark as error
                if current_word.typed.len() < current_word.text.len() {
                    current_word.has_error = true;
                    self.errors += current_word.text.len() - current_word.typed.len();
                }

            }
            
            // Move to next word
            self.current_word_index += 1;
            self.typed_text.push(' ');
            self.cursor_position += 1;
        } else {
            self.stop_time = Some(Instant::now());
        }

    }

    pub fn handle_backspace(&mut self) {
        if !self.typed_text.is_empty() {
            let last_char = self.typed_text.pop();
            self.cursor_position = self.cursor_position.saturating_sub(1);
            
            if last_char == Some(' ') {
                // Moving back to previous word
                if self.current_word_index > 0 {
                    self.current_word_index -= 1;
                    let current_word = &mut self.words[self.current_word_index];
                    current_word.is_completed = false;
                    current_word.is_correct = false;
                }
            } else if self.current_word_index < self.words.len() {
                // Remove character from current word
                let current_word = &mut self.words[self.current_word_index];
                if !current_word.typed.is_empty() {
                    current_word.typed.pop();
                    current_word.is_completed = false;
                    current_word.is_correct = false;
                    current_word.has_error = false;
                }
            }
        }
    }

    pub fn is_completed(&self) -> bool {
        self.current_word_index >= self.words.len()
    }

    pub fn get_wpm(&self) -> f64 {
        if let Some(start_time) = self.start_time {

            if let Some(stop_time) = self.stop_time {
                let elapsed = (stop_time - start_time).as_secs_f64() / 60.0; // Convert to minutes
                if elapsed > 0.0 {
                    let words_typed = self.typed_text.split_whitespace().count() as f64;
                    return words_typed / elapsed;
                }
            }

            let elapsed = start_time.elapsed().as_secs_f64() / 60.0; // Convert to minutes
            if elapsed > 0.0 {
                let words_typed = self.typed_text.split_whitespace().count() as f64;
                return words_typed / elapsed;
            }
        }
        0.0
    }

    pub fn get_accuracy(&self) -> f64 {
        if self.total_characters > 0 {
            let correct_chars = self.total_characters - self.errors;
            (correct_chars as f64 / self.total_characters as f64) * 100.0
        } else {
            100.0
        }
    }
}

impl Word {
    pub fn get_display_state(&self) -> WordDisplayState {
        if self.is_completed {
            if self.is_correct {
                WordDisplayState::Correct
            } else {
                WordDisplayState::Incorrect
            }
        } else if !self.typed.is_empty() {
            WordDisplayState::Typing
        } else {
            WordDisplayState::Untyped
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WordDisplayState {
    Untyped,
    Typing,
    Correct,
    Incorrect,
}

// Sample sentences for typing practice
pub const SAMPLE_SENTENCES: &[&str] = &[
    "The quick brown fox jumps over the lazy dog",
    "Pack my box with five dozen liquor jugs",
    "How vexingly quick daft zebras jump",
    "Waltz bad nymph for quick jigs vex",
    "Programming is the art of telling another human what one wants the computer to do",
    "Code is like humor when you have to explain it its bad",
    "First solve the problem then write the code",
    "The best error message is the one that never shows up",
];






