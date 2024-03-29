use ignore::WalkBuilder;
use std::fs;
use std::io::Write;
use std::path::Path;
use structopt::StructOpt;
use regex::Regex;

#[derive(StructOpt, Debug)]
/// Batch replace text file.
struct Replace {
    #[structopt(short, long)]
    /// File path to replace
    input: String,

    #[structopt(short, long, default_value = "*")]
    /// File extensions use * to accept all ext
    ext: String,

    #[structopt(short, long)]
    /// Original string
    from: String,

    #[structopt(short, long)]
    /// Replaced string
    to: String,
    
    #[structopt(short, long)]
    /// Read hidden file
    hidden: bool,
}

fn main() {
    let opt = Replace::from_args();
    let path = Path::new(&opt.input);
    if path.is_dir() {
        for dir in WalkBuilder::new(path)
            .standard_filters(!opt.hidden).build()
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
        {
            if &opt.ext == "*"
                || dir
                    .path()
                    .extension()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default()
                    == &opt.ext
            {
                perform_replace(dir.path(), &opt.from, &opt.to);
            }
        }
    } else if opt.ext == "*"
        || path
            .extension()
            .unwrap_or_default()
            .to_string_lossy()
            .contains(&opt.ext)
    {
        perform_replace(path, &opt.from, &opt.to);
    }
}

fn perform_replace(path: &Path, from: &str, to: &str) {
    println!("Replacing file {}", path.display());
    let re = Regex::new(from).expect("Failed to compile regex");
    let source = fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Can not read file {}", &path.display()));
    let output = re.replace_all(&source, to).into_owned();
    let mut f = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&path)
        .expect("Failed to rewrite file");
    f.write_all(output.as_bytes()).unwrap();
    f.flush().unwrap();
}
