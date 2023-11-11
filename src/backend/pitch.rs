#![allow(unused)]

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Accidental {
    Flat,
    Natural,
    Sharp,
}

impl std::fmt::Display for Accidental {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Accidental::Flat => write!(f, "b"),
            Accidental::Natural => write!(f, ""),
            Accidental::Sharp => write!(f, "#"),
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct NoteValue {
    pub note: char,
    pub accidental: Accidental,
    pub octave: i32,
}

impl std::fmt::Display for NoteValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.note, self.accidental, self.octave)
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct NoteValueResult {
    pub value: NoteValue,
    pub error: f32,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Pitch(f32);

impl Pitch {
    pub fn new(pitch: f32) -> Self {
        Pitch(pitch)
    }

    pub fn from_note(note: char, accidental: Accidental, octave: i32) -> Result<Self, String> {
        let mut note_value = match note {
            'c' => 0,
            'd' => 2,
            'e' => 4,
            'f' => 5,
            'g' => 7,
            'a' => 9,
            'b' => 11,
            _ => return Err(format!("invalid note character: {}", note)),
        };

        note_value += match accidental {
            Accidental::Flat => -1,
            Accidental::Natural => 0,
            Accidental::Sharp => 1,
        };

        Ok(Pitch::new((note_value + octave * 12) as f32))
    }

    pub fn from_string(string: &str) -> Result<Self, String> {
        if string.len() == 0 {
            return Err("string must be non-empty".to_string());
        }
        let mut chars = string.chars().peekable();

        let note = chars.next().unwrap().to_ascii_lowercase();

        let accidental = match chars.peek() {
            Some('#') => {
                chars.next();
                Accidental::Sharp
            }
            Some('b') => {
                chars.next();
                Accidental::Flat
            }
            Some(_) => Accidental::Natural,
            None => return Pitch::from_note(note, Accidental::Natural, 4),
        };

        let octave_string: String = chars.collect();

        let octave_result = octave_string.parse::<i32>();

        if let Ok(octave) = octave_result {
            return Pitch::from_note(note, accidental, octave);
        }

        if accidental == Accidental::Natural {
            return Err("expected accidental or octave".to_string());
        }
        return Err("expected octave".to_string());
    }

    pub fn from_frequency(frequency: f32) -> Self {
        // we need to convert from a frequency to our weird other measurement where 0 means C0, 3 means D#0, 12 means C1 etc.
        // that means we need to convert from into a logarithmic scale.

        let pitch = 12. * (frequency / 16.35160).log2();

        // pitch / 12. = (frequency / 16.35160).log2();

        Pitch::new(pitch)
    }

    pub fn from_midi(midi: u8) -> Self {
        Pitch::new(midi as f32 - 12.0)
    }

    pub fn frequency(&self) -> f32 {
        16.35160 * 2.0_f32.powf(self.0 / 12.0)
    }

    pub fn midi(&self) -> u8 {
        (self.0 + 12.0).round() as u8
    }

    pub fn to_note_value(&self) -> NoteValueResult {
        let rounded = self.0.round();
        let error = self.0 - rounded;
        let int_value = rounded as i32;

        let note = int_value % 12;
        let octave = int_value / 12;

        let (note, accidental) = match note {
            0 => ('C', Accidental::Natural),
            1 => ('C', Accidental::Sharp),
            2 => ('D', Accidental::Natural),
            3 => ('D', Accidental::Sharp),
            4 => ('E', Accidental::Natural),
            5 => ('F', Accidental::Natural),
            6 => ('F', Accidental::Sharp),
            7 => ('G', Accidental::Natural),
            8 => ('G', Accidental::Sharp),
            9 => ('A', Accidental::Natural),
            10 => ('A', Accidental::Sharp),
            11 => ('B', Accidental::Natural),
            _ => unreachable!(),
        };

        NoteValueResult {
            value: NoteValue {
                note,
                accidental,
                octave,
            },
            error,
        }
    }
}

impl std::fmt::Display for Pitch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let note_value = self.to_note_value();
        write!(f, "{}[{:+.2}]", note_value.value, note_value.error)
    }
}
