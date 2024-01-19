
use std::{path::{Path, PathBuf}, collections::HashMap, io::Read};

use anyhow::*;
use filetime::FileTime;
use itertools::Itertools;
use serde::Deserialize;
use rayon::prelude::*;

type GateId = i64;
type SystemId = i64;

#[derive(Debug, Clone, Deserialize)]
struct SDEGate {
    pub destination: GateId,
}

#[derive(Debug, Clone, Deserialize)]
struct SDESystem {
    pub security: f32,
    #[serde(rename = "solarSystemID")]
    pub solar_system_id: i64,
    pub stargates: HashMap<GateId, SDEGate>,
}

#[derive(Debug, Clone, Deserialize)]
struct SDEConstellation {
    #[serde(rename = "constellationID")]
    pub constellation_id: i64,
}

#[derive(Debug, Clone, Deserialize)]
struct SDERegion {
    #[serde(rename = "regionID")]
    pub region_id: i64,
}

#[derive(Debug, Clone)]
struct System {
    pub name: String,
    pub security: f32,
    pub id: SystemId,
    pub const_id: i64,
    pub constellation: String,
    pub region_id: i64,
    pub region: String,
    pub jumps: Vec<SystemId>,
}

fn main() -> Result<()> {
    let _ = dotenv::from_filename(".env.build");

    #[cfg(debug_assertions)]
    println!("cargo:rerun-if-changed={}", file!());
    
    println!("cargo:rerun-if-env-changed=EVE_SDE_ZIP_PATH");

    let sde = PathBuf::from(std::env::var("EVE_SDE_ZIP_PATH").context("EVE_SDE_ZIP_PATH environment variable must be set")?);

    if !Path::new(&sde).is_file() {
        bail!("EVE_SDE_ZIP_PATH environment variable must point to the sde zip file");
    }

    let sde_time = sde.metadata()?.modified()?;

    println!("cargo:rerun-if-changed={}", sde.to_str().unwrap());

    let out = Path::new(&std::env::var("OUT_DIR").unwrap()).join("systems.rs");

    if out.is_file() {
        let out_time = out.metadata()?.modified()?;

        if out_time == sde_time {
            return Ok(());
        }
    }

    let (systems, names) = load_universe()?;

    let mut systems = systems.into_iter().map(|(_, s)| s).collect::<Vec<_>>();

    systems.sort_by_key(|s| s.id);

    let systems = systems.into_iter()
        .map(|sys| {
            let System { name, security, id, const_id, constellation, region_id, region, jumps } = sys;
    
            let jumps = jumps.iter().map(|i| i.to_string()).join(", ");
    
            format!("    System {{ id: {id}, security: {security:?}, name: \"{name}\", constellation_id: {const_id}, constellation: \"{constellation}\", region_id: {region_id}, region: \"{region}\", jumps: &[{jumps}] }}")
        })
        .join(",\n");

    let mut names = names.into_iter().collect::<Vec<_>>();

    names.sort_by(|(_, l), (_, r)| l.cmp(r));

    let names = names.into_iter()
        .filter_map(|(id, (group, name))| {
            if group == 3 {
                Some(format!("    (\"{name}\", {id})"))
            } else {
                None
            }
        })
        .join(",\n");

    std::fs::write(&out, format!("
pub const SYSTEMS: &'static [System] = &[
{}
];
pub const REGIONS: &'static [(&'static str, i64)] = &[
{}
];
", systems, names))?;

    filetime::set_file_mtime(&out, FileTime::from_last_modification_time(&sde.metadata()?))?;

    Ok(())
}

