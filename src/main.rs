use std::fs::read_dir;

fn main() {
    let directory = std::env::args().nth(1).expect("Expect Pattern");
    let l = directory.len();

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
