use std::collections::BTreeMap;

#[rustfmt::skip]
pub fn get_scales() -> BTreeMap<String, Vec<&'static str>> {
    BTreeMap::from([
        (String::from("ionian"), vec!["1", "2", "3", "4", "5", "6", "7"]),
        (String::from("major"), vec!["1", "2", "3", "4", "5", "6", "7"]),

        (String::from("dorian"), vec!["1", "2", "b3", "4", "5", "6", "b7"]),

        (String::from("phrygian"), vec!["1", "b2", "b3", "4", "5", "b6", "b7"]),

        (String::from("mixolydian"), vec!["1", "2", "3", "4", "5", "6", "b7"]),

        (String::from("lydian"), vec!["1", "2", "3", "#4", "5", "6", "7"]),

        (String::from("aeolian"), vec!["1", "2", "b3", "4", "5", "b6", "b7"]),
        (String::from("minor"), vec!["1", "2", "b3", "4", "5", "b6", "b7"]),
        (String::from("natural minor"), vec!["1", "2", "b3", "4", "5", "b6", "b7"]),

        (String::from("locrian"), vec!["1", "b2", "b3", "4", "#4", "b6", "b7"]),

        (String::from("major pentatonic"), vec!["1", "2", "3", "5", "6"]),

        (String::from("minor pentatonic"), vec!["1", "b3", "4", "5", "b7"]),

        (String::from("blues"), vec!["1", "b3", "4", "#4", "5", "b7"]),
        (String::from("minor blues"), vec!["1", "b3", "4", "#4", "5", "b7"]),

        (String::from("major blues"), vec!["1", "2", "b3", "3", "5", "6"]),

        (String::from("harmonic minor"), vec!["1", "2", "b3", "4", "5", "b6", "7"]),

        (String::from("melodic minor"), vec!["1", "2", "b3", "4", "5", "6", "7"]),

        (String::from("phrygian dominant"), vec!["1", "b2", "3", "4", "5", "b6", "b7"]),

        (String::from("double harmonic"), vec!["1", "b2", "3", "4", "5", "b6", "7"]),
        (String::from("arabic"), vec!["1", "b2", "3", "4", "5", "b6", "7"]),

        (String::from("lydian dominant"), vec!["1", "2", "3", "#4", "5", "6", "b7"]),
        (String::from("acoustic"), vec!["1", "2", "3", "#4", "5", "6", "b7"]),
    ])
}

pub fn is_scale_note(note: &str, scale: &str) -> bool {
    let scales = get_scales();

    if let Some(scale) = scales.get(scale) {
        scale.contains(&note)
    } else {
        false
    }
}
