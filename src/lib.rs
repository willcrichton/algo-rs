#![feature(globs)]
#![feature(default_type_params)]
#![allow(dead_code)]
extern crate rustc;

use rustc::util::nodemap::FnvHasher;
use std::collections::{HashMap, HashSet};

mod kth_largest;

mod graph;
mod mst;
mod max_flow;

mod point;
mod closest_pair;

pub type FnvMap<K, V> = HashMap<K, V, FnvHasher>;
pub type FnvSet<K> = HashSet<K, FnvHasher>;
