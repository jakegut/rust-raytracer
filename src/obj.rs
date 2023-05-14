use crate::vec3::Vec3;
use std::error::Error;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub struct OBJ {
    pub ver_array: Vec<Vec3>,
    pub face_array: Vec<(u32, u32, u32)>,
}

pub fn load_obj(path: String) -> Result<OBJ, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut ver_array = vec![];
    let mut face_array = vec![];
    for line in reader.lines() {
        let str = line.expect("unable to process line");
        let parts: Vec<&str> = str.split_whitespace().collect();
        if parts.len() == 0 {
            continue;
        }
        match parts[0] {
            "v" => {
                let v: Vec<f64> = parts[1..]
                    .into_iter()
                    .map(|s| s.parse::<f64>().expect("String not a valid float"))
                    .collect();
                ver_array.push(Vec3::new(v[0], v[1], v[2]))
            }
            "f" => {
                let v: Vec<u32> = parts[1..]
                    .into_iter()
                    .map(|s| s.parse::<u32>().expect("Not a valid u32"))
                    .collect();
                face_array.push((v[0], v[1], v[2]));
            }
            _ => continue,
        };
    }
    Ok(OBJ {
        ver_array,
        face_array,
    })
}
