use std::env;

pub struct Config { 
    dir_path: String,
    sort_date: bool,
    organise: bool,
    display: bool,
    delete_paths: Vec<String>,
    excluded_extensions: Vec<String>,
    types: Vec<String>,
    move_paths: Vec<String>,
}

impl Config {
    pub fn dir_path(&self) -> &String {
        &self.dir_path
    }

    pub fn sort_date(&self) -> bool {
        self.sort_date
    }

    pub fn organise(&self) -> bool {
        self.organise
    }

    pub fn display(&self) -> bool {
        self.display
    }
    // delete_path func
    pub fn get_delete_paths(&self) -> &Vec<String> {
        &self.delete_paths
    }

    // excluded_extensions func
    pub fn get_excluded_extensions(&self) -> &Vec<String> {
        &self.excluded_extensions
    }
    
    // types func
    pub fn get_types(&self) -> &Vec<String> {
        &self.types
    }

    // move_paths func
    pub fn get_move_paths(&self) -> &Vec<String> {
        &self.move_paths
    }




    
}

pub fn get_args() -> Config {

    let matches = super::args::parse_arguments();

    Config {
        dir_path: {
            if matches.contains_id("Path"){
                matches.get_one::<String>("Path").expect("Path does not exist").to_string()
            } else {
                env::current_dir().expect("Could not get current directory").to_string_lossy().to_string()   
            }
        },
        sort_date: matches.get_flag("Date"),
        organise: matches.get_flag("Organise"),
        display: matches.get_flag("Display"),
        delete_paths: {
            matches
                .get_many::<String>("Del")
                .unwrap_or_default()
                .cloned()
                .collect()
        },
        excluded_extensions: {
            matches
                .get_many::<String>("Exclude")
                .unwrap_or_default()
                .cloned()
                .collect()
        },
        types: {
            matches
                .get_many::<String>("Type")
                .unwrap_or_default()
                .cloned()
                .collect()
        },
        move_paths: {
            matches
                .get_many::<String>("Move")
                .unwrap_or_default()
                .cloned()
                .collect()
        },
    }

}