use std::fs;
use std::io::Write;
use std::path::PathBuf;
use colored::*;
use tunings::get_tunings;

pub mod scales;
pub mod tunings;

pub struct Setup <'a> {
    pub scale: Option<&'a str>,
    pub position: usize,
}

#[derive(Debug)]
pub struct ChromaticScale {
    pub root: String,
    pub notes: Vec<String>
}

#[derive(Debug, PartialEq, Clone)]
pub struct Tuning {
    pub blow: Vec<Option<usize>>,
    pub draw: Vec<Option<usize>>,
    pub bends_half: Vec<Option<usize>>,
    pub bends_full: Vec<Option<usize>>,
    pub bends_one_and_half: Vec<Option<usize>>,
    pub blow_bends_half: Vec<Option<usize>>,
    pub blow_bends_full: Vec<Option<usize>>,
    pub overblows: Vec<Option<usize>>,
    pub overdraws: Vec<Option<usize>>,
}

impl Default for Tuning {
    fn default() -> Tuning {
        Tuning {
            blow: vec![Some(0), Some(4), Some(7), Some(0), Some(4), Some(7), Some(0), Some(4), Some(7), Some(0)],
            draw: vec![Some(2), Some(7), Some(11), Some(2), Some(5), Some(9), Some(11), Some(2), Some(5), Some(9)],
            bends_half: vec![Some(1), Some(6), Some(10), Some(1), None, Some(8), None, None, None, None],
            bends_full: vec![None, Some(5), Some(9), None, None, None, None, None, None, None],
            bends_one_and_half: vec![None, None, Some(8), None, None, None, None, None, None, None],
            blow_bends_half: vec![None, None, None, None, None, None, None, Some(3), Some(6), Some(11)],
            blow_bends_full: vec![None, None, None, None, None, None, None, None, None, Some(10)],
            overblows: vec![Some(3), None, None, Some(3), Some(6), Some(10), None, None, None, None],
            overdraws: vec![None, None, None, None, None, None, Some(1), None, Some(8), Some(1)],
        }
    }
}

impl From<&str> for Tuning {
    fn from(s: &str) -> Tuning {
        let (top, bottom) = str_to_rows(s);
        Tuning::new(top, bottom)
    }
}

impl Tuning {
    pub fn new(top_notes: Vec<usize>, bottom_notes: Vec<usize>) -> Tuning {
        let blow: Vec<Option<usize>> = top_notes.iter().map(|x| Some(*x)).collect();
        let draw: Vec<Option<usize>> = bottom_notes.iter().map(|x| Some(*x)).collect();
        let len = top_notes.len();
        let mut bends_half: Vec<Option<usize>> = vec![None; len];
        let mut bends_full: Vec<Option<usize>> = vec![None; len];
        let mut bends_one_and_half: Vec<Option<usize>> = vec![None; len];
        let mut blow_bends_half: Vec<Option<usize>> = vec![None; len];
        let mut blow_bends_full: Vec<Option<usize>> = vec![None; len];
        let mut overblows: Vec<Option<usize>> = vec![None; len];
        let mut overdraws: Vec<Option<usize>> = vec![None; len];

        let top_notes = adjust_octaves(&top_notes);
        let bottom_notes = adjust_octaves(&bottom_notes);
        let (_, duplicates) = notes_in_order(&top_notes, &bottom_notes);

        for (i, (top, bottom)) in top_notes.iter().zip(bottom_notes).enumerate() {
            let top = *top;
            let bottom = bottom;

            if bottom > top {
                let _ = overblows.get_mut(i).unwrap().insert((bottom + 1) % 12);

                if bottom - top >= 4 {
                    let _ = bends_one_and_half.get_mut(i).unwrap().insert((bottom - 3) % 12);
                }
                if bottom - top >= 3 {
                    let _ = bends_full.get_mut(i).unwrap().insert((bottom - 2) % 12);
                }
                if bottom - top >= 2 {
                    let _ = bends_half.get_mut(i).unwrap().insert((bottom - 1) % 12);
                }
            } else {
                let _ = overdraws.get_mut(i).unwrap().insert((top + 1) % 12);

                if top - bottom >= 3 {
                    let _ = blow_bends_full.get_mut(i).unwrap().insert((top - 2) % 12);
                }
                if top - bottom >= 2 {
                    let _ = blow_bends_half.get_mut(i).unwrap().insert((top - 1) % 12);
                }
            }
        }

        // remove duplicate overblows/overdraws
        for note in duplicates.iter().step_by(2) {
            let mut note = note.clone();

            if note.contains('o') {
                note.pop();
                let note = note.parse::<i32>().unwrap();

                if note < 0 {
                    let index = ((-note) -1) as usize;
                    overdraws.get_mut(index).unwrap().take();
                } else {
                    let index = (note -1) as usize;
                    overblows.get_mut(index).unwrap().take();
                }
            }
        }

        Tuning {
            blow,
            draw,
            bends_half,
            bends_full,
            bends_one_and_half,
            blow_bends_half,
            blow_bends_full,
            overblows,
            overdraws,
        }
    }

