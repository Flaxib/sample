// You should change this.
//
// Depending on your implementation, there are a variety of potential errors
// which might occur. They aren't checked by the test suite in order to
// allow the greatest freedom of implementation, but real libraries should
// provide useful, descriptive errors so that downstream code can react
// appropriately.
//
// One common idiom is to define an Error enum which wraps all potential
// errors. Another common idiom is to use a helper type such as failure::Error
// which does more or less the same thing but automatically.
#[derive(Debug)]
pub enum Error {
    UnknownTonic,
    IntervalsToLarge,
}

pub struct Scale {
    scale: Vec<String>,
}

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        let chromatic = Scale::chromatic(tonic)?;

        let get_increment = |c| match c {
            'M' => 2,
            'm' => 1,
            'A' => 3,
            _ => todo!(),
        } as usize;
        let intervals_len = intervals.chars().map(get_increment).map(|i| i as i32).sum();

        if 12 < intervals_len {
            return Err(Error::IntervalsToLarge);
        }

        let mut i = 0;
        let mut scale = vec![chromatic.scale[i].clone()];
        for inter in intervals.chars() {
            i += get_increment(inter);
            scale.push(chromatic.scale[i].clone());
        }
        Ok(Scale { scale })
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        let tonic_with_sharp = vec![
            "C", "a", "G", "D", "A", "E", "B", "F#", "e", "b", "f#", "c#", "g#", "d#",
        ];
        let tonic_with_flat = vec![
            "F", "Bb", "Eb", "Ab", "Db", "Gb", "d", "g", "c", "f", "bb", "eb",
        ];

        let all_tonics = tonic_with_sharp
            .iter()
            .cloned()
            .chain(tonic_with_flat.iter().cloned())
            .collect::<Vec<&str>>();
        if !all_tonics.contains(&tonic) {
            return Err(Error::UnknownTonic);
        }

        let scale_with_sharp = vec![
            "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
        ];
        let scale_with_flat = vec![
            "F", "Gb", "G", "Ab", "A", "Bb", "B", "C", "Db", "D", "Eb", "E",
        ];

        let use_sharp = tonic_with_sharp.contains(&tonic);

        let capitalized_first_letter = tonic
            .chars()
            .next()
            .unwrap()
            .to_ascii_uppercase()
            .to_string();
        let remaining = tonic.chars().skip(1).collect::<String>();
        let capital_tonic = capitalized_first_letter + &remaining[..];

        let scale_to_use = match use_sharp {
            true => scale_with_sharp,
            false => scale_with_flat,
        };

        let start_index = scale_to_use
            .iter()
            .position(|&note| note == capital_tonic)
            .unwrap();

        let mut scale = vec![];
        let scale_len = scale_to_use.len();
        for i in 0..(scale_len + 1) {
            scale.push(scale_to_use[(start_index + i) % scale_len].to_string());
        }

        Ok(Scale { scale })
    }

    pub fn enumerate(&self) -> Vec<String> {
        self.scale.clone()
    }
}
