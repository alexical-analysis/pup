use std::collections::HashMap;
use std::mem;

use bumpalo::Bump;

use crate::types::checked_ty::{CheckedTy, CheckedTyValue};

pub struct TyStore {
    arena: Bump,
    map: HashMap<&'static CheckedTyValue, CheckedTy>,
    vec: Vec<&'static CheckedTyValue>,
}

impl TyStore {
    pub fn new() -> Self {
        Self {
            arena: Bump::new(),
            map: HashMap::new(),
            vec: Vec::new(),
        }
    }

    pub fn get_ty(&mut self, type_value: CheckedTyValue) -> CheckedTy {
        if let Some(&id) = self.map.get(&type_value) {
            return id;
        }

        // SAFETY: Bump allocations live in heap-allocated chunks that never
        // move. We never expose these references beyond the lifetime of `self`.
        let interned: &'static CheckedTyValue =
            unsafe { mem::transmute(self.arena.alloc(type_value)) };
        let idx = self.vec.len() as u32;
        let ty = CheckedTy(idx);

        self.vec.push(interned);
        self.map.insert(interned, ty);

        ty
    }
}
