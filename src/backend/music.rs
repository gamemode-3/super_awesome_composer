use std::fmt::{format, Display};

#[derive(Debug)]
pub struct Project {
    pub metadata: ProjectMetadata,
    pub content: ProjectContent,
}

impl Project {
    pub fn new(metadata: ProjectMetadata, content: ProjectContent) -> Self {
        Project { metadata, content }
    }
}

impl Default for Project {
    fn default() -> Self {
        Project::new(ProjectMetadata::default(), ProjectContent::default())
    }
}

impl core::fmt::Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "--- metadata ---\n{}\n----------------\n{}", self.metadata, self.content)
    }
}

#[derive(Debug)]
pub struct ProjectMetadata {}

impl ProjectMetadata {
    pub fn new() -> Self {
        ProjectMetadata {}
    }
}

impl Default for ProjectMetadata {
    fn default() -> Self {
        ProjectMetadata {}
    }
}

impl core::fmt::Display for ProjectMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct ProjectContent {
    pub name: String,
    pub artist: String,
    pub composer: Option<String>,
    pub tempo: Vec<(TimePoint, f32)>,
    pub time_signature: Vec<(TimePoint, TimeSignature)>,
    pub key: Vec<(TimePoint, Key)>,
    pub tracks: Vec<Track>,
}

impl ProjectContent {
    pub fn new(
        name: String,
        artist: String,
        composer: Option<String>,
        tempo: Vec<(TimePoint, f32)>,
        time_signature: Vec<(TimePoint, TimeSignature)>,
        key: Vec<(TimePoint, Key)>,
        tracks: Vec<Track>,
    ) -> Self {
        assert!(!tempo.is_empty());
        assert!(!time_signature.is_empty());
        assert!(!key.is_empty());
        ProjectContent {
            name,
            artist,
            composer,
            tempo: tempo,
            time_signature: time_signature,
            key: key,
            tracks,
        }
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn with_tempo(mut self, tempo: f32) -> Self {
        self.tempo[0] = (TimePoint::new(0, 0.0), tempo);
        self
    }

    pub fn with_time_signature(mut self, time_signature: TimeSignature) -> Self {
        self.time_signature[0] = (TimePoint::new(0, 0.0), time_signature);
        self
    }

    pub fn with_key(mut self, key: Key) -> Self {
        self.key[0] = (TimePoint::new(0, 0.0), key);
        self
    }

    pub fn with_track(mut self, track: Track) -> Self {
        self.tracks.push(track);
        self
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name
    }

    pub fn set_tempo(&mut self, tempo: f32) {
        self.tempo[0] = (TimePoint::new(0, 0.0), tempo)
    }

    pub fn set_time_signature(&mut self, time_signature: TimeSignature) {
        self.time_signature[0] = (TimePoint::new(0, 0.0), time_signature)
    }

    pub fn set_track(&mut self, track: Track) {
        self.tracks.push(track)
    }
}

impl Default for ProjectContent {
    fn default() -> Self {
        ProjectContent {
            name: "Unnamed Score".to_string(),
            artist: "Unknown Artist".to_string(),
            composer: None,
            tempo: vec![(TimePoint::new(0, 0.0), 120.0)],
            time_signature: vec![(TimePoint::new(0, 0.0), TimeSignature::default())],
            key: vec![(TimePoint::new(0, 0.0), Key::default())],
            tracks: vec![],
        }
    }
}

impl core::fmt::Display for ProjectContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\nby {}\n", self.name, self.artist)?;
        if let Some(composer) = &self.composer {
            write!(f, "composed by {}\n", composer)?;
        }
        write!(f, "\n{}bpm\n", self.tempo[0].1)?;
        write!(f, "{}\n", self.time_signature[0].1)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct TimeSignature {
    pub units: Vec<TimeSignatureUnit>,
}

impl TimeSignature {
    pub fn new(units: Vec<TimeSignatureUnit>) -> Self {
        TimeSignature { units }
    }
}

impl Default for TimeSignature {
    fn default() -> Self {
        TimeSignature::new(vec![TimeSignatureUnit::default()])
    }
}

impl core::fmt::Display for TimeSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.units
                .iter()
                .map(|u| format!("{}", u))
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

#[derive(Debug)]
pub struct Key {
    pub name: String,
    pub root: f32,
}

impl Default for Key {
    fn default() -> Self {
        Key {
            name: "C".to_string(),
            root: 0.0,
        }
    }
}

#[derive(Debug)]
pub struct TimeSignatureUnit {
    pub repititions: u32,
    pub beat: Vec<RhythmPoint>,
}

impl TimeSignatureUnit {
    pub fn new(repititions: u32, beat: Vec<RhythmPoint>) -> Self {
        TimeSignatureUnit { repititions, beat }
    }
}

impl core::fmt::Display for TimeSignatureUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = format!("{}", self.repititions);
        for b in &self.beat {
            s = format!("{}{}", s, b);
        }
        write!(f, "{}", s)
    }
}

impl Default for TimeSignatureUnit {
    fn default() -> Self {
        TimeSignatureUnit::new(4, vec![RhythmPoint::new(0)])
    }
}

#[derive(Debug)]
pub struct Track {
    pub instrument: Instrument,
    notes: Vec<Note>,
    rhythm_points: Vec<RhythmPoint>,
}

impl Track {
    pub fn new(instrument: Instrument, notes: Vec<Note>, rhythm_points: Vec<RhythmPoint>) -> Self {
        Track {
            instrument,
            notes,
            rhythm_points,
        }
    }
}

impl Default for Track {
    fn default() -> Self {
        Track {
            instrument: Instrument::default(),
            notes: vec![],
            rhythm_points: vec![],
        }
    }
}

#[derive(Debug)]
pub struct Instrument {}

impl Instrument {
    pub fn new() -> Self {
        Instrument {}
    }
}

impl Default for Instrument {
    fn default() -> Self {
        Instrument {}
    }
}

#[derive(Debug)]
pub struct Note {
    start_measure: u32,
    end_measure: u32,
    start_beat: f32,
    end_beat: f32,
    pitch_points: Vec<PitchPoint>,
}

impl Note {
    pub fn new(
        start_measure: u32,
        end_measure: u32,
        start_beat: f32,
        end_beat: f32,
        pitch_points: Vec<PitchPoint>,
    ) -> Self {
        Note {
            start_measure,
            end_measure,
            start_beat,
            end_beat,
            pitch_points,
        }
    }
}

#[derive(Debug)]
pub struct PitchPoint {
    pitch: f32,
    velocity: f32,
    rel_beat: f32,
}

#[derive(Debug)]
pub struct RhythmPoint {
    divisions: u32,
    connect_next: Option<bool>,
}

impl RhythmPoint {
    pub fn new(divisions: u32) -> Self {
        RhythmPoint {
            divisions,
            connect_next: None,
        }
    }
}

impl core::fmt::Display for RhythmPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.divisions {
                0 => "・".to_string(),
                1 => ":".to_string(),
                2 => "⋮".to_string(),
                _ => format!("{{{}}}", &"o".repeat(self.divisions as usize)),
            }
        )
    }
}

#[derive(Debug)]
pub struct TimePoint {
    measure: u32,
    beat: f32,
}

impl TimePoint {
    pub fn new(measure: u32, beat: f32) -> Self {
        TimePoint { measure, beat }
    }
}

impl core::fmt::Display for TimePoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.measure, self.beat)
    }
}