use std::path::PathBuf;
use std::{collections::HashMap, io::Read};
use std::fs::File;
use std::io::Write;
use std::error::Error;
use serde::{Serialize, Deserialize};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about)]
struct Args {
    bookmark: Option<String>,
    #[arg(short, long)]
    list: bool,
    #[arg(short, long)]
    remove: bool,
}


#[derive(Serialize, Deserialize, Debug)]
struct Bookmarks {
    marks: HashMap<String, PathBuf>,
}

impl Bookmarks {
    fn new() -> Self {
        Self { marks: HashMap::new() }
    }

    fn from_file(path: PathBuf) -> Result<Self, Box<dyn Error>> {
        let mut file = File::open(path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;

        Ok(serde_json::from_str(&buf)?)
    }

    fn to_file(&self, path: PathBuf) -> Result<(), Box<dyn Error>> {
        let file = File::create(path)?;
        let buf = serde_json::to_string(self)?;
        
        write!(&file, "{}", buf)?;

        Ok(())
    }

    fn get(&mut self, key: String, value: PathBuf) -> Option<PathBuf> {
        let val = match self.marks.get(&key) {
            Some(val) => val,
            None => {
                self.marks.insert(key, value.clone());
                &value
            },
        };

        Some(val.to_path_buf())
    }

    fn remove(&mut self, key: String) {
        self.marks.remove(&key);
    }

    fn list(&self) -> String {
        let mut res = String::new();

        for (key, value) in self.marks.iter() {
            res.push_str(&format!("{}: {:?}\n", key, value));
        }

        res
    }
}

fn main() {
    let path = PathBuf::from("/home/dennis/.cache/bookmark/bookmarks.json");
    let args = Args::parse();

    let mut bm = match Bookmarks::from_file(path.clone()) {
        Ok(val) => val,
        Err(_) => Bookmarks::new(),
    };

    if args.list {
        eprint!("{}", bm.list());

    } else if args.remove {
        bm.remove(args.bookmark.unwrap());

    } else {
        let value = bm.get(args.bookmark.unwrap(), std::env::current_dir().unwrap());
        bm.to_file(path).unwrap();
        print!("{}", value.unwrap().to_str().unwrap());
    }
}
