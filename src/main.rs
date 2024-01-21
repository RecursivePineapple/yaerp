
use std::{collections::{HashMap, HashSet, VecDeque}, sync::OnceLock, path::PathBuf};

use anyhow::{Context, bail, anyhow};
use itertools::Itertools;
use clap::Parser;

mod systems;
use crate::systems::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[doc = "A file containing copy+pasted bookmarks from the locations window in eve.\n"]
    #[doc = "Each line must match this regex: ^[A-Z]{3}-\\d{3} +[\\w\\-]+ +(-&gt; +[\\w\\-]+|\\(\\w+\\))\\t([^\\t]+\\t){2}[^\\t]+"]
    #[arg(short = 'w', long = "wormholes")]
    wormhole_bookmarks: Vec<PathBuf>,

    #[doc = "A file containing SMT-compatible connections for ansiblexes.\n"]
    #[doc = "Each line must match this regex: ^(#.*|\\d+\\s+[\\w\\-]+\\s+-->\\s+[\\w\\-]+)?$"]
    #[arg(short = 'a', long = "ansiblexes")]
    ansiblex_files: Vec<PathBuf>,

    #[doc = "Filtered jumps are not removed, but the penalties are still applied.\n"]
    #[doc = "A filtered jump is counted as 1000 jumps in the distance calculation.\n"]
    #[arg(long = "no-filter")]
    no_filter: bool,

    /// Routes will never enter Thera, Turnur, Zarzakh, or Pochven, and will try to get out as soon as possible
    #[arg(long = "no-special")]
    no_special: bool,

    /// Routes will never enter j-space, and will try to get out as soon as possible
    #[arg(long = "no-jspace")]
    no_jspace: bool,

    /// Routes will never enter nullsec, and will try to get out as soon as possible
    #[arg(long = "no-nullsec")]
    no_nullsec: bool,

    /// Routes will never enter lowsec, and will try to get out as soon as possible
    #[arg(long = "no-lowsec")]
    no_lowsec: bool,

    /// Routes will never enter highsec, and will try to get out as soon as possible
    #[arg(long = "no-highsec")]
    no_highsec: bool,

    /// The route will be exactly what is given, and no attempt will be made to optimize it.
    #[arg(long = "exact-route")]
    exact_route: bool,

    /// Routes will never enter this region, and will try to get out as soon as possible
    #[arg(short = 'r', long = "region-blacklist")]
    region_blacklist: Vec<String>,

    #[doc = "When set, only these nullsec regions will be enterable.\n"]
    #[doc = "Does not effect lowsec or highsec.\n"]
    #[arg(short = 'n', long = "ns-region-whitelist")]
    ns_region_whitelist: Vec<String>,

    /// Routes will never enter this system, and will try to get out as soon as possible
    #[arg(short = 's', long = "system-blacklist")]
    system_blacklist: Vec<String>,

    #[doc = "The systems to travel through (first is the start, last is the end).\n"]
    #[doc = "2 or more systems must be specified.\n"]
    #[doc = "The most optimal route is chosen, if more than 3 are entered (respects the start and end systems).\n"]
    #[arg(required = true, num_args = 2..)]
    waypoints: Vec<String>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Jump {
    Wormhole,
    Gate,
    Ansiblex,
}

type JumpMap = HashMap<SystemId, HashSet<(SystemId, Jump)>>;

fn load_stargates(jumps: &mut JumpMap) {
    for sys in systems::SYSTEMS {
        for j in sys.jumps {
            jumps.entry(sys.id).or_default().insert((*j, Jump::Gate));
        }
    }
}

static SYSTEMS_BY_NAME: OnceLock<HashMap<&'static str, i64>> = OnceLock::new();

fn init_systems_by_name() -> &'static HashMap<&'static str, i64> {
    SYSTEMS_BY_NAME.get_or_init(|| {
        let mut map = HashMap::new();

        for sys in systems::SYSTEMS {
            map.insert(sys.name, sys.id);
        }

        map
    })
}

