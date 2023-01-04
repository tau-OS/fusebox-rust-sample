use std::{collections::HashMap, ffi::c_void};

use fusebox::FuseCategory;
use glib::Object;
use util::str_hashmap_to_glib_hashtable_value;

mod imp;
mod util;

glib::wrapper! {
    pub struct SampleFuse(ObjectSubclass<imp::SampleFuse>)
        @extends fusebox::Fuse;
}

impl SampleFuse {
    pub fn new() -> Self {
        let supported_settings: HashMap<&str, Option<&str>> = HashMap::from([("wallpaper", None)]);
        Object::new(&[
            ("category", &FuseCategory::Personal),
            ("code-name", &"sample-fuse"),
            ("display-name", &"Sample Fuse"),
            ("description", &"Does nothing, nya!"),
            ("icon", &"system-run"),
            ("supported-settings", &unsafe {
                str_hashmap_to_glib_hashtable_value(supported_settings)
            }),
        ])
    }
}

#[no_mangle]
pub unsafe extern "C" fn get_fuse(_: c_void) -> SampleFuse {
    println!("Activating Sample fuse from Rust!");
    gtk4::init().unwrap();
    SampleFuse::new()
}
