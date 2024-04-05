use std::path::Path;
use libc::mkfifo;
use std::fs;
use std::ffi::CString;

pub fn create_pipe(slippi_path: &Path) -> fs::File {

    let pipe_dir = slippi_path.join("Pipes");
    let pipe_path = pipe_dir.join("macboxx");
    if pipe_path.exists() {
        return fs::File::create(&pipe_path).unwrap();
    }

    std::fs::create_dir_all(&pipe_dir).unwrap();
    let pipe_filename = CString::new(pipe_path.to_str().unwrap().as_bytes()).unwrap();
    unsafe {
        if mkfifo(pipe_filename.as_ptr(), 0444) != 0 {
            panic!("failed to make fifo");
        }
    }
    println!("Connected to slippi!");
    fs::File::create(&pipe_path).unwrap()
}
