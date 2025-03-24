use std::{fs, io::{self, Write}, path::Path, time::SystemTime, sync::Mutex};
use::chrono::{DateTime, Utc};
use walkdir::WalkDir;
use super::file_type::{get_file_type, FileType};
use rayon::prelude::*;

const EXECUTABLES: &str = "Executables";
const SUBTITLES: &str = "Subtitles";
const ZIPFILES: &str = "Zipfiles";

fn move_to_subfolder(path: &Path, subfolder_name: &str) -> io::Result<()> {
    let parent = path.parent().expect("File does not have a parent directory");
    
    let target_dir = parent.join(subfolder_name);


    if !target_dir.exists() {
        fs::create_dir_all(&target_dir)?;
    }

    let filename = path.file_name().expect("Cannot get a name");

    let target_path = target_dir.join(filename);

    fs::rename(path, &target_path)?;


    Ok(()) 
}

fn exclude_contains(path: &Path, excluded_extensions: &Vec<String>, extension_bool: &bool) -> bool{

    if *extension_bool {
        excluded_extensions.iter().any(|excluded| { 
            path.extension()
                .and_then(|ext| ext.to_str()) 
                .map_or(false, |ext| ext == *excluded) 
        })
    } else{
        excluded_extensions.iter().any(|name| {
            let name_lower = name.to_ascii_lowercase();
            path.file_name()
                .expect("cannot read the file name")
                .to_string_lossy()
                .to_lowercase()
                .contains(&name_lower)
        })
    } 

}

pub fn organise_dir(file_path: &str, excluded_extensions: &Vec<String>) -> io::Result<()> {
    for entry in WalkDir::new(file_path)
        .into_iter()
        .filter_entry(|e| {
            !e.path().ancestors().any(|ancestor| {
                ancestor.file_name().map_or(false, |name| name == EXECUTABLES || name == SUBTITLES || name == ZIPFILES)
            })
        })
        .filter_map(Result::ok)
         {
            let path = entry.path();

            if exclude_contains(path, &excluded_extensions, &true) {
                continue;
            }

            match get_file_type(path) {
                FileType::Executables => move_to_subfolder(path, EXECUTABLES)?,
                FileType::Subtitles => move_to_subfolder(path, SUBTITLES)?,
                FileType::ZipFiles => move_to_subfolder(path, ZIPFILES)?,
                FileType::Other => {},
            }
        }
    Ok(())

}

pub fn sort_by_date(file_path: &str, excluded_extensions: &Vec<String>) -> io::Result<()> {
    let mut files_with_dates: Vec<(String, SystemTime)> = Vec::new();
    let skibidi = false;

    for entry in WalkDir::new(file_path).min_depth(1).max_depth(1) {
        let entry = entry?;
        let path = entry.path();

        if exclude_contains(path, &excluded_extensions, &skibidi) {continue}; 

        if let Ok(metadata) = fs::metadata(path) {
            if let Ok(modified_time) = metadata.modified() {
                files_with_dates.push((path.display().to_string(), modified_time));
            }
        }
    }

    files_with_dates.sort_by_key(|&(_, modified_time)| std::cmp::Reverse(modified_time));

    for (path, time) in &files_with_dates {
        let datetime: DateTime<Utc> = (*time).into();
    
        println!("{} modified at {}", path, datetime.format("%Y-%m-%d %H:%M:%S"));
    }


    Ok(())


}

pub fn type_sort(dir: &str, types: &Vec<String>) -> io::Result<()> {
    let stdout = io::stdout();
    let handle = Mutex::new(io::BufWriter::new(stdout));
    
    WalkDir::new(dir)
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .par_bridge()
            .filter_map(|entry| {
                entry.ok().filter(|e| !exclude_contains(e.path(), types, &false))
            }) 
            .for_each(|entry| {
                let mut guard = handle.lock().unwrap();
                writeln!(guard, "{}", entry.path().display()).expect("failed to write to stdout");
            });
    handle.lock().unwrap().flush()?;

    Ok(())
}

pub fn print_dir(file_path: &str, excluded_extensions: &Vec<String>) -> io::Result<()>{
    let stdout = io::stdout();
    let handle = Mutex::new(io::BufWriter::new(stdout));
     WalkDir::new(file_path)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .par_bridge()
        .filter_map(|entry| {
            entry.ok().filter(|e| !exclude_contains(e.path(), excluded_extensions, &false))
        })
        .for_each(|entry| {
            let mut guard = handle.lock().unwrap();
            writeln!(guard, "{}", entry.path().display()).expect("failed to write to stdout");
        });

    handle.lock().unwrap().flush()?;
        

    Ok(())  
}

pub fn move_file(path: &Vec<String>) {
    let file_path = Path::new(&path[0]);
    let file_name = file_path.file_name().expect("cannot read the file name");
    let move_path = Path::new(&path[1]);

    if !move_path.exists() {
        fs::create_dir_all(move_path).expect("Cannot create directory");
    }

    let target_dir = move_path.join(file_name);

    fs::rename(file_path, target_dir).expect("Cannot move file");
}

pub fn delete_paths(path_vec: &Vec<String>){
    for path_str in path_vec {
        let path = Path::new(path_str);

        if !path.exists() {
            println!("The path you provided does not exist");
        } 
    
        if path.is_file(){ 
            fs::remove_file(path).expect("Could not delete the file");
        } else if path.is_dir() {
            fs::remove_dir(path).expect("Could not delete the folder");
        }

    }

}

