use std::fs;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "arbor")]
struct Args {
    /// The path where the tree should begin
    #[structopt(default_value = ".")]
    path: PathBuf,
    /// If true, then hidden files will be included in the output
    #[structopt(short = "a", long = "all")]
    all: bool,
}

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

impl Tree {
    fn name(&self) -> &str {
        match self {
            Tree::File { name } => &name,
            Tree::Dir { name, .. } => &name,
        }
    }
}

fn read_dir_rec<P: AsRef<Path>>(path: P, all: bool, buf: &mut Vec<Tree>) -> std::io::Result<()> {
    for entry in fs::read_dir(path)? {
        let file = entry?;
        let name = file.file_name().as_os_str().to_string_lossy().into_owned();
        if !all && name.starts_with(".") {
            continue;
        }
        if file.file_type()?.is_dir() {
            let mut children = Vec::new();
            read_dir_rec(file.path(), all, &mut children)?;
            buf.push(Tree::Dir { name, children });
        } else {
            buf.push(Tree::File { name });
        }
    }
    Ok(())
}

fn read_dir(args: &Args, buf: &mut Vec<Tree>) -> std::io::Result<()> {
    read_dir_rec(&args.path, args.all, buf)
}

#[derive(Debug)]
enum Padding {
    Blank,
    Bar,
}

fn print_tree(tree: &Tree) {
    use Padding::*;

    fn rec(tree: &Tree, prev: &mut Vec<Padding>, last: bool) {
        if !prev.is_empty() {
            for i in 0..(prev.len() - 1) {
                match prev[i] {
                    Blank => print!("    "),
                    Bar => print!("│   "),
                }
            }
            if last {
                print!("└───");
            } else {
                print!("├───");
            }
        }

        println!("{}", tree.name());
        match tree {
            Tree::File { .. } => {}
            Tree::Dir { children, .. } => {
                let len = children.len();
                for (i, child) in children.iter().enumerate() {
                    let next_last = i == len - 1;
                    prev.push(if next_last { Blank } else { Bar });
                    rec(child, prev, next_last);
                    prev.pop();
                }
            }
        }
    }

    let mut prev = Vec::new();
    rec(tree, &mut prev, true);
}

fn main() -> std::io::Result<()> {
    let args = Args::from_args();
    let mut children: Vec<Tree> = Vec::new();
    read_dir(&args, &mut children)?;
    let tree = Tree::Dir {
        name: ".".into(),
        children,
    };
    print_tree(&tree);
    Ok(())
}
