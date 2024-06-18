use std::path::Path;
use std::collections::{HashMap, HashSet};
use libc::{statvfs, S_IFLNK, S_IFMT, S_IFDIR, stat, closedir, opendir, readdir};
use std::ffi::{CString, CStr};
use std::mem;
use rayon::prelude::*;
use std::sync::{Mutex, Arc};

const BYTES: f64 = 1_073_741_824.0;
const BATCH_SIZE: usize = 1024;

pub fn disk() -> (HashMap<&'static str, f64>, f64, f64){
    let dir_paths = vec!["/usr", "/var", "/home", "/boot"];
    
    let total_size = Arc::new(Mutex::new(0));
    let total_free = get_free_space("/").unwrap_or(0);
    let disk_space = Arc::new(Mutex::new(HashMap::new()));
    
    dir_paths.par_iter().for_each(|&path| {
        let thread_total_size = Arc::clone(&total_size);
        let thread_disk_space = Arc::clone(&disk_space);

        match calculate_dir_size(path) {
            Ok(size) => {
                let gb = size as f64 / BYTES;
                {
                    let mut total_size = thread_total_size.lock().unwrap();
                    *total_size += size;
                }
                {
                    let mut disk_space = thread_disk_space.lock().unwrap();
                    disk_space.insert(path, gb);
                }
                println!("{}: {:.2} GB", path, gb);
            },
            Err(e) => eprintln!("Error calculating size for {}: {}", path, e),
        }
    });

    let total_size = *total_size.lock().unwrap();
    let disk_space = Arc::try_unwrap(disk_space).unwrap().into_inner().unwrap();
    return (
        disk_space, 
        total_size as f64 / BYTES, 
        total_free as f64 / BYTES
    );
}

fn get_free_space<P: AsRef<Path>>(folder_path: P) -> Result<u64, String> {
    let c_path = CString::new(folder_path.as_ref().to_str().unwrap()).unwrap();
    let mut stat: statvfs = unsafe { std::mem::zeroed() };

    let result = unsafe { statvfs(c_path.as_ptr(), &mut stat) };

    if result == 0 {
        let free_space = stat.f_bfree * stat.f_bsize as u64;
        Ok(free_space)
    } else {
        Err("Failed to get filesystem statistics".to_string())
    }
}

fn calculate_dir_size<P: AsRef<Path>>(folder_path: P) -> Result<u64, String>{
    let c_path = CString::new(folder_path.as_ref().to_str().unwrap()).unwrap();
    let ignore: HashSet<&str> = [
        "/proc", "/sys", "/dev", "/run", 
        "/tmp", "/mnt", "/lost+found", 
        "/var/run", "/var/lock", "/var/tmp", "/root",
    ].iter().cloned().collect();
    
    unsafe {
        return calculate_dir_size_c(&c_path, &ignore)
    }
}

unsafe fn calculate_dir_size_c(c_path: &CString, ignore: &HashSet<&str>) -> Result<u64, String>{
    
    let dir = opendir(c_path.as_ptr());
    if dir.is_null() {
        return Err("Failed to open directory".to_string());
    }

    let mut total_size = 0;
    let mut entries = Vec::with_capacity(BATCH_SIZE);

    loop {
        let entry = readdir(dir);
        
        if entry.is_null() {
            if !entries.is_empty() {
                total_size += process_entries(&entries, ignore)?;
                entries.clear();
            }
            break;
        }
        
        let entry = &*entry;
        let entry_name = CStr::from_ptr(entry.d_name.as_ptr()).to_str().unwrap();

        // Skip current and parent directory entries
        if entry_name == "." || entry_name == ".." {
            continue;
        }

        let entry_path = format!("{}/{}", c_path.to_str().unwrap(), entry_name);
        entries.push(entry_path);

        if entries.len() >= BATCH_SIZE {
            total_size += process_entries(&entries, ignore)?;
            entries.clear();
        }
    
    }

    closedir(dir);
    Ok(total_size)
}

unsafe fn process_entries(entries: &[String], ignore: &HashSet<&str>) -> Result<u64, String> {
    
    let sizes: Vec<Result<u64, String>> = entries.par_iter().map(|entry_path| {
        let entry_path_c = CString::new(entry_path.clone()).unwrap();
        if should_ignore(&entry_path, ignore) {
            return Ok(0);
        }

        let mut stat_buf: stat = mem::zeroed();
        if libc::lstat(entry_path_c.as_ptr(), &mut stat_buf) == -1 {
            return Ok(0);
        }

        if (stat_buf.st_mode & S_IFMT) == S_IFLNK {
            return Ok(0);
        }

        if (stat_buf.st_mode & S_IFMT) == S_IFDIR {
            match calculate_dir_size_c(&entry_path_c, ignore) {
                Ok(size) => Ok(size),
                Err(_) => return Ok(0),
            }
        } else {
            Ok(stat_buf.st_size as u64)
        }
    }).collect();

    Ok(sizes.into_iter().try_fold(0, |acc, size| size.map(|s| acc + s))?)

}

fn should_ignore(entry: &str, ignore: &HashSet<&str>) -> bool {
    for ig in ignore{
        if entry.contains(ig){
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_dir_size() {
        let size = calculate_dir_size("/home").unwrap();
        assert!(size > 0);
    }

}
