use std::collections::HashMap;
use std::mem;

use bumpalo::Bump;

/// An interred string that does not own it's underlying data. MStr can be directly compared against
/// other MStr values for string equality
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct MStr(u32);

/// Converts a normal Rust string into an interred MStr
pub struct StrStore {
    // store all the string once in the bump allocator and then reference those values in all the hash
    // map as well as the vector
    arena: Bump,
    map: HashMap<&'static str, MStr>,
    vec: Vec<&'static str>,
}

impl StrStore {
    pub fn new() -> Self {
        Self {
            arena: Bump::new(),
            map: HashMap::new(),
            vec: Vec::new(),
        }
    }

    pub fn get_mstr(&mut self, s: &str) -> MStr {
        if let Some(&id) = self.map.get(s) {
            return id;
        }

        // SAFETY: Bump allocations live in heap-allocated chunks that never
        // move. We never expose these references beyond the lifetime of `self`.
        let interned: &'static str = unsafe { mem::transmute(self.arena.alloc_str(s)) };
        let idx = self.vec.len() as u32;
        let mstr = MStr(idx);

        self.vec.push(interned);
        self.map.insert(interned, mstr);

        mstr
    }

    pub fn get_str(&self, mstr: MStr) -> &str {
        self.vec[mstr.0 as usize]
    }
}
