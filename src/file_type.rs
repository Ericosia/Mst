use std::path::Path;

pub enum FileType{
    Executables, 
    Subtitles,
    ZipFiles,
    Other,  
}

pub fn get_file_type(path: &Path) -> FileType{
    if path.extension().map_or(false, |ext| ext == "exe" || ext == "msi"){
        FileType::Executables
    } else if path.extension().map_or(false, |ext| ext == "ass" || ext == "srt"){
        FileType::Subtitles
    } else if path.extension().map_or(false, |ext| ext == "zip"){
        FileType::ZipFiles
    } else {
        FileType::Other
    }
}