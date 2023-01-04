use fusebox::subclass::prelude::FuseImpl;
use glib::{subclass::prelude::*, Cast};
use gtk4::{prelude::*, Align, Grid};
use gtk4::{Label, Widget};

pub struct SampleFuse {
    widget: Widget,
}

impl Default for SampleFuse {
    fn default() -> Self {
        let main_grid = Grid::builder()
            .margin_start(18)
            .margin_end(18)
            .row_spacing(12)
            .build();

        let view_label = Label::builder()
            .label("Sample Fuse")
            .css_classes(vec!["view-title".to_string()])
            .build();

        let hello_label = Label::builder()
            .label("Hello from Rust!")
            .halign(Align::Start)
            .build();

        main_grid.attach(&view_label, 0, 0, 1, 1);
        main_grid.attach(&hello_label, 0, 1, 1, 1);

        Self {
            widget: main_grid.upcast(),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for SampleFuse {
    const NAME: &'static str = "SampleFuse";
    type Type = super::SampleFuse;
    type ParentType = fusebox::Fuse;
}

impl ObjectImpl for SampleFuse {}

impl FuseImpl for SampleFuse {
    fn get_widget(&self) -> Widget {
        self.widget.clone()
    }

    fn shown(&self) {
        println!("Sample fuse shown!");
    }

    fn hidden(&self) {
        println!("Sample fuse hidden!");
    }
}
