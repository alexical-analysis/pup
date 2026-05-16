use std::fs;
use std::path::PathBuf;

use crate::ast::parser::Parser;
use crate::compiler::context::Context;
use crate::compiler::module::Module;

pub struct File {
    path: PathBuf,
    source: String,
}

pub struct Builder<'ctx> {
    ctx: &'ctx mut Context,
}

impl<'ctx> Builder<'ctx> {
    pub fn compile(&mut self, root: PathBuf) {
        let files = self.get_all_files(root);
        let modules = self.get_all_modules(&files);

        // TODO: build the module DAG
        // TODO: loop of the modules in the module DAG
        //     TODO: run the noder on the HirModule
        //     TODO: run the grapher on the MirModule
        //     TODO: run codegen on the GenStore
        //
        // TODO: run the linker on the module DAG
    }

    fn get_all_files(&self, root: PathBuf) -> Vec<File> {
        let mut files = vec![];
        for entry in fs::read_dir(root).expect("failed to read root") {
            let path = match entry {
                Ok(e) => e.path(),
                Err(e) => {
                    eprintln!("failed to get file {:?}", e);
                    continue;
                }
            };

            if path.is_dir() {
                continue;
            }

            let ext = match path.extension() {
                Some(ext) => ext,
                None => continue,
            };

            if ext != "pup" {
                continue;
            }

            let file = match fs::read_to_string(&path) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("failed to read .pup file {:?}", e);
                    continue;
                }
            };

            files.push(File { path, source: file })
        }

        files
    }

    fn get_all_modules(&mut self, files: &[File]) -> Vec<Module> {
        let mut modules = vec![];
        for file in files {
            let module = self.ctx.create_module(file.path.clone());
            modules.push(module);

            let mut ast_module = self.ctx.get_ast_module(module);
            let mut parser = Parser::new(&mut ast_module);
            parser.parse(&file.source);
        }

        modules
    }
}
