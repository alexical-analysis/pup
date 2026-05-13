use crate::compiler::str_store::{MStr, StrStore};

pub struct Context {
    str_store: StrStore,
}

impl Context {
    pub fn new() -> Self {
        Self {
            str_store: StrStore::new(),
        }
    }

    pub fn get_mstr(&mut self, s: &str) -> MStr {
        self.str_store.get_mstr(s)
    }

    pub fn get_str(&mut self, s: MStr) -> &str {
        self.str_store.get_str(s)
    }
}
