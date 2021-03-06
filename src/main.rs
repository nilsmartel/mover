use std::fs::read_dir;

fn main() {
    let directory = if let Some(dir) = std::env::args().nth(1) {
        dir
    } else {
        eprintln!("Expected Argument");
        std::process::exit(0);
    };

    let files: Vec<std::fs::DirEntry> = read_dir(".")
        .into_iter()
        .next()
        .unwrap()
        .into_iter()
        .filter(|entry| {
            if let Ok(entry) = entry {
                let entry: &std::fs::DirEntry = entry;

                return entry.file_name().to_str().unwrap().starts_with(&directory);
            }
            false
        })
        .map(|entry| entry.unwrap())
        .collect();

    std::fs::create_dir(&directory).expect("failed to create directory");

    let l = directory.len();
    files.into_iter().for_each(|file: std::fs::DirEntry| {
        std::fs::rename(
            file.file_name(),
            &format!(
                "{}/{}",
                &directory,
                &file.file_name().to_str().unwrap()[l..].trim()
            ),
        );
    })
}
