// SPDX-License-Identifier: MIT

fn main() {
    gbuild::Resource::builder()
        .src_dir("src")
        .definition_file("src/gbuild-test.gresource.xml")
        .build()
        .expect("Bad arguments for gresource compilation")
        .compile();
}