    fn print_row(row: &[Option<usize>], setup: &Setup) {
        for x in row {
            let n  = match x {
                None => String::from(" "),
                Some(x) => to_scale_degree(*x, setup.position),
            };
            Tuning::print_colorized(setup.scale, &n[..], &n[..]);
        }
        println!();
    }

    fn print_colorized(scale: Option<&str>, degree: &str, note: &str) {
        if let Some(scale) = scale {
            if scales::is_scale_note(degree, scale) {
                print!("{:width$} ", note.green(), width = 3);
            } else {
                print!("{:width$} ", note, width = 3);
            }
        } else {
            print!("{:width$} ", note, width = 3);
        }
    }

    fn print_scale_degrees(&self, setup: Setup) {
        print!("{:width$} ", "overblows", width = 20);
        Tuning::print_row(&self.overblows, &setup);

        print!("{:width$} ", "blow bends full step", width = 20);
        Tuning::print_row(&self.blow_bends_full, &setup);
        print!("{:width$} ", "blow bends half step", width = 20);
        Tuning::print_row(&self.blow_bends_half, &setup);
        print!("{:width$} ", "blow", width = 20);
        Tuning::print_row(&self.blow, &setup);

        self.print_number_row();

        print!("{:width$} ", "draw", width = 20);
        Tuning::print_row(&self.draw, &setup);
        print!("{:width$} ", "bends half step", width = 20);
        Tuning::print_row(&self.bends_half, &setup);
        print!("{:width$} ", "bends full step", width = 20);
        Tuning::print_row(&self.bends_full, &setup);
        print!("{:width$} ", "bends 1 1/2 step", width = 20);
        Tuning::print_row(&self.bends_one_and_half, &setup);

        print!("{:width$} ", "overdraws", width = 20);
        Tuning::print_row(&self.overdraws, &setup);
    }

    fn print_number_row(&self) {
        let mut numbers = String::from("");
        numbers.push('1');
        for i in 1..self.blow.len() {
            numbers.push_str("   ");
            let i = i+1;
            numbers.push_str(&i.to_string());
        }
        println!("{:width$} {}", "", numbers.blue(), width = 20);
    }

    fn print_layout(&self, root: &ChromaticScale, setup: Setup) {
        print!("{:width$} ", "overblows", width = 20);
        Tuning::print_row_notes(&self.overblows, root, &setup);

        print!("{:width$} ", "blow bends full step", width = 20);
        Tuning::print_row_notes(&self.blow_bends_full, root, &setup);
        print!("{:width$} ", "blow bends half step", width = 20);
        Tuning::print_row_notes(&self.blow_bends_half, root, &setup);
        print!("{:width$} ", "blow", width = 20);
        Tuning::print_row_notes(&self.blow, root, &setup);

        self.print_number_row();

        print!("{:width$} ", "draw", width = 20);
        Tuning::print_row_notes(&self.draw, root, &setup);
        print!("{:width$} ", "bends half step", width = 20);
        Tuning::print_row_notes(&self.bends_half, root, &setup);
        print!("{:width$} ", "bends full step", width = 20);
        Tuning::print_row_notes(&self.bends_full, root, &setup);
        print!("{:width$} ", "bends 1 1/2 step", width = 20);
        Tuning::print_row_notes(&self.bends_one_and_half, root, &setup);

        print!("{:width$} ", "overdraws", width = 20);
        Tuning::print_row_notes(&self.overdraws, root, &setup);
    }

