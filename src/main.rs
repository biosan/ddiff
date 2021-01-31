use humansize::{file_size_opts as options, FileSize};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::time::Instant;
use structopt::StructOpt;
use tabwriter::TabWriter;
use walkdir::{DirEntry, WalkDir};

extern crate humansize;

const CHUNK_SIZE: usize = 16 * 1024;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path_a: std::path::PathBuf,
    #[structopt(parse(from_os_str))]
    path_b: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();

    // Start a timer to measure performance
    let timer = Instant::now();

    // Hash every file in A and B directories
    let (path_to_hash_a, size_a) = hash_files(&args.path_a);
    let (path_to_hash_b, size_b) = hash_files(&args.path_b);

    let files_a: HashSet<&PathBuf> = path_to_hash_a.keys().collect();
    let files_b: HashSet<&PathBuf> = path_to_hash_b.keys().collect();

    // Get all files with the same path and different hash (return a list of hash and paths)
    let different_hash_files: Vec<(&PathBuf, &String, &String)> = files_a
        .intersection(&files_b)
        .map(|path| {
            (
                *path,
                path_to_hash_a.get(*path).unwrap(),
                path_to_hash_b.get(*path).unwrap(),
            )
        })
        .filter(|(_, hash_a, hash_b)| hash_a != hash_b)
        .collect();

    let diff_a_b: Vec<(&PathBuf, &String)> = files_a
        .difference(&files_b)
        .map(|path| (*path, path_to_hash_a.get(*path).unwrap()))
        .collect();
    let diff_b_a: Vec<(&PathBuf, &String)> = files_b
        .difference(&files_a)
        .map(|path| (*path, path_to_hash_b.get(*path).unwrap()))
        .collect();

    // TODO: Compute a list of files in diffs with the same hash

    //
    // Let's print some stuff out!
    //

    // Show if there are files with the same path but different hash
    if different_hash_files.len() > 0 {
        print_different_hash(&different_hash_files);
    }

    // Show if there are files in A but not in B
    if diff_a_b.len() > 0 {
        print_diff(&args.path_a, &args.path_b, &diff_a_b);
    }

    // Show if there are files in B but not in A
    if diff_b_a.len() > 0 {
        print_diff(&args.path_b, &args.path_a, &diff_b_a);
    }

    // Print some info on ddiff performance
    println!(
        "\nddiff checked {} files, about {}, in {:?}\n",
        files_a.len() + files_b.len(),
        (size_a + size_b).file_size(options::BINARY).unwrap(),
        timer.elapsed(),
    );

    if different_hash_files.len() + diff_a_b.len() + diff_b_a.len() == 0 {
        println!(
            "Great! {} and {} are equal!",
            args.path_a.to_str().unwrap(),
            args.path_b.to_str().unwrap(),
        );
    }
}

fn hash_files(directory: &Path) -> (HashMap<PathBuf, String>, usize) {
    let dir = directory.canonicalize().unwrap();
    // Get all files and directories inside `dir`
    let files = WalkDir::new(&dir)
        .into_iter()
        // Get only "openable" files
        .filter_map(Result::ok)
        // Get exclude directories
        .filter(|e: &DirEntry| !e.file_type().is_dir())
        // Transform `DirEntry` into a `PathBuf`
        .map(|e: DirEntry| e.into_path())
        .collect::<Vec<PathBuf>>();
    let pb = ProgressBar::new(files.len() as u64);
    pb.set_style(ProgressStyle::default_bar().template(
        "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
    ));
    let hashed_files: HashMap<PathBuf, (String, usize)> = files
        .par_iter()
        .progress_with(pb)
        // Compute file hash and build up a pair to create the output hashmap
        .map(|path: &PathBuf| (hash_file(&path).unwrap(), path))
        // I'm stupid... I understand why I can't do this directly in the line above,
        //   but I didn't find a way around it
        // NOTE: Also strip base directory prefix from path
        .map(|(hash, path)| (path.strip_prefix(&dir).unwrap().to_path_buf(), hash))
        .collect();
    let total_size = hashed_files.iter().map(|(_, (_, bytes))| bytes).sum();
    let hashes = hashed_files
        .into_iter()
        .map(|(path, (hash, _))| (path, hash))
        .collect();
    (hashes, total_size)
}

fn hash_file(path: &std::path::Path) -> std::io::Result<(String, usize)> {
    let mut hash = blake3::Hasher::new();
    let mut buffer: [u8; CHUNK_SIZE] = [0; CHUNK_SIZE];
    let mut file = std::fs::File::open(path)?;
    let mut bytes = 0;
    // TODO: Implement chunked reading using an iterator some day!
    loop {
        let bytes_read = file.read(&mut buffer).unwrap_or(0);
        bytes += bytes_read;
        hash.update(&buffer[0..bytes_read]);
        if bytes_read == 0 {
            break;
        }
    }
    Ok((hash.finalize().to_hex().to_ascii_lowercase(), bytes))
}

// Files with same path and different hash
fn print_different_hash(files: &Vec<(&PathBuf, &String, &String)>) {
    let mut tw = TabWriter::new(vec![]);
    tw.write_all(b"\n\n>>> Files with same path but with different hash\n\n")
        .unwrap();
    tw.write(b"\tpath\thash A\thash B\n").unwrap();
    for (path, hash_a, hash_b) in files {
        tw.write(format!("\t{}\t{}\t{}\n", path.to_str().unwrap(), hash_a, hash_b,).as_bytes())
            .unwrap();
    }
    tw.flush().unwrap();
    let output = String::from_utf8(tw.into_inner().unwrap()).unwrap();
    print!("{}", output);
}

// Files in A but not in B
fn print_diff(path_a: &PathBuf, path_b: &PathBuf, path_hash: &Vec<(&PathBuf, &String)>) {
    let mut tw = TabWriter::new(vec![]);
    tw.write_all(
        format!(
            "\n\n>>> Files in {} but not in {}\n\n",
            path_a.to_str().unwrap(),
            path_b.to_str().unwrap()
        )
        .as_bytes(),
    )
    .unwrap();
    tw.write(
        format!(
            "\tpath\thash {}\thash {}\n",
            path_a.to_str().unwrap(),
            path_b.to_str().unwrap()
        )
        .as_bytes(),
    )
    .unwrap();
    for (path, hash) in path_hash {
        tw.write_all(format!("\t{}\t{}\n", path.to_str().unwrap(), hash,).as_bytes())
            .unwrap();
    }
    tw.flush().unwrap();
    let output = String::from_utf8(tw.into_inner().unwrap()).unwrap();
    print!("{}", output);
}