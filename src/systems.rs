
pub type SystemId = i64;

#[derive(Debug, Clone)]
pub struct System {
    pub id: SystemId,
    pub name: &'static str,
    pub security: f32,
    pub constellation_id: i64,
    pub constellation: &'static str,
    pub region_id: i64,
    pub region: &'static str,
    pub jumps: &'static [SystemId],
}

// rust analyzer doesn't like it when the systems are actually loaded

#[cfg(not(empty_systems))]
include!(concat!(env!("OUT_DIR"), "/systems.rs"));

#[cfg(empty_systems)]
pub const SYSTEMS: &'static [System] = &[];

#[cfg(empty_systems)]
pub const REGIONS: &'static [(&'static str, i64)] = &[];
