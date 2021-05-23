// SPDX-License-Identifier: MIT

#[macro_use]
extern crate derive_builder;

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use quick_xml::Reader;
use quick_xml::events::Event;

#[derive(Builder, Default, Debug)]
#[builder(try_setter, setter(into), build_fn(validate = "Self::validate"))]
pub struct Resource {
    src_dir: PathBuf,
    definition_file: PathBuf,
}

impl Resource {
    pub fn builder() -> ResourceBuilder {
        ResourceBuilder::default()
    }

    pub fn compile(&self) {
        let out_dir = env::var_os("OUT_DIR").unwrap();

        let src_dir = self.src_dir.as_path();

        let definition_basename = self.definition_file.as_path().file_name()
            .unwrap_or_else(|| panic!("Bad definition path: {}", &self.definition_file.as_path().to_string_lossy()))
            .to_string_lossy()
            .to_string();

        let mut xml_reader = Reader::from_file(&self.definition_file)
            .unwrap_or_else(|error| {
                panic!("Unable to open {}: {}",
                    &self.definition_file.as_path().to_string_lossy(),
                    error
                );
            });
        let mut buf = vec![];
        let mut inside_file_tag = false;
        loop {
            match xml_reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    if let b"file" = e.name() {
                        inside_file_tag = true;
                    }
                },
                Ok(Event::Text(e)) => {
                    if !inside_file_tag {
                        continue;
                    }

                    inside_file_tag = false;

                    let relative_path = e.unescape_and_decode(&xml_reader).unwrap();
                    let mut file_path = src_dir.to_path_buf();
                    file_path.push(relative_path);

                    println!("cargo:rerun-if-changed={}", &file_path.to_string_lossy());
                },
                Err(e) => panic!("Error at position {}: {:?}", xml_reader.buffer_position(), e),
                Ok(Event::Eof) => break,
                _ => (),
            }
            buf.clear();
        }

        let cfilename = match definition_basename.strip_suffix(".xml") {
            Some(basename) => format!("{}.c", basename),
            None => panic!("Bad extension for definition file, expected .xml: {}", definition_basename),
        };
        let cfile_path = Path::new(&out_dir).join(&cfilename);
        let dest_path = cfile_path.to_string_lossy();

        Command::new("glib-compile-resources")
            .arg("--generate-source")
            .arg(self.definition_file.as_path())
            .arg(format!("--sourcedir={}", &src_dir.to_string_lossy()).as_str())
            .arg(format!("--target={}", &dest_path))
            .output()
            .expect("unable to generate resources source file");

        let cflags = Command::new("pkg-config")
            .arg("--cflags")
            .arg("--static")
            .arg("--libs")
            .arg("gio-2.0")
            .output()
            .map_or_else(
                |e| {
                    panic!("Could not get cflags from pkg-config for gio-2.0: {}", e)
                },
                |output| {
                    std::str::from_utf8(&output.stdout).unwrap().to_string()
            });

        let old_cflags = env::var_os("CFLAGS");

        env::set_var("CFLAGS", cflags);
        cc::Build::new()
            .file(&*dest_path)
            .compile(cfilename.strip_suffix(".c").unwrap());

        match old_cflags {
            Some(old_value) => env::set_var("CFLAGS", old_value),
            None => env::remove_var("CFLAGS"),
        }

        println!("cargo:rerun-if-changed={}", &self.definition_file.as_path().to_string_lossy());
        println!("cargo:rerun-if-changed=build.rs");
    }
}

impl ResourceBuilder {
    fn validate(&self) -> Result<(), String> {
        if let Some(ref src_dir) = self.src_dir {
            if src_dir.as_path().is_dir() {
                Ok(())
            } else {
                Err(format!("{} is not a directory",
                    &src_dir.as_os_str().to_string_lossy())
                )
            }
        } else {
            Err("src_dir not provided".to_string())
        }
    }
}