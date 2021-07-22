fn printnotes(indices: &Vec<Option<usize>>) {
    //                0     1    2    3     4    5     6     7    8    9    10   11
    let notes = vec!["C", "Dd", "D", "Eb", "E", "F", "F#", "G", "Ab", "A", "Bb", "B"];

    for i in indices {
	// println!("{:?}", i);
	let n = match *i {
	    Some(n) => notes.get(n).unwrap().to_string(),
	    None => String::from(" "),
	};
	// print!("{} ", n);
	print!("{:width$} ", n, width = 3);
    }
    print!("\n");
}

pub fn test() {
    let top = vec![Some(0), Some(4), Some(7), Some(0), Some(4), Some(7), Some(0), Some(4), Some(7), Some(0)];
    let bottom = vec![Some(2), Some(7), Some(11), Some(2), Some(5), Some(9), Some(11), Some(2), Some(5), Some(9)];
    let bends_half = vec![Some(1), Some(6), Some(10), None, Some(8), None, None, None, None];
    let bends_full = vec![None, Some(5), Some(9), None, None, None, None, None, None, None];
    let bends_one_and_half = vec![None, None, Some(8), None, None, None, None, None, None, None];
    let blow_bends_half = vec![None, None, None, None, None, None, None, Some(3), Some(6), Some(11)];
    let blow_bends_full = vec![None, None, None, None, None, None, None, None, None, Some(10)];

    printnotes(&blow_bends_full);
    printnotes(&blow_bends_half);
    printnotes(&top);
    printnotes(&bottom);
    printnotes(&bends_half);
    printnotes(&bends_full);
    printnotes(&bends_one_and_half);
}