    fn print_row_notes(indices: &[Option<usize>], root: &ChromaticScale, setup: &Setup) {
        //                   0     1    2    3     4    5     6     7    8    9    10   11
        // let notes = vec!["C", "Dd", "D", "Eb", "E", "F", "F#", "G", "Ab", "A", "Bb", "B"];
        let notes = &root.notes;

        for i in indices {
            let n = match *i {
                Some(n) => notes.get(n).unwrap().to_string(),
                None => String::from(" "),
            };

            let degree: String;
            if let Some(index) = *i {
                degree = to_scale_degree(index, setup.position);
            } else {
                degree = String::from("");
            }
            Tuning::print_colorized(setup.scale, &degree, &n);
        }
        println!();
    }
}

impl ChromaticScale {
    pub fn new(note: &str, use_sharps: Option<bool>) -> ChromaticScale {
        let sharp;
        if let Some(value) = use_sharps {
            sharp = value;
            if (sharp && vec!["Bb", "Eb", "Ab", "Db", "Gb", "Cb", "Fb"].contains(&note)) ||
                (! sharp && vec!["F#", "C#", "G#", "D#", "A#", "E#", "B#"].contains(&note)) {
                    panic!("cannot choose sharp/flat notes if root is sharp/flat");
                }
        } else {
            sharp = !vec!["Bb", "Eb", "Ab", "Db", "Gb", "F"].contains(&note);
        }

        if ! vec!["F", "C", "G", "D", "A", "E", "B",
                  "Bb", "Eb", "Ab", "Db", "Gb",
                  "F#", "C#", "G#", "D#", "A#",]
            .contains(&note) {
                panic!("invalid root note");
            }

        let notes = if sharp {
            vec!["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"]
        } else {
            vec!["C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab", "A", "Bb", "B"]
        };

        let mut pos = notes.iter().position(|&n| n == note).unwrap();
        let mut v = Vec::new();
        for _i in 0..notes.len() {
            v.push(notes.get(pos % 12).unwrap().to_string());
            pos += 1;
        }

        ChromaticScale {
            root: note.to_string(),
            notes: v,
        }
    }
}

pub fn run(tuning: &str, key: &str, sharp: Option<bool>, setup: Setup) {
    let tuning = read_tuning_from_hashmap_or_file(tuning);
    let v = ChromaticScale::new(key, sharp);
    tuning.print_layout(&v, setup);
}

pub fn run_degrees(tuning: &str, setup: Setup) {
    let tuning = read_tuning_from_hashmap_or_file(tuning);
    tuning.print_scale_degrees(setup);
}

// C E G C E G -> 0 4  7 0 4 7
// D G B D F A -> 2 7 11 2 5 9
fn convert_to_numbers(top: Vec<&str>, bottom: Vec<&str>) -> (Vec<usize>, Vec<usize>) {
    let mut top_numbers: Vec<usize> = Vec::new();
    let mut bottom_numbers: Vec<usize> = Vec::new();

    let flat_notes = vec!["Bb", "Eb", "Ab", "Db", "Gb"];
    let sharp = ! (top.iter().any(|s| flat_notes.contains(s)) || bottom.iter().any(|s| flat_notes.contains(s)));

    let scale = ChromaticScale::new(top.get(0).unwrap(), Some(sharp));

    for note in top.iter() {
        top_numbers.push(
            scale.notes.iter().position(|x| note == x).unwrap()
        );
    }
    for note in bottom.iter() {
        bottom_numbers.push(
            scale.notes.iter().position(|x| note == x).unwrap()
        );
    }
    (top_numbers, bottom_numbers)
}

fn to_scale_degree(index: usize, position: usize) -> String {
    let index = (index + (position-1)*5) % 12;
    let degrees = ["1", "b2", "2", "b3", "3", "4", "#4", "5", "b6", "6", "b7", "7"];
    String::from(degrees[index])
}

fn read_tuning_from_hashmap_or_file(tuning_name: &str) -> Tuning {
    let tunings = get_tunings();
    let contents = if let Some(notes) = tunings.get(tuning_name) {
        notes.to_string()
    } else {
        let mut filepath = dirs::config_dir().unwrap();
        filepath.push("harptool");
        if ! filepath.is_dir() {
            write_default_layouts();
        }

        filepath.push(tuning_name);

         match fs::read_to_string(&filepath) {
            Ok(contents) => contents,
            Err(_) => {
                eprintln!("tuning file \"{}\" not found", filepath.to_string_lossy());
                std::process::exit(-1);
            }
        }
    };
    let (top, bottom) = str_to_rows(&contents);

    Tuning::new(top, bottom)
}

