# gbuild

A library to perform the usual non-compile build tasks on glib/gio/gtk-based
libray and applications.

Right now it only supports embedding gio GResources into the final binary.

[Documentation](https://docs.rs/gbuild)

## Using gbuild

First, you'll want to both add a build script for your crate (`build.rs`) and
also add this crate to your `Cargo.toml` via:

```toml
[build-dependencies]
gbuild = "0.1"
```

Next up, you'll want to write a build script like so:

```rust,no_run
// build.rs
fn main() {
    gbuild::Resource::builder()
        .src_dir("src")
        .definition_file("src/br.kov.duplikat.gresource.xml")
        .build()
        .expect("Bad arguments for gresource compilation")
        .compile();
}
```

That will generate the `.c` file from the `xml`description, build it, and it will
be linked into your rust binary, making it possible to use resources from your
code without any further registration like this, for instance:

```rust,no_run
fn load_readme() -> Result<String, String>
    let gfile = gio::File::for_uri("resource:///br/kov/duplikat/README");
    match gfile.load_contents(gio::NONE_CANCELLABLE) {
        Ok((bytes, _string)) => {
            std::str::from_utf8(&bytes).unwrap().to_string()
        },
        Err(e) => format!("Could not load test file: {}", e),
    }
```

## External configuration

This crate uses the `pkg-config` binary (not the crate) to obtain the flags for
including and linking `gio-2.0`, so it will be affected by any environment variable
that affects `pkg-config`.

For building the resulting C file, this crate uses the [`cc` crate](https://crates.io/crates/cc),
which can also be affected by various environment variables. Do note that at the moment
this crate will replace any existing `CFLAGS` variable with its own while building the
resources `c` file.

# License

This project is licensed under the MIT license, see the [LICENSE](LICENSE) file.
