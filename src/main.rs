
mod sensorsc;
mod sensors;

extern crate gtk;
use gtk::prelude::*;
#[allow(unused_imports)]
use gtk::{Window, WindowType, Box, Orientation, ListBox, Label, Stack, ListBoxRow, StackSidebar,
          Separator, ScrolledWindow, Adjustment};
extern crate glib;


use sensors::*;


fn remove_nul(s: &str) -> String {
    // println!("##########> {:?} -> {:?}", s.as_bytes(), s.replace("\0", "").as_bytes());

    // s.replace("\0", "")
    s.to_string()
}

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    println!("Using GTK {}.{}", gtk::get_major_version(), gtk::get_minor_version());

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Fan control");
    window.set_default_size(350, 400);

    let main_box = Box::new(Orientation::Horizontal, 0);
    window.add(&main_box);

    let sensors_sidebar = StackSidebar::new();
    main_box.pack_start(&sensors_sidebar, false, true, 0);

    let sensors_stack = Stack::new();
    main_box.pack_end(&sensors_stack, true, true, 0);


    let sensors = get_sensors("/etc/sensors3.conf");
    println!("{} sensors", sensors.len());

    for ref sensor in &sensors {
        println!(">{} @{}", &sensor.prefix, &sensor.path);

        let scroll = ScrolledWindow::new(None, None);
        sensors_stack.add_titled(&scroll,
                                 &remove_nul(&(sensor.addr.to_string() + "-" + &sensor.prefix.to_string())),
                                 &remove_nul(&sensor.prefix));


        let sensor_page = Box::new(Orientation::Vertical, 0);
        scroll.add(&sensor_page);

        for ref feat in &sensor.features {
            if feat.type_ == sensors_feature_type::SENSORS_FEATURE_FAN {

                let fan_container = Box::new(Orientation::Vertical, 0);
                sensor_page.add(&fan_container);

                let fan_name = Label::new(None);
                fan_name.set_markup(&("<big>- ".to_string() + &feat.name + &" -</big>".to_string()));
                fan_container.add(&fan_name);

                for ref subfeat in &feat.subfeatures {
                    let lbl = Label::new(Some(&subfeat.name));
                    fan_container.add(&lbl);

                    let path = subfeat_path(&sensor, &feat, &subfeat);
                }

                sensor_page.add(&Separator::new(Orientation::Horizontal));
            }

        }

    }

    sensors_sidebar.set_stack(&sensors_stack);

    println!("GTK Setup done");

    window.show_all();
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    gtk::main();
}


use std::fs::File;
use std::sync::Arc;
use std::marker::Send;

// #[derive(Send)]
struct ValueViewer {

    file: File,
}
unsafe impl std::marker::Send for ValueViewer {}
impl ValueViewer {
    fn new(name: &str, path: &str) -> Arc<ValueViewer> {
        let mut ret = Arc::new(ValueViewer{
            file: File::open(path).unwrap(),
        });


        {
            let mut ret2 = ret.clone();
            glib::timeout_add(500, move ||{
                let mut s = String::new();

                use std::io::Read;
                ret2.file.read_to_string(&mut s);
                
                Continue(true)
            });
        }

        ret
    }
}