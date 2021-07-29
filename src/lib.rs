use std::fs;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug)]
struct Scale {
    root: String,
    notes: Vec<String>
}

#[derive(Debug, PartialEq, Clone)]
pub struct Tuning {
    blow: Vec<Option<usize>>,
    draw: Vec<Option<usize>>,
    bends_half: Vec<Option<usize>>,
    bends_full: Vec<Option<usize>>,
    bends_one_and_half: Vec<Option<usize>>,
    blow_bends_half: Vec<Option<usize>>,
    blow_bends_full: Vec<Option<usize>>,
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
	}
    }
}

impl Tuning {
    pub fn new(top_notes: Vec<usize>, bottom_notes: Vec<usize>) -> Tuning {
	fn is_within_5_semitones(top: usize, bottom: usize) -> bool {
	    (bottom as i32 - top as i32).abs() < 5
	}

	let blow: Vec<Option<usize>> = top_notes.iter().map(|x| Some(*x)).collect();
	let draw: Vec<Option<usize>> = bottom_notes.iter().map(|x| Some(*x)).collect();
	let mut bends_half: Vec<Option<usize>> = vec![None, None, None, None, None, None, None, None, None, None];
	let mut bends_full: Vec<Option<usize>> = vec![None, None, None, None, None, None, None, None, None, None];
	let mut bends_one_and_half: Vec<Option<usize>> = vec![None, None, None, None, None, None, None, None, None, None];
	let mut blow_bends_half: Vec<Option<usize>> = vec![None, None, None, None, None, None, None, None, None, None];
	let mut blow_bends_full: Vec<Option<usize>> = vec![None, None, None, None, None, None, None, None, None, None];

	for (i, (top, bottom)) in top_notes.iter().zip(bottom_notes.clone()).enumerate() {
	    let mut top = *top;
	    let mut bottom = bottom;

	    if ! is_within_5_semitones(top, bottom) {
		if top > bottom {
		    bottom += 12;
		} else {
		    top += 12;
		}
	    }

	    if bottom > top {
		if bottom - top >= 4 {
		    bends_one_and_half.get_mut(i).unwrap().insert((bottom - 3) % 12);
		}
		if bottom - top >= 3 {
		    bends_full.get_mut(i).unwrap().insert((bottom - 2) % 12);
		}
		if bottom - top >= 2 {
		    bends_half.get_mut(i).unwrap().insert((bottom - 1) % 12);
		}
	    } else {
		if top - bottom >= 3 {
		    blow_bends_full.get_mut(i).unwrap().insert((top - 2) % 12);
		}
		if top - bottom >= 2 {
		    blow_bends_half.get_mut(i).unwrap().insert((top - 1) % 12);
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
	}
    }
}

impl Scale {
    fn new(note: &str, sharp_notes: Option<bool>) -> Scale {
	let sharp;
	if let Some(value) = sharp_notes {
	    sharp = value;
	    if (sharp && vec!["Bb", "Eb", "Ab", "Db", "Gb", "Cb", "Fb"].contains(&note)) ||
		(! sharp && vec!["F#", "C#", "G#", "D#", "A#", "E#", "B#"].contains(&note)) {
		    panic!("cannot choose sharp/flat notes if root is sharp/flat");
		}
	} else {
	    sharp = if vec!["Bb", "Eb", "Ab", "Db", "Gb", "Cb", "Fb"].contains(&note) {
		false
	    } else {
		true
	    }
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

	Scale {
	    root: note.to_string(),
	    notes: v,
	}
    }

    fn printrow(&self, indices: &Vec<Option<usize>>) {
	//                   0     1    2    3     4    5     6     7    8    9    10   11
	// let notes = vec!["C", "Dd", "D", "Eb", "E", "F", "F#", "G", "Ab", "A", "Bb", "B"];
	let notes = &self.notes;

	for i in indices {
	    let n = match *i {
		Some(n) => notes.get(n).unwrap().to_string(),
		None => String::from(" "),
	    };
	    print!("{:width$} ", n, width = 3);
	}
	print!("\n");
    }

    fn printlayout(&self, tuning: &Tuning) {
	print!("{:width$} ", "blow bends full step", width = 20);
	self.printrow(&tuning.blow_bends_full);
	print!("{:width$} ", "blow bends half step", width = 20);
	self.printrow(&tuning.blow_bends_half);
	print!("{:width$} ", "blow", width = 20);
	self.printrow(&tuning.blow);
	println!("{:width$} 1   2   3   4   5   6   7   8   9   10", "",  width = 20);
	print!("{:width$} ", "draw", width = 20);
	self.printrow(&tuning.draw);
	print!("{:width$} ", "bends half step", width = 20);
	self.printrow(&tuning.bends_half);
	print!("{:width$} ", "bends full step", width = 20);
	self.printrow(&tuning.bends_full);
	print!("{:width$} ", "bends 1 1/2 step", width = 20);
	self.printrow(&tuning.bends_one_and_half);
    }
}

pub fn run(tuning: &str, key: &str, sharp: Option<bool>) {
    let tuning = read_tuning_from_file(tuning);
    let v = Scale::new(key, sharp);
    v.printlayout(&tuning)
}

fn convert_to_numbers(top: Vec<&str>, bottom: Vec<&str>) -> (Vec<usize>, Vec<usize>) {
    let mut top_numbers: Vec<usize> = Vec::new();
    let mut bottom_numbers: Vec<usize> = Vec::new();

    let flat_notes = vec!["Bb", "Eb", "Ab", "Db", "Gb", "F"];
    let sharp = ! top.iter().any(|s| flat_notes.contains(s));

    let scale = Scale::new(top.get(0).unwrap(), Some(sharp));

    for note in top.iter() {
	top_numbers.push(
	    scale.notes.iter().position(|x| note == &x).unwrap()
	);
    }
    for note in bottom.iter() {
	bottom_numbers.push(
	    scale.notes.iter().position(|x| note == &x).unwrap()
	);
    }
    (top_numbers, bottom_numbers)
}

fn read_tuning_from_file(filename: &str) -> Tuning {
    let mut filepath = dirs::config_dir().unwrap();
    filepath.push("harptool");
    if ! filepath.is_dir() {
	write_default_layouts();
    }

    filepath.push(filename);

    let contents: Vec<String> = fs::read_to_string(filepath)
	.expect("note layout file not found")
	.lines()
	.map(|s| s.to_string())
	.collect();
    let top: Vec<&str> = contents.get(0).unwrap()
	.split(" ").collect();
    let bottom: Vec<&str> = contents.get(1).unwrap()
	.split(" ").collect();

    let (top, bottom) = convert_to_numbers(top, bottom);

    Tuning::new(top, bottom)
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
    }
}
