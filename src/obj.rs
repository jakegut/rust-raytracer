use glam::{DVec2, Vec2};

use crate::vec3::Vec3;
use std::error::Error;
use std::str::FromStr;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

//f {vert_idx}</<uv_idx></normal_idx>> ...
// 1-indexed, so idx will be 0 if not found
pub struct FaceIdx {
    pub vert_idx: usize,
    pub uv_idx: usize,
    pub normal_idx: usize,
}

impl From<Vec<usize>> for FaceIdx {
    fn from(value: Vec<usize>) -> Self {
        let mut vert_idx = 0;
        let mut uv_idx = 0;
        let mut normal_idx = 0;

        if value.len() >= 1 {
            vert_idx = value[0];
        } else if value.len() >= 2 {
            uv_idx = value[1]
        } else if value.len() >= 3 {
            normal_idx = value[2];
        };

        Self {
            vert_idx,
            uv_idx,
            normal_idx,
        }
    }
}

pub struct FaceInfo {
    pub verts: Vec<FaceIdx>,
}

pub struct OBJ {
    pub vers: Vec<Vec3>,
    pub faces: Vec<FaceInfo>,
    pub uvs: Vec<DVec2>,
    pub normals: Vec<Vec3>,
}

pub fn load_obj(path: String) -> Result<OBJ, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut vers = vec![];
    let mut faces = vec![];
    let mut uvs = vec![];
    let mut normals = vec![];

    for line in reader.lines() {
        let str = line.expect("unable to process line");
        let parts: Vec<&str> = str.split_whitespace().collect();
        if parts.len() == 0 {
            continue;
        }
        match parts[0] {
            "v" => {
                let v = parse_parts::<f64>(&parts[1..]);
                vers.push(Vec3::new(v[0], v[1], v[2]))
            }
            "vt" => {
                let v = parse_parts(&parts[1..]);
                uvs.push(DVec2::new(v[0], v[1]));
            }
            "vn" => {
                let v = parse_parts(&parts[1..]);
                normals.push(Vec3::new(v[0], v[1], v[2]))
            }
            "f" => {
                let v = parts[1..]
                    .iter()
                    .map(|s| s.split('/').collect::<Vec<&str>>())
                    .map(|ps| {
                        ps.iter()
                            .map(|s| s.parse::<usize>().unwrap_or(0))
                            .collect::<Vec<usize>>()
                    })
                    .map(FaceIdx::from)
                    .collect::<Vec<FaceIdx>>();
                faces.push(FaceInfo { verts: v })
            }
            _ => continue,
        };
    }
    Ok(OBJ {
        vers,
        faces,
        normals,
        uvs,
    })
}

fn parse_parts<T: FromStr>(parts: &[&str]) -> Vec<T>
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    parts
        .into_iter()
        .map(|&s| s.parse::<T>().expect("String not a valid num"))
        .collect()
}
