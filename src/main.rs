#![feature(rustc_private)]

#[allow(plugin_as_library)]
extern crate clippy;
extern crate rustc;
extern crate rustc_driver;
extern crate rustc_plugin;

use rustc::session::Session;
use rustc_driver::{driver, CompilerCalls, Compilation};

struct ClippyCompilerCalls;

impl<'a> CompilerCalls<'a> for ClippyCompilerCalls {
    fn build_controller(&mut self, _: &Session) -> driver::CompileController<'a> {
        let mut control = driver::CompileController::basic();

        control.after_parse.callback = Box::new(|state| {
            let mut registry = rustc_plugin::registry::Registry::new(state.session, state.krate.unwrap());
            clippy::plugin_registrar(&mut registry);

            let rustc_plugin::registry::Registry { early_lint_passes, late_lint_passes, lint_groups, llvm_passes, attributes, mir_passes, .. } = registry;
            let sess = &state.session;
            let mut ls = sess.lint_store.borrow_mut();
            for pass in early_lint_passes {
                ls.register_early_pass(Some(sess), true, pass);
            }
            for pass in late_lint_passes {
                ls.register_late_pass(Some(sess), true, pass);
            }

            for (name, to) in lint_groups {
                ls.register_group(Some(sess), true, name, to);
            }

            *sess.plugin_llvm_passes.borrow_mut() = llvm_passes;
            *sess.plugin_mir_passes.borrow_mut() = mir_passes;
            *sess.plugin_attributes.borrow_mut() = attributes;
        });
        control.after_analysis.stop = Compilation::Stop;

        control
    }
}

use std::path::Path;

fn main() {
    use std::env;

    let path = env::current_dir().unwrap();
    let path = path.join("target").join("debug").join("deps");

    let args = wrap_args(env::args(), path);
    for file in std::fs::read_dir("src").expect("no `src` directory available") {
        if let Ok(file) = file {
            let name = file.file_name();
            if let Some(name) = name.to_str() {
                if name == "lib.rs" || name == "main.rs" {
                    let mut args = args.clone();
                    args.push(format!("src/{}", name));
                    if name == "lib.rs" {
                        args.push("--crate-type".to_owned());
                        args.push("lib".to_owned());
                    }
                    println!("{:?}", args);
                    rustc_driver::run_compiler(&args, &mut ClippyCompilerCalls);
                }
            }
        }
    }
}

fn wrap_args<T, I, P>(it: I, dep_path: P) -> Vec<String>
    where T: AsRef<str>,
          I: IntoIterator<Item=T>,
          P: AsRef<Path> {

    let it = it.into_iter();
    let mut args = vec!["rustc".to_owned()];

    for arg in it.skip(2) {
        let arg = arg.as_ref().to_owned();
        args.push(arg);
    }
    args.push("-L".to_owned());
    args.push(dep_path.as_ref().to_string_lossy().into_owned());
    args.push(String::from("--sysroot"));
    args.push(format!("{}/.multirust/toolchains/nightly", std::env::var("HOME").unwrap()));
    args.push("-Zno-trans".to_owned());
    args.push("-Dclippy");
    args
}
