mod file_operation;
mod args_parse;
pub mod args;
pub mod file_type;


use std::{io, time::Instant};
use file_operation::{delete_paths, move_file, organise_dir, print_dir, sort_by_date, type_sort};
use args_parse::get_args;


fn main() -> io::Result<()>{
    let star = Instant::now();

    let config = get_args();
    let dir_path = config.dir_path();
    let excluded_extensions = config.get_excluded_extensions();
    let move_paths = config.get_move_paths();
    let types = config.get_types();
    let del_paths = config.get_delete_paths();


    if config.organise() {
        organise_dir(dir_path, excluded_extensions)?;
    }
    if config.display(){
        print_dir(dir_path, excluded_extensions)?;
    }
    
    if config.sort_date() {
        sort_by_date(dir_path, &config.get_excluded_extensions())?;
    }

    if !move_paths.is_empty() {
        move_file(move_paths);
    }
    
    if !types.is_empty(){
        type_sort(dir_path, &types)?;
    }


    if !del_paths.is_empty() {
        delete_paths(del_paths);
    }
    let duration = star.elapsed();
    println!("it took: {} ms", duration.as_millis());     
    Ok(())   

}
