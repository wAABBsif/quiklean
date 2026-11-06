use std::{fs, io::Read, path::PathBuf, str::FromStr};
use chrono::{DateTime, TimeZone};

//Should be /home/[USER]/.local/share/Trash
pub fn get_trash_dir() -> Option<PathBuf> {
    let mut path = dirs::data_local_dir()?;
    path.push("Trash");
    if !fs::exists(&path).ok()? {
        return None
    }
    return Some(path);
}
//Should be /home/[USER]/.local/share/Trash/files
pub fn get_trash_files_dir() -> Option<PathBuf> {
    let mut path = get_trash_dir()?;
    path.push("files");
    if !fs::exists(&path).ok()? {
        return None
    }
    return Some(path);
}

//Should be /home/[USER]/.local/share/Trash/info
pub fn get_trash_info_dir() -> Option<PathBuf> {
    let mut path = get_trash_dir()?;
    path.push("info");
    if !fs::exists(&path).ok()? {
        return None
    }
    return Some(path);
}

pub struct TrashEntry {
    pub file: PathBuf,
    pub info: PathBuf,
}

impl TrashEntry {
    pub fn new (file: PathBuf, info: PathBuf) -> TrashEntry {
        TrashEntry {
            file,
            info,
        }
    }

    pub fn read_trash_dir(files: &PathBuf, info: &PathBuf) -> Result<Vec<TrashEntry>,
        Box<dyn std::error::Error>>
    {
        let mut result: Vec<TrashEntry> = Vec::new();

        for path in fs::read_dir(files)? {
            if path.is_err() {
                continue
            }
            let name = path.unwrap().file_name();
            let mut file = files.clone();
            let mut info = info.clone();

            file.push(&name);
            //Trashinfo files have the same name, but with the extension.
            info.push(&name);
            info.add_extension("trashinfo");
            if !fs::exists(&info).unwrap_or(false) {
                continue
            }

            result.push(TrashEntry::new(file, info));
        }

        Ok(result)
    }
}

pub struct TrashInfo {
    pub path: PathBuf,
    pub deletion_date: DateTime<chrono::Local>
}

impl TrashInfo {
    pub fn read_file(path: &PathBuf) -> Result<TrashInfo, Box <dyn std::error::Error>> {
        let mut file = fs::File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let mut lines = contents.lines();
        //Trashinfo files MUST contain this as the first line.
        if lines.next().unwrap_or_default() != "[Trash Info]" {
            Err("Info file does not contain [Trash Info] as first line")?;
        }
        
        //The specification says that the first instance of these values is to be used.
        //Therefore, we only set these if they still don't already have a value.
        let mut path: Option<PathBuf> = None;
        let mut deletion_date: Option<DateTime<chrono::Local>> = None;
        for line in lines {
            let mut entries = line.split('=');
            let key = entries.next().unwrap_or_default();
            let value = entries.next().unwrap_or_default();
            match &key[..] {
                "Path" => {
                    if path.is_some() {
                        continue
                    }
                    let result = TrashInfo::get_path_from_string(value);
                    if result.is_err() {
                        continue
                    }
                    path = Some(result.unwrap());
                }
                "DeletionDate" => {
                    if deletion_date.is_some() {
                        continue
                    }
                    let result = TrashInfo::get_deletion_date_from_string(value);
                    if result.is_err() {
                        continue
                    }
                    deletion_date = Some(result.unwrap());
                }
                _ => continue
            }
        }

        Ok(TrashInfo {
            path: path.ok_or_else(|| "Could not find path")?,
            deletion_date: deletion_date.ok_or_else(|| "Could not find deletion date")?,
        })
    }

    fn get_path_from_string(path: &str) -> Result<PathBuf, Box <dyn std::error::Error>> {
        //We COULD decode the percent-encoding but it's not necessary for functionality
        Ok(PathBuf::from_str(path)?)
    }

    fn get_deletion_date_from_string(deletion_date: &str) -> Result<DateTime<chrono::Local>,
        Box <dyn std::error::Error>>
    {
        //The way chrono sees rfc3339 and the way Linux sees it is seemingly different
        let fixed = chrono::DateTime::parse_from_rfc3339(&(String::from(deletion_date) + "Z"))?;
        let naive = fixed.naive_utc();
        let result = chrono::Local.from_local_datetime(&naive);
        return Ok(result.single().ok_or_else(|| "Could not convert time")?);
    }
}