fn find_system_by_name(name: &str) -> anyhow::Result<SystemId> {
    let sys = init_systems_by_name();

    if let Some(id) = sys.get(name) {
        return Ok(*id);
    }

    let name = name.to_lowercase();

    let matches = SYSTEMS.iter().filter(|s| s.name.to_lowercase().contains(&name)).collect_vec();

    match matches.len() {
        0 => {
            bail!("could not find system '{name}'");
        }
        1 => {
            Ok(matches[0].id)
        }
        more => {
            bail!("ambiguous system name '{name}': matched {more} systems");
        }
    }
}

fn find_region_by_name(name: &str) -> anyhow::Result<SystemId> {
    if let Ok(idx) = REGIONS.binary_search_by(|(n, _)| n.cmp(&name)) {
        return Ok(REGIONS[idx].1);
    }

    let name = name.to_lowercase();

    let matches = REGIONS.iter().filter(|s| s.0.to_lowercase().contains(&name)).collect_vec();

    match matches.len() {
        0 => {
            bail!("could not find region '{name}'");
        }
        1 => {
            Ok(matches[0].1)
        }
        more => {
            bail!("ambiguous region name '{name}': matched {more} regions");
        }
    }
}

fn find_system_by_id(id: SystemId) -> &'static System {
    &SYSTEMS[SYSTEMS.binary_search_by_key(&id, |s| s.id).unwrap()]
}

fn get_system_name(id: SystemId) -> &'static str {
    find_system_by_id(id).name
}

fn try_parse_wh_line(line: &str) -> anyhow::Result<(SystemId, SystemId)> {
    let cols = line.split('\t').collect::<Vec<_>>();
    let bm_name = cols[0]
        .split(' ')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    let bm_system = cols[3];

    let (a, b) = if bm_name.contains(&"-&gt;") {
        (bm_name[1], bm_name[3])
    } else {
        (bm_name[1], bm_system)
    };

    Ok((find_system_by_name(a)?, find_system_by_name(b)?))
}

fn load_wormholes(content: &str, jumps: &mut JumpMap) {
    for line in content.lines() {
        let (system_id, dst_id) = try_parse_wh_line(line).with_context(|| format!("could not parse line '{line}'")).unwrap();

        jumps.entry(system_id).or_default().insert((dst_id, Jump::Wormhole));
        jumps.entry(dst_id).or_default().insert((system_id, Jump::Wormhole));
    }
}

