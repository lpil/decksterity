extern crate claxon;

pub fn read_flac(path: String) {
    let mut reader = claxon::FlacReader::open(path).unwrap();

    let some_samples: Vec<_> = reader
        .samples()
        .map(|s| s.unwrap() as f64)
        .skip(500)
        .take(50)
        .collect();

    println!("some_samples {:?}", some_samples);
}
