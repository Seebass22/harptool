use std::collections::HashMap;

pub fn is_scale_note(note: &str, scale: &str) -> bool {
    let mut scales = HashMap::new();
    // modes
    scales.insert(String::from("ionian"), vec!["1", "2", "3", "4", "5", "6", "7"]);
    scales.insert(String::from("major"), vec!["1", "2", "3", "4", "5", "6", "7"]);

    scales.insert(String::from("dorian"), vec!["1", "2", "b3", "4", "5", "6", "b7"]);

    scales.insert(String::from("phrygian"), vec!["1", "b2", "b3", "4", "5", "b6", "b7"]);

    scales.insert(String::from("lydian"), vec!["1", "2", "3", "#4", "5", "6", "7"]);

    scales.insert(String::from("mixolydian"), vec!["1", "2", "3", "4", "5", "6", "b7"]);

    scales.insert(String::from("aeolian"), vec!["1", "2", "b3", "4", "5", "b6", "b7"]);
    scales.insert(String::from("minor"), vec!["1", "2", "b3", "4", "5", "b6", "b7"]);
    scales.insert(String::from("natural_minor"), vec!["1", "2", "b3", "4", "5", "b6", "b7"]);

    scales.insert(String::from("locrian"), vec!["1", "b2", "b3", "4", "b5", "b6", "b7"]);

    // pentatonic/blues
    scales.insert(String::from("major_pentatonic"), vec!["1", "2", "3", "5", "6"]);

    scales.insert(String::from("minor_pentatonic"), vec!["1", "b3", "4", "5", "b7"]);

    scales.insert(String::from("blues"), vec!["1", "b3", "4", "#4", "5", "b7"]);
    scales.insert(String::from("minor_blues"), vec!["1", "b3", "4", "#4", "5", "b7"]);

    scales.insert(String::from("major_blues"), vec!["1", "2", "b3", "3", "5", "6"]);

    // variations/other
    scales.insert(String::from("harmonic_minor"), vec!["1", "2", "b3", "4", "5", "b6", "7"]);

    scales.insert(String::from("melodic_minor"), vec!["1", "2", "b3", "4", "5", "6", "7"]);

    scales.insert(String::from("phrygian_dominant"), vec!["1", "b2", "3", "4", "5", "b6", "b7"]);

    scales.insert(String::from("double_harmonic"), vec!["1", "b2", "3", "4", "5", "b6", "7"]);
    scales.insert(String::from("arabic"), vec!["1", "b2", "3", "4", "5", "b6", "7"]);

    scales.insert(String::from("lydian_dominant"), vec!["1", "2", "3", "#4", "5", "6", "b7"]);
    scales.insert(String::from("acoustic"), vec!["1", "2", "3", "#4", "5", "6", "b7"]);

    if let Some(scale) = scales.get(scale) {
        scale.contains(&note)
    } else {
        false
    }
}