fn try_parse_ansi_line(mut line: &str) -> anyhow::Result<(SystemId, SystemId)> {
    line = line.trim_start_matches(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']);

    let (from, to) = line.split_once("-->").ok_or_else(|| anyhow!("could not find '-->' on line"))?;

    let from = from.trim();
    let to = to.trim();

    Ok((find_system_by_name(from)?, find_system_by_name(to)?))
}

fn load_ansiblexes(content: &str, jumps: &mut JumpMap) {
    for mut line in content.lines() {
        line = line.trim();

        if line.starts_with("#") || line.is_empty() {
            continue;
        }

        let (system_id, dst_id) = try_parse_ansi_line(line).with_context(|| format!("could not parse line '{line}'")).unwrap();

        jumps.entry(system_id).or_default().insert((dst_id, Jump::Ansiblex));
        jumps.entry(dst_id).or_default().insert((system_id, Jump::Ansiblex));
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum SystemSecurity {
    Highsec,
    Lowsec,
    Nullsec,
    Jspace,
    Special,
}

impl System {
    fn get_security_class(&self) -> SystemSecurity {
        if self.security >= 0.5 {
            return SystemSecurity::Highsec;
        }

        if self.security > 0.0 && self.security < 0.5 {
            return SystemSecurity::Lowsec;
        }

        if self.region_id == 10000070 { // Pochven
            return SystemSecurity::Special;
        }

        if /* Thera */ self.id == 31000005 || /* Turnur */ self.id == 30002086 || /* Zarzakh */ self.id == 30100000 {
            return SystemSecurity::Special;
        }

        if self.security == -0.99 { // Wormhole space
            return SystemSecurity::Jspace;
        }

        if self.security < 0.0 {
            return SystemSecurity::Nullsec;
        }

        panic!()
    }
}

struct SystemValidityChecker {
    invalid_securities: Vec<SystemSecurity>,
    invalid_regions: Vec<i64>,
    valid_ns_regions: Vec<i64>,
    invalid_systems: Vec<i64>,
}

impl SystemValidityChecker {
    pub fn new(args: &Args) -> anyhow::Result<Self> {
        let mut invalid_securities = Vec::<SystemSecurity>::new();

        if args.no_highsec {
            invalid_securities.push(SystemSecurity::Highsec);
        }

        if args.no_lowsec {
            invalid_securities.push(SystemSecurity::Lowsec);
        }

        if args.no_nullsec {
            invalid_securities.push(SystemSecurity::Nullsec);
        }

        if args.no_jspace {
            invalid_securities.push(SystemSecurity::Jspace);
        }

        if args.no_special {
            invalid_securities.push(SystemSecurity::Special);
        }

        let invalid_regions = args.region_blacklist.iter()
            .map(|r| find_region_by_name(r))
            .collect::<Result<Vec<_>, _>>()?;

        let invalid_systems = args.system_blacklist.iter()
            .map(|sys| find_system_by_name(sys))
            .collect::<Result<Vec<_>, _>>()?;

        let valid_ns_regions = args.ns_region_whitelist.iter()
            .map(|r| find_region_by_name(r))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            invalid_securities,
            invalid_regions,
            valid_ns_regions,
            invalid_systems,
        })
    }

    pub fn is_system_valid(&self, sys: &System) -> bool {
        if self.invalid_securities.contains(&sys.get_security_class()) {
            return false;
        }

        if self.invalid_systems.contains(&sys.id) {
            return false;
        }

        if self.invalid_regions.contains(&sys.region_id) {
            return false;
        }

        if sys.get_security_class() == SystemSecurity::Nullsec && self.valid_ns_regions.len() > 0 && !self.valid_ns_regions.contains(&sys.region_id) {
            return false;
        }

        true
    }
}

fn filter_jumps(jumps: &mut JumpMap, checker: &SystemValidityChecker) -> anyhow::Result<()> {
    for (from, to) in jumps.iter_mut() {
        let from_valid = checker.is_system_valid(find_system_by_id(*from));

        to.retain(|(dst, _)| {
            let to_valid = checker.is_system_valid(find_system_by_id(*dst));
            
            let invalid_jump = from_valid && !to_valid;

            !invalid_jump
        });
    }
    
    jumps.retain(|_, v| !v.is_empty());

    Ok(())
}

fn get_shortest_path(jumps: &JumpMap, from: i64, to: i64, checker: &SystemValidityChecker) -> Option<Vec<(SystemId, SystemId, Jump)>> {
    let mut parents = HashMap::<SystemId, (SystemId, Jump, i32)>::new();

    let mut queue = VecDeque::new();

    queue.push_back((from, 0));

    while let Some((curr, curr_dist)) = queue.pop_front() {
        if let Some(jumps) = jumps.get(&curr) {
            for (neighbour, via) in jumps {
                let n_valid = checker.is_system_valid(find_system_by_id(*neighbour));

                let n_dist = curr_dist + if n_valid { 1 } else { 1000 };

                let n = parents.get(neighbour);

                if n.is_none() || n_dist < n.unwrap().2 {
                    parents.insert(*neighbour, (curr, *via, n_dist));
                    queue.push_back((neighbour.clone(), n_dist));
                }
            }
        }
    }

    let mut path = Vec::new();

    let mut curr = to;

    while curr != from {
        let parent = parents.get(&curr);

        match parent {
            Some((parent, via, _)) => {
                path.push((curr, parent.clone(), *via));
                curr = *parent;
            },
            None => {
                return None;
            },
        }
    }

    path.reverse();

    Some(path)
}

#[derive(Debug)]
struct Route {
    pub start: SystemId,
    pub end: SystemId,
    pub jumps: Option<Vec<(SystemId, SystemId, Jump)>>,
}

fn main() -> anyhow::Result<()> {

    let args = Args::parse();

    let mut jumps = JumpMap::new();

    load_stargates(&mut jumps);

    for wh in &args.wormhole_bookmarks {
        load_wormholes(std::fs::read_to_string(wh)?.as_str(), &mut jumps);
    }

    for ansi in &args.ansiblex_files {
        load_ansiblexes(std::fs::read_to_string(ansi)?.as_str(), &mut jumps);
    }

    let checker = SystemValidityChecker::new(&args)?;

    if !args.no_filter {
        filter_jumps(&mut jumps, &checker)?;
    }

    let start = find_system_by_name(args.waypoints.first().unwrap())?;
    let middle: Vec<_> = args.waypoints[1..args.waypoints.len()-1].iter().map(|s| find_system_by_name(s)).collect::<Result<Vec<_>, _>>()?;
    let end = find_system_by_name(args.waypoints.last().unwrap())?;

    let mut paths = Vec::new();

    let potential_routes = if args.exact_route {
        let mut systems = Vec::new();

        systems.push(start);
        systems.extend(middle.into_iter());
        systems.push(end);

        vec![systems]
    } else {
        middle.iter()
            .permutations(middle.len())
            .map(|middle| {
                let mut systems = Vec::new();
        
                systems.push(start);
                systems.extend(middle.into_iter().copied());
                systems.push(end);

                systems
            })
            .collect_vec()
    };

    for systems in potential_routes.into_iter() {

        let mut route = Vec::new();

        for i in 0..systems.len() - 1 {
            route.push(Route {
                start: systems[i],
                end: systems[i + 1],
                jumps: get_shortest_path(&jumps, systems[i], systems[i + 1], &checker)
            });
        }
    
        let info = (
            route.iter().all(|r| r.jumps.is_some()),
            route.iter().map(|r| r.jumps.as_ref().map(|v| v.len()).unwrap_or_default()).sum::<usize>()
        );

        paths.push((
            systems,
            route,
            info,
        ));
    }

    let (systems, route, (valid, jumps)) = paths.into_iter()
        .min_by_key(|(_, _, (valid, len))| (if *valid { 0 } else { 1 }, *len)).unwrap();

    println!("\nBest route:");

    for r in route {
        if let Some(jumps) = r.jumps {
            println!("\nFrom {} to {}: ({} jumps)", get_system_name(r.start), get_system_name(r.end), jumps.len());

            for (to, from, via) in jumps {

                let to_sys = find_system_by_id(to);

                println!(
                    "  {} -> {} ({:.2}, {}, via {}){}",
                    get_system_name(from),
                    get_system_name(to),
                    to_sys.security,
                    to_sys.region,
                    match via {
                        Jump::Wormhole => "wormhole",
                        Jump::Gate => "gate",
                        Jump::Ansiblex => "ansiblex",
                    },
                    match (checker.is_system_valid(find_system_by_id(from)), checker.is_system_valid(to_sys)) {
                        (true, false) => "    Warning: entering filtered system",
                        (false, false) => "    Warning: both systems are filtered out",
                        (true, true) => "",
                        (false, true) => ""
                    }
                );
            }
        } else {
            println!("\nNo route from {} to {}", get_system_name(r.start), get_system_name(r.end));
        }
    }

    println!("\nShorthand route:");

    for system in systems {
        println!("  {}", get_system_name(system));
    }

    println!("\nTotal jumps: {jumps}");

    if !valid {
        println!("\nWarning: could not find a complete route; your restrictions are likely too strict");
    }

    Ok(())
}
