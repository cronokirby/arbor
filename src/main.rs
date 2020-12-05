use std::fs;

fn main() -> std::io::Result<()> {
    let mut files: Vec<String> = Vec::new();
    for entry in fs::read_dir(".")? {
        let dir = entry?;
        files.push(dir.file_name().as_os_str().to_string_lossy().into_owned());
    }
    println!("{:?}", files);
    Ok(())
}
