use std::fs;
use std::path::Path;

/// Represents a tree of files
///
/// A single instance of a Tree is either a leaf, being a file node, with just that
/// component of the path, or it's a directory, with a name, and a vector of children,
/// also being complete trees.
#[derive(Debug)]
enum Tree {
    File { name: String },
    Dir { name: String, children: Vec<Tree> },
}

fn read_dir<P: AsRef<Path>>(path: P, buf: &mut Vec<Tree>) -> std::io::Result<()> {
    for entry in fs::read_dir(path)? {
        let file = entry?;
        let name = file.file_name().as_os_str().to_string_lossy().into_owned();
        if file.file_type()?.is_dir() {
            let mut children = Vec::new();
            read_dir(file.path(), &mut children)?;
            buf.push(Tree::Dir { name, children });
        } else {
            buf.push(Tree::File { name });
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut children: Vec<Tree> = Vec::new();
    read_dir(".", &mut children)?;
    let tree = Tree::Dir { name: ".".into(), children };
    println!("{:?}", tree);
    Ok(())
}
