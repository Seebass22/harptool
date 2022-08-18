use std::collections::BTreeMap;

#[rustfmt::skip]
pub fn get_scales() -> BTreeMap<String, Vec<&'static str>> {
    let mut scales = BTreeMap::new();
    scales.insert(String::from("ionian"), vec!["1", "2", "3", "4", "5", "6", "7"]);
    scales.insert(String::from("major"), vec!["1", "2", "3", "4", "5", "6", "7"]);

    scales.insert(String::from("dorian"), vec!["1", "2", "b3", "4", "5", "6", "b7"]);

    scales.insert(String::from("phrygian"), vec!["1", "b2", "b3", "4", "5", "b6", "b7"]);

    scales.insert(String::from("mixolydian"), vec!["1", "2", "3", "4", "5", "6", "b7"]);

    scales.insert(String::from("lydian"), vec!["1", "2", "3", "#4", "5", "6", "7"]);

    scales.insert(String::from("aeolian"), vec!["1", "2", "b3", "4", "5", "b6", "b7"]);
    scales.insert(String::from("minor"), vec!["1", "2", "b3", "4", "5", "b6", "b7"]);
    scales.insert(String::from("natural minor"), vec!["1", "2", "b3", "4", "5", "b6", "b7"]);

    scales.insert(String::from("locrian"), vec!["1", "b2", "b3", "4", "#4", "b6", "b7"]);

    scales.insert(String::from("major pentatonic"), vec!["1", "2", "3", "5", "6"]);

    scales.insert(String::from("minor pentatonic"), vec!["1", "b3", "4", "5", "b7"]);

    scales.insert(String::from("blues"), vec!["1", "b3", "4", "#4", "5", "b7"]);
    scales.insert(String::from("minor blues"), vec!["1", "b3", "4", "#4", "5", "b7"]);

    scales.insert(String::from("major blues"), vec!["1", "2", "b3", "3", "5", "6"]);

    scales.insert(String::from("harmonic minor"), vec!["1", "2", "b3", "4", "5", "b6", "7"]);

    scales.insert(String::from("melodic minor"), vec!["1", "2", "b3", "4", "5", "6", "7"]);

    scales.insert(String::from("phrygian dominant"), vec!["1", "b2", "3", "4", "5", "b6", "b7"]);

    scales.insert(String::from("double harmonic"), vec!["1", "b2", "3", "4", "5", "b6", "7"]);
    scales.insert(String::from("arabic"), vec!["1", "b2", "3", "4", "5", "b6", "7"]);

    scales.insert(String::from("lydian dominant"), vec!["1", "2", "3", "#4", "5", "6", "b7"]);
    scales.insert(String::from("acoustic"), vec!["1", "2", "3", "#4", "5", "6", "b7"]);
    scales
}

pub fn is_scale_note(note: &str, scale: &str) -> bool {
    let scales = get_scales();

    if let Some(scale) = scales.get(scale) {
        scale.contains(&note)
    } else {
        false
    }
}