fn load_universe() -> Result<(HashMap<i64, System>, HashMap<i64, (i64, String)>), Error> {

    let mut systems = HashMap::<SystemId, System>::new();

    let sde = std::env::var("EVE_SDE_ZIP_PATH").context("EVE_SDE_ZIP_PATH environment variable must be set")?;

    if !Path::new(&sde).is_file() {
        bail!("EVE_SDE_ZIP_PATH environment variable must point to the sde zip file");
    }

    let mut sde = zip::read::ZipArchive::new({
        std::fs::OpenOptions::new()
            .read(true)
            .open(Path::new(&sde))?
    })?;

    let mut pending_regions = Vec::new();

    let mut names = String::new();
    sde.by_name("sde/bsd/invUniqueNames.yaml")?.read_to_string(&mut names)?;

    let names = std::thread::spawn(move || {
        load_names(&names)
    });

    for idx in 0..sde.len() {
        let mut file = sde.by_index(idx)?;
        let path = match file.enclosed_name() {
            Some(p) => p.to_owned(),
            None => {
                continue;
            }
        };

        if path.starts_with("sde/fsd/universe/eve") || path.starts_with("sde/fsd/universe/wormhole") {
            let mut buf = String::new();
            file.read_to_string(&mut buf)?;

            pending_regions.push((path, buf));
        }
    }

    let names = names.join().unwrap();

    let parsed = pending_regions.par_iter().filter_map(parse).collect::<Vec<_>>();

    let mut sde_regions_by_name = HashMap::<String, i64>::new();

    let mut sde_constellations_by_name = HashMap::<String, i64>::new();

    for file in &parsed {
        match file {
            ParsedFile::Region(reg_name, reg) => {
                sde_regions_by_name.insert(reg_name.clone(), reg.region_id);
            },
            ParsedFile::Constellation(con_name, con) => {
                sde_constellations_by_name.insert(con_name.clone(), con.constellation_id);
            },
            _ => {}
        }
    }

    let mut sde_systems = HashMap::<i64, (i64, i64, &SDESystem)>::new();

    for file in &parsed {
        match file {
            ParsedFile::System(reg_name, con_name, sys) => {
                sde_systems.insert(sys.solar_system_id, (
                    *sde_regions_by_name.get(reg_name).unwrap(),
                    *sde_constellations_by_name.get(con_name).unwrap(),
                    sys
                ));
            },
            _ => {}
        }
    }

    let mut gates = HashMap::<GateId, (GateId, SystemId)>::new();

    for (reg_id, con_id, sys) in sde_systems.into_values() {
        systems.insert(sys.solar_system_id, System {
            name: names.get(&sys.solar_system_id).unwrap().1.clone(),
            security: sys.security,
            id: sys.solar_system_id,
            const_id: con_id,
            constellation: names.get(&con_id).unwrap().1.clone(),
            region_id: reg_id,
            region: names.get(&reg_id).unwrap().1.clone(),
            jumps: Vec::new(),
        });

        for (gid, gate) in sys.stargates.iter() {
            gates.insert(*gid, (gate.destination, sys.solar_system_id));
        }
    }

    for (_, (dst_gate, from)) in &gates {
        let (_, to) = gates.get(&dst_gate).unwrap();

        systems.get_mut(from).unwrap().jumps.push(*to);
    }

    Ok((systems, names))
}

fn load_names(unique_names: &str) -> HashMap<i64, (i64, String)> {
    let mut out = HashMap::new();

    let mut group_id: Option<i64> = None;
    let mut item_id: Option<i64> = None;
    let mut item_name: Option<String> = None;

    for mut line in unique_names.lines() {
        if line.starts_with("-") && item_id.is_some() && item_name.is_some() {
            
            if let Some(group) = group_id.take() {
                if matches!(group, 3 | 4 | 5) {
                    out.insert(item_id.take().unwrap(), (group, item_name.take().unwrap()));
                }
            }

            line = &line[1..];
        }

        line = line.trim();

        if line.starts_with("groupID") {
            line = line.trim_start_matches("groupID:").trim();

            group_id = Some(line.parse().unwrap());
        } else if line.starts_with("itemID") {
            line = line.trim_start_matches("itemID:").trim();

            item_id = Some(line.parse().unwrap());
        } else if line.starts_with("itemName") {
            line = line.trim_start_matches("itemName:").trim();

            item_name = Some(line.parse().unwrap());
        }
    }

    let group = group_id.take().unwrap();

    if matches!(group, 3 | 4 | 5) {
        out.insert(item_id.take().unwrap(), (group, item_name.take().unwrap()));
    }

    out
}

enum ParsedFile {
    Region(String, SDERegion),
    Constellation(String, SDEConstellation),
    System(String, String, SDESystem),
}

fn parse((path, contents): &(PathBuf, String)) -> Option<ParsedFile> {
    if path.file_name().unwrap() == "region.staticdata" {
        let reg = serde_yaml::from_str::<SDERegion>(contents).unwrap();

        let mut components = path.components();

        components.next_back();

        let reg_name = components.next_back().unwrap().as_os_str().to_string_lossy().to_string();

        Some(ParsedFile::Region(reg_name, reg))
    } else if path.file_name().unwrap() == "constellation.staticdata" {
        let con = serde_yaml::from_str::<SDEConstellation>(contents).unwrap();

        let mut components = path.components();

        components.next_back();

        let const_name = components.next_back().unwrap().as_os_str().to_string_lossy().to_string();

        Some(ParsedFile::Constellation(const_name, con))
    } else if path.file_name().unwrap() == "solarsystem.staticdata" {
        let sys = serde_yaml::from_str::<SDESystem>(contents).unwrap();

        let mut components = path.components();

        components.next_back();
        components.next_back();

        let const_name = components.next_back().unwrap().as_os_str().to_string_lossy().to_string();
        let reg_name = components.next_back().unwrap().as_os_str().to_string_lossy().to_string();

        Some(ParsedFile::System(reg_name, const_name, sys))
    } else {
        None
    }
}
