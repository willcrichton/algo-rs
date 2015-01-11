#![allow(dead_code)]
#![allow(unstable)]

extern crate rustc;

use rustc::util::nodemap::FnvHasher;
use std::collections::{HashMap, HashSet};
use std::collections::hash_state::DefaultState;

pub mod kth_largest;

pub mod graph;
pub mod mst;
pub mod max_flow;

pub mod point;
pub mod closest_pair;

pub type FnvMap<K, V> = HashMap<K, V, DefaultState<FnvHasher>>;
pub type FnvSet<K> = HashSet<K, DefaultState<FnvHasher>>;