pub fn str_to_notes_in_order(input: &str) -> (Vec<String>, Vec<String>) {
    let (top, bottom) = str_to_rows(input);
    let top = adjust_octaves(&top);
    let bottom = adjust_octaves(&bottom);
    notes_in_order(&top, &bottom)
}

// "C E G\nD G B" -> [0 4 7], [2 7 11]
fn str_to_rows(input: &str) -> (Vec<usize>, Vec<usize>) {
    let contents: Vec<String> = input
        .lines()
        .map(|s| s.to_string())
        .collect();
    let top: Vec<&str> = contents.get(0).unwrap()
        .split(' ').collect();
    let bottom: Vec<&str> = contents.get(1).unwrap()
        .split(' ').collect();

    convert_to_numbers(top, bottom)
}

fn write_default_layouts() {
    fn write_layout(tuning: &str, name: &str, path: &mut PathBuf) {
        path.push(name);
        let mut f = fs::File::create(&path).unwrap();
        f.write_all(tuning.as_bytes()).unwrap();
        path.pop();
    }

    let richter = "C E G C E G C E G C\nD G B D F A B D F A\n";
    let country = "C E G C E G C E G C\nD G B D F# A B D F A\n";
    let wilde = "C E G C E E G C E A\nD G B D F G B D G C\n";
    let melody_maker = "C E A C E G C E G C\nD G B D F# A B D F# A\n";
    let natural_minor = "C Eb G C Eb G C Eb G C\nD G Bb D F A Bb D F A\n";
    let harmonic_minor = "C Eb G C Eb G C Eb G C\nD G B D F Ab B D F Ab\n";
    let paddy_richter = "C E A C E G C E G C\nD G B D F A B D F A\n";

    let mut filepath = dirs::config_dir().unwrap();
    filepath.push("harptool");
    fs::create_dir_all(&filepath).expect("could not create note layout dir");

    write_layout(richter, "richter", &mut filepath);
    write_layout(country, "country", &mut filepath);
    write_layout(wilde, "wilde", &mut filepath);
    write_layout(melody_maker, "melody_maker", &mut filepath);
    write_layout(natural_minor, "natural_minor", &mut filepath);
    write_layout(harmonic_minor, "harmonic_minor", &mut filepath);
    write_layout(paddy_richter, "paddy_richter", &mut filepath);
}

// 0 4 7 0 4 7 -> 0 4 7 12 16 19
fn adjust_octaves(row: &[usize]) -> Vec<usize> {
    let mut row = row.to_vec();
    let mut last = 0;
    let mut add = 0;
    for x in row.iter_mut() {
        if *x < last {
            add += 1;
        } 
        last = *x;
        *x += 12 * add;
    }
    row
}

