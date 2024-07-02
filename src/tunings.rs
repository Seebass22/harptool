use std::collections::HashMap;

#[rustfmt::skip]
pub fn get_tunings() -> HashMap<&'static str, &'static str> {
    HashMap::<&str, &str>::from([
        ("richter", "C E G C E G C E G C\nD G B D F A B D F A\n"),
        ("country", "C E G C E G C E G C\nD G B D F# A B D F A\n"),
        ("wilde tuning", "C E G C E E G C E A\nD G B D F G B D G C\n"),
        ("wilde minor tuning", "C Eb G C Eb Eb G C Eb A\nD G Bb D F G Bb D G C\n"),
        ("melody maker", "C E A C E G C E G C\nD G B D F# A B D F# A\n"),
        ("natural minor", "C Eb G C Eb G C Eb G C\nD G Bb D F A Bb D F A\n"),
        ("harmonic minor", "C Eb G C Eb G C Eb G C\nD G B D F Ab B D F Ab\n"),
        ("paddy richter", "C E A C E G C E G C\nD G B D F A B D F A\n"),
        ("pentaharp", "A D E A D E A D E A\nC Eb G C Eb G C Eb G C"),
        ("powerdraw", "C E G C E G A C E A\nD G B D F A B D G C"),
        ("powerbender", "C E G C D F A C E A\nD G B D E G B D G C"),
        ("diminished", "C Eb Gb A C Eb Gb A C Eb\nD F Ab B D F Ab B D F"),
        ("spiral", "C E G B D F A C E G\nD F A C E G B D F A"),
        ("lucky 13 diminished", "A C Eb Gb A C Eb Gb A C Eb Gb A\nB D F Ab B D F Ab B D F Ab B"),
        ("lucky 13 powerchromatic", "C D F A C D F A C D F A C\nD E G B D E G B D E G B D"),
        ("easy 3rd", "C E G C E G C E G C\nD F A D F A B D F A"),
        ("4 hole richter", "C E G C\nD F A B"),
        ("5 hole richter", "C E G C E\nD F A B D"),
    ])
}
