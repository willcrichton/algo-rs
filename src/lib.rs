#![allow(dead_code)]

extern crate rustc;

use rustc::util::nodemap::FnvHasher;
use std::collections::{HashMap, HashSet};

pub mod kth_largest;

pub mod graph;
pub mod mst;
pub mod max_flow;

pub mod point;
pub mod closest_pair;

pub type FnvMap<K, V> = HashMap<K, V, FnvHasher>;
pub type FnvSet<K> = HashSet<K, FnvHasher>;
