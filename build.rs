use std::{
    env, 
    fs, 
    path::{
        Path, 
        PathBuf, 
    }, 
};

const COPY_DIRS: [&str; 1] = ["assets"];

fn copy_dir<P, Q>(from: P, to: Q) where P: AsRef<Path>, Q: AsRef<Path> {
    let to: PathBuf = to.as_ref().to_path_buf();

    for path in fs::read_dir(from).unwrap() {
        let path: PathBuf = path.unwrap().path();
        let to: PathBuf = to.clone().join(path.file_name().unwrap());

        if path.is_file() {
            fs::copy(&path, to).unwrap();
        } else if path.is_dir() {
            if !to.exists() {
                fs::create_dir(&to).unwrap();
            }
            copy_dir(&path, to);
        }
    }
}

fn main() {
    for directory in COPY_DIRS {
        let out  = PathBuf::from(format!("target/{}/{}", env::var("PROFILE").unwrap(), directory));

        if out.exists() {
            fs::remove_dir_all(&out).unwrap();
        }
        fs::create_dir(&out).unwrap();
        copy_dir(directory, &out);
    }
}