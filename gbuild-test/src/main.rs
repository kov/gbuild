// SPDX-License-Identifier: MIT

// FIXME: This is required to get the non-test build to link propertly.
// Why?
#![allow(unused_imports, clippy::single_component_path_imports)]
use gio;

fn main () {
}

#[cfg(test)]
mod test {
    use gio::prelude::FileExt;

    #[test]
    fn load_text_resource() {
        let gfile = gio::File::for_uri("resource:///resourcetest/resources/test.txt");
        match gfile.load_contents(gio::NONE_CANCELLABLE) {
            Ok((bytes, _string)) => {
                assert_eq!(std::str::from_utf8(&bytes).unwrap(), "embedded gresource test");
            },
            Err(e) => panic!("Could not load test file: {}", e),
        }
    }
}