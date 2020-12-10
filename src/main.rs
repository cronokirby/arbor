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
    /// The maximum depth to recurse at. 0 will just print the tree root.
    #[structopt(short = "d", long = "depth")]
    depth: Option<u32>,
    /// If true, don't use unicode characters for tree branchess
    #[structopt(long = "ascii")]
    ascii: bool
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

fn read_dir_rec<P: AsRef<Path>>(path: P, depth: u32, args: &Args, buf: &mut Vec<Tree>) -> std::io::Result<()> {
    for entry in fs::read_dir(path)? {
        let file = entry?;
        let name = file.file_name().as_os_str().to_string_lossy().into_owned();
        if !args.all && name.starts_with(".") {
            continue;
        }

        if file.file_type()?.is_dir() {
            let mut children = Vec::new();
            let next_depth = depth + 1;
            if args.depth.map(|max| next_depth <= max).unwrap_or(true) {
                read_dir_rec(file.path(), next_depth, args, &mut children)?;
            }
            buf.push(Tree::Dir { name, children });
        } else {
            buf.push(Tree::File { name });
        }
    }
    Ok(())
}

fn read_dir(args: &Args, buf: &mut Vec<Tree>) -> std::io::Result<()> {
    if let Some(0) = args.depth {
        return Ok(());
    }
    read_dir_rec(&args.path, 1, args, buf)
}

/// A printer allows us to print some parts of the tree using different settings
struct Printer {
    /// Whether or not we should print things using only ASCII characters
    ascii: bool
}

impl Printer {
    fn new(ascii: bool) -> Self {
        Printer { ascii }
    }

    fn print_node(&self, node: &Tree) {
        println!("{}", node.name());
    }

    fn print_last_connection(&self) {
        if self.ascii {
            print!("\\---");
        } else {
            print!("└───");
        }
    }

    fn print_connection(&self) {
        if self.ascii {
            print!("|---");
        } else {
            print!("├───");
        }
    }

    fn print_bar(&self) {
        if self.ascii {
            print!("|   ");
        } else {
            print!("│   ");
        }
    }

    fn print_blank(&self) {
        print!("    ");
    }
}

#[derive(Debug)]
enum Padding {
    Blank,
    Bar,
}

fn print_tree(tree: &Tree, printer: &Printer) {
    use Padding::*;

    fn rec(printer: &Printer, tree: &Tree, prev: &mut Vec<Padding>, last: bool) {
        if !prev.is_empty() {
            for i in 0..(prev.len() - 1) {
                match prev[i] {
                    Blank => printer.print_blank(),
                    Bar => printer.print_bar(),
                }
            }
            if last {
                printer.print_last_connection();
            } else {
                printer.print_connection();
            }
        }

        printer.print_node(tree);
        match tree {
            Tree::File { .. } => {}
            Tree::Dir { children, .. } => {
                let len = children.len();
                for (i, child) in children.iter().enumerate() {
                    let next_last = i == len - 1;
                    prev.push(if next_last { Blank } else { Bar });
                    rec(printer, child, prev, next_last);
                    prev.pop();
                }
            }
        }
    }

    let mut prev = Vec::new();
    rec(printer, tree, &mut prev, true);
}

fn main() -> std::io::Result<()> {
    let args = Args::from_args();
    let mut children: Vec<Tree> = Vec::new();
    read_dir(&args, &mut children)?;
    let tree = Tree::Dir {
        name: ".".into(),
        children,
    };
    let printer = Printer::new(args.ascii);
    print_tree(&tree, &printer);
    Ok(())
}
