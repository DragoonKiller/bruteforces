
use std::io::*;
use std::fs::*;

/// Create directory.
/// Supported syntax:
/// (1) dir1/dir2/dir3/
/// (2) /dir1/dir2/dir3/
/// (3) /dir1/dir2/dir3
/// (4) ./dir1/./dir2/dir3/
pub fn DigDir(path : &str)
{
    let s : Vec<&str> = path.split('/').filter(|&x| x != "." && x != "").collect();
    s.iter().fold(String::new(), |x, &y|
    {
        let g = x + y + "/";
        create_dir(g.as_str()).ok();
        g
    });
}

/// Create file.
/// Supported syntax:
/// (1) [dir]/filename
pub fn CreateFile(filepath : &str) -> Option<File>
{
    let mut s : Vec<&str> = filepath.split('/').filter(|&x| x != "." && x != "").collect();
    if let Some(filename) = s.pop()
    {
        let path = s.join("/");
        DigDir(&path);
        return Some(OpenOptions::new().create(true).write(true).truncate(true).open(filepath).ok().unwrap());
    }
    None
}

/// Read file.
/// Supported syntax:
/// (1) [dir]/filename
pub fn ReadFile(filepath : &str) -> Option<File>
{
    File::open(filepath).ok()
}

// write all of a string to a file.
pub fn WriteAll(filepath : &str, src : &str) -> std::result::Result<(), String>
{
    match CreateFile(filepath)
    {
        Some(mut file) =>
        {
            file.write_all(src.as_bytes());
            Ok(())
        }
        None => { Err("".to_string()) }
    }
}

/// Read file to the end.
pub fn ReadAll(filepath : &str) -> Option<String>
{
    match ReadFile(filepath)
    {
        Some(mut file) =>
        {
            let mut s = String::new();
            file.read_to_string(&mut s);
            Some(s)
        }
        _ => return None
    }
}

/// Get directory a file located.
pub fn Dir(filepath : &str) -> Option<String>
{
    let mut s : Vec<String> = filepath.split('/').map(|x| x.to_string()).collect();
    if s.len() == 0 { return None; }
    s.pop();
    Some(format!("./{}", s.join("/")))
}