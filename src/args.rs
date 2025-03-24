use clap::{ command, Arg, ArgAction, ArgMatches};

pub fn parse_arguments() -> ArgMatches {
    command!().about("Simple programe to organise and sort files")
    .arg(
        Arg::new("Path")
            .short('p')
            .help("Takes a path. If not provided will use current dir")
    )
    .arg(
        Arg::new("Organise")
            .short('o')
            .help("Organise the given path")
            .action(ArgAction::SetTrue)
    )
    .arg(
        Arg::new("Type")
            .short('t')
            .help("Show files only with this type")
            .num_args(1..)
    )
    .arg(
        Arg::new("Date")
            .short('d')
            .help("Sort files/folders by date")
            .action(ArgAction::SetTrue)
            .conflicts_with_all(["Display", "Type"])
    )
    .arg(
        Arg::new("Exclude")
            .short('e')
            .num_args(1..)
            .help("File extensions to exclude from organising")
            .conflicts_with("Type")   
    )
    .arg(
        Arg::new("Display")
            .long("ls")
            .help("Prints directory")   
            .action(ArgAction::SetTrue)
    )
    .arg(
        Arg::new("Move")
            .short('m')
            .help("Moves files. Takes 2 args: path of the file, path you want to move the file in ")
            .num_args(2)
            .conflicts_with_all(["Display", "Type", "Exclude", "Organise", "Path"])
    )
    .arg(
        Arg::new("Del")
            .long("del")
            .help("takes a name or path as an argument and deletes it. if not provided it would get it from current directory")
            .num_args(1..)
    )
    .get_matches()

}