fn notes_in_order(top: &[usize], bottom: &[usize]) -> (Vec<String>, Vec<String>) {
    fn getnote(hole: i32, bends: i32, overblow: bool) -> String {
        let mut hole = hole.to_string();
        if overblow {
            hole.push('o');
        }
        for _ in 0..bends {
            hole.push('\'');
        }
        hole
    }
    let harplen = top.len() as i32;
    let top: Vec<i32> = top.iter().map(|x| *x as i32).collect();
    let bottom: Vec<i32> = bottom.iter().map(|x| *x as i32).collect();

    let mut hole = 1;
    let mut accounted = -1;
    let mut lastdirection = 1;
    let mut lasthigher = 0;
    let mut res: Vec<String> = Vec::new();
    let mut duplicated: Vec<String> = Vec::new();
    let mut alternative;

    for (top, bottom) in top.iter().zip(bottom.iter()) {
        let higher;
        let lower;
        let direction;
        let mut note;
        let mut ob_duplicated = false;

        if *bottom > *top {
            higher = *bottom;
            lower = *top;
            direction = 1;
        } else {
            higher = *top;
            lower = *bottom;
            direction = -1;
        }

        // overblows
        if (lower - accounted) > 1 {
            res.push(getnote(lastdirection * (hole-1), 0, true));
            accounted += 1;
            // fix case if notes still missing after adding overblow
            while (lower - accounted) > 1 {
                res.push("X".to_string());
                accounted += 1;
            }
        } else if hole > 1 {
            // replacement note can only be found after evaluating blow, draw and bent notes
            ob_duplicated = true;
        }

        // lower note
        note = getnote(direction * hole, 0, false);
        if lower > accounted {
            res.push(note);
            accounted += 1;
        } else {
            alternative = res.get(lower as usize).unwrap();
            duplicated.push(note.clone());
            duplicated.push(alternative.clone());
        }

        // bends
        let bends = higher - lower;
        for step in (1..bends).rev() {
            note = getnote(direction * -hole, step, false);
            if (higher - step) > accounted {
                res.push(note);
                accounted += 1;
            } else {
                alternative = res.get((higher-step) as usize).unwrap();
                duplicated.push(note.clone());
                duplicated.push(alternative.clone());
            }
        }

        // higher note
        note = getnote(direction * -hole, 0, false);
        if higher > accounted {
            res.push(note);
            accounted += 1;
        } else {
            alternative = res.get(lower as usize).unwrap();
            duplicated.push(note.clone());
            duplicated.push(alternative.clone());
        }

        // last hole overblow/overdraw
        if (hole == harplen) && (accounted == higher) {
            res.push(getnote(direction * hole, 0, true));
            accounted += 1;
        }

        // find replacement for overblow/overdraw
        if ob_duplicated {
            note = getnote(lastdirection * (hole-1), 0, true);
            alternative = res.get((lasthigher+1) as usize).unwrap();
            duplicated.push(note.clone());
            duplicated.push(alternative.clone());
        }

        hole += 1;
        lastdirection = direction;
        lasthigher = higher;
    }
    (res, duplicated)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tuning_new() {
        let tuning = Tuning::new(
            vec![0, 4, 7, 0, 4, 7, 0, 4, 7, 0],
            vec![2, 7, 11, 2, 5, 9, 11, 2, 5, 9],
        );
        assert_eq!(tuning, Tuning::default());
    }

    #[test]
    fn test_tuning_from_str() {
        let richter = "C E G C E G C E G C\nD G B D F A B D F A\n";
        let tuning = Tuning::from(richter);
        assert_eq!(tuning, Tuning::default());
    }

    #[test]
    fn test_convert_to_numbers() {
        let top = vec!["C", "E", "G", "C", "E", "G", "C", "E", "G", "C"];
        let bottom = vec!["D", "G", "B", "D", "F", "A", "B", "D", "F", "A"];
        let top_numbers = vec![0, 4, 7, 0, 4, 7, 0, 4, 7, 0];
        let bottom_numbers = vec![2, 7, 11, 2, 5, 9, 11, 2, 5, 9];
        let (res_top, res_bottom) = convert_to_numbers(top, bottom);
        assert_eq!(top_numbers, res_top);
        assert_eq!(bottom_numbers, res_bottom);

        let top = vec!["D", "F#", "A", "D", "F#", "A", "D", "F#", "A", "D"];
        let bottom = vec!["E", "A", "C#", "E", "G", "B", "C#", "E", "G", "B"];
        let (res_top, res_bottom) = convert_to_numbers(top, bottom);
        assert_eq!(top_numbers, res_top);
        assert_eq!(bottom_numbers, res_bottom);

        let top = vec!["F", "A", "C", "F", "A", "C", "F", "A", "C", "F"];
        let bottom = vec!["G", "C", "E", "G", "Bb", "D", "E", "G", "Bb", "D"];
        let (res_top, res_bottom) = convert_to_numbers(top, bottom);
        assert_eq!(top_numbers, res_top);
        assert_eq!(bottom_numbers, res_bottom);
    }

    #[test]
    fn test_str_to_rows() {
        let richter = "C E G C E G C E G C\nD G B D F A B D F A\n";
        let (top, bottom) = str_to_rows(richter);
        let expected_top = vec![0, 4, 7, 0, 4, 7, 0, 4, 7, 0];
        let expected_bottom = vec![2, 7, 11, 2, 5, 9, 11, 2, 5, 9];
        assert_eq!(expected_bottom, bottom);
        assert_eq!(expected_top, top);
    }

    #[test]
    fn test_adjust_octaves() {
        let notes = vec![0, 4, 7, 0, 4, 7, 0, 4, 7, 0];
        let res = adjust_octaves(&notes);
        let expected = vec![0, 4, 7, 12, 16, 19, 24, 28, 31, 36];
        assert_eq!(res, expected);

        let notes = vec![2, 7, 11, 2, 5, 9, 11, 2, 5, 9];
        let res = adjust_octaves(&notes);
        let expected = vec![2, 7, 11, 14, 17, 21, 23, 26, 29, 33];
        assert_eq!(res, expected);

        let notes = vec![0, 4, 7, 0, 4, 4, 7, 0, 4, 9];
        let res = adjust_octaves(&notes);
        let expected = vec![0, 4, 7, 12, 16, 16, 19, 24, 28, 33];
        assert_eq!(res, expected);

        let notes = vec![2, 7, 11, 2, 5, 7, 11, 2, 7, 0];
        let res = adjust_octaves(&notes);
        let expected = [2, 7, 11, 14, 17, 19, 23, 26, 31, 36];
        assert_eq!(res, expected);
    }

    #[test]
    fn test_notes_in_order() {
        let top = vec![0, 4, 7, 12, 16, 19, 24, 28, 31, 36];
        let bottom = vec![2, 7, 11, 14, 17, 21, 23, 26, 29, 33];
        let res = notes_in_order(&top, &bottom);
        let expected = vec![
            "1", "-1'", "-1", "1o", "2", "-2''", "-2'", "-2", "-3'''", "-3''", "-3'", "-3", "4", "-4'",
            "-4", "4o", "5", "-5", "5o", "6", "-6'", "-6", "6o", "-7", "7", "-7o", "-8", "8'", "8",
            "-9", "9'", "9", "-9o", "-10", "10''", "10'", "10", "-10o",
        ];
        assert_eq!(res.0, expected);
    }

    #[test]
    fn test_str_to_notes_in_order() {
        let richter = "C E G C E G C E G C\nD G B D F A B D F A\n";
        let (notes, duplicated) = str_to_notes_in_order(richter);
        let expected = vec![
            "1", "-1'", "-1", "1o", "2", "-2''", "-2'", "-2", "-3'''", "-3''", "-3'", "-3", "4", "-4'",
            "-4", "4o", "5", "-5", "5o", "6", "-6'", "-6", "6o", "-7", "7", "-7o", "-8", "8'", "8",
            "-9", "9'", "9", "-9o", "-10", "10''", "10'", "10", "-10o",
        ];
        assert_eq!(notes, expected);
        let expected_duplicated = ["3", "-2", "2o", "-3'''", "3o", "4", "-8o", "-9"];
        assert_eq!(duplicated, expected_duplicated);

        let wilde = "C E G C E E G C E A\nD G B D F G B D G C\n";
        let res = str_to_notes_in_order(wilde);
        let expected = vec![
            "1", "-1'", "-1", "1o", "2", "-2''", "-2'", "-2", "-3'''", "-3''", "-3'", "-3", "4", "-4'",
            "-4", "4o", "5", "-5", "-6'", "-6", "-7'''", "-7''", "-7'", "-7", "8", "-8'", "-8", "8o",
            "9", "-9''", "-9'", "-9", "9o", "10", "-10''", "-10'", "-10", "10o",
        ];
        assert_eq!(res.0, expected);
    }

    #[test]
    fn test_notes_in_order_no_panics() {
        let mut tunings: Vec<&str> = Vec::new();
        tunings.push("C E G C E G C E G C\nD G B D F A B D F A\n");
        tunings.push("C E G C E G C E G C\nD G B D F# A B D F A\n");
        tunings.push("C E G C E E G C E A\nD G B D F G B D G C\n");
        tunings.push("C E A C E G C E G C\nD G B D F# A B D F# A\n",);
        tunings.push("C Eb G C Eb G C Eb G C\nD G Bb D F A Bb D F A\n",);
        tunings.push("C Eb G C Eb G C Eb G C\nD G B D F Ab B D F Ab\n",);
        tunings.push("C E A C E G C E G C\nD G B D F A B D F A\n",);
        tunings.push("A D E A D E A D E A\nC Eb G C Eb G C Eb G C");
        tunings.push("C E G C E G A C E A\nD G B D F A B D G C");
        tunings.push("C E G C D F A C E A\nD G B D E G B D G C");
        for tuning in tunings {
            let (_, _) = str_to_notes_in_order(tuning);
        }
    }
}
