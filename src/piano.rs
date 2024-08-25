use crate::button::Button;
use std::fmt::Display;
use minifb::Key;

#[derive(Debug, Copy, Clone)]
enum Note {
    C,
    Db,
    D,
    Eb,
    E,
    F,
    Gb,
    G,
    Ab,
    A,
    Bb,
    B
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match *self {
            Note::C => write!(f, "C"),
            Note::Db => write!(f, "C#"),
            Note::D => write!(f, "D"),
            Note::Eb => write!(f, "D#"),
            Note::E => write!(f, "E"),
            Note::F => write!(f, "F"),
            Note::Gb => write!(f, "F#"),
            Note::G => write!(f, "G"),
            Note::Ab => write!(f, "G#"),
            Note::A => write!(f, "A"),
            Note::Bb => write!(f, "A#"),
            Note::B => write!(f, "B"),
        }
    }
}

fn switch_white_note(note: Note) -> Note {
    match note {
        Note::C => Note::D,
        Note::D => Note::E,
        Note::E => Note::F,
        Note::F => Note::G,
        Note::G => Note::A,
        Note::A => Note::B,
        Note::B => Note::C,
        _ => Note::C,
    }
}

fn switch_black_note(note: Note) -> Note {
    match note {
        Note::Db => Note::Eb,
        Note::Eb => Note::Gb,
        Note::Gb => Note::Ab,
        Note::Ab => Note::Bb,
        Note::Bb => Note::Db,
        _ => Note::Db,
    }  
}

fn switch_key(key: Key) -> Key {
    match key {
        Key::A => Key::S,
        Key::S => Key::D,
        Key::D => Key::F,
        Key::F => Key::G,
        Key::G => Key::H,
        Key::H => Key::J,
        Key::J => Key::K,
        Key::K => Key::L,
        Key::L => Key::A,
        _ => Key::A,
    }
}
fn switch_black_key(key: Key) -> Key {
    match key {
        Key::W => Key::E,
        Key::E => Key::T,
        Key::T => Key::Y,
        Key::Y => Key::U,
        Key::U => Key::O,
        Key::O => Key::P,
        Key::P => Key::W,
        _ => Key::W,
    }
}

pub fn white_keys(quantity: usize, width: usize, buffer_height: usize, height: usize) -> Vec<Button> {
    let mut keys: Vec<Button> = Vec::new();
    let mut horizontal_pos: usize = 0;
    let separation = width + 10;
    let mut pitch = 4;
    let mut note: Note = Note::C;
    let mut key: Key = Key::A;
    let mut counter = 1;

    for _ in 0..quantity {
        // Determine relative position of key
        let pitch_note: String = format!("{}{}", note, pitch);
        let button = Button::new(
            horizontal_pos, // Position in X axis
            (buffer_height / 2) - (height / 2), // Position in Y axis
            width, // Width
            height + (height / 2), // Height
            Some(key),
            0xC4C4C4,
            0x767676,
            Some(pitch_note),
        );
        keys.push(button);
        horizontal_pos += separation;
        note = switch_white_note(note);
        key = switch_key(key);
        if counter % 8 == 7 {
            pitch += 1;
        }
        counter += 1;
    }

    keys
}

pub fn black_keys(quantity: usize, mut width: usize, buffer_height: usize, height: usize) -> Vec<Button> {
    let original_width = width;
    width -= width / 3 ;
    let mut keys: Vec<Button> = Vec::new();
    let mut horizontal_pos: usize = width;
    let separation = width + 10 + (original_width / 3);
    let big_separation = original_width + separation + 10;
    let mut pitch = 4;
    let mut note: Note = Note::Db;
    let mut key: Key = Key::A;
    let mut counter = 1;

    for _ in 0..quantity {
        // Determine relative position of key
        let pitch_note: String = format!("{}{}", note, pitch);
        let button = Button::new(
            horizontal_pos, // Position in X axis
            (buffer_height / 2) - ((buffer_height / 3) / 2), // Position in Y axis
            width, // Width
            height, // Height
            Some(key),
            0x000000,
            0xC4C4C4,
            Some(pitch_note),
        );
        keys.push(button);
        let modulo = counter % 10;
        if counter != 1 && (modulo == 0 || modulo == 2 || modulo == 5 || modulo == 7) {
            horizontal_pos += big_separation;
        } else { 
            horizontal_pos += separation;
        }
        note = switch_black_note(note);
        key = switch_black_key(key);
        if counter % 6 == 5 {
            pitch += 1;
        }
        counter += 1;
    }
    keys
}
