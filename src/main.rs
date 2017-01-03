
use std::ptr;
use std::ffi::CString;

extern crate gtk;
use gtk::prelude::*;
use gtk::{Button, Window, WindowType, Box, Orientation, ListBox, Label};

extern crate libc;

mod sensors;
use sensors::*;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("First GTK+ Program");
    window.set_default_size(350, 70);


    let main_box = Box::new(Orientation::Horizontal, 0);
    window.add(&main_box);

    let fan_list = ListBox::new();
    main_box.pack_start(&fan_list, false, true, 0);

    // for fan in &["Fan0", "Fan1", "Custom fan"] {
    //     fan_list.insert(&Label::new(Some(fan)), -1);
    // }


    unsafe {
        sensors_init(libc::fopen(CString::new("/etc/sensors3.conf")
                                     .unwrap()
                                     .into_raw(),
                                 CString::new("r")
                                     .unwrap()
                                     .into_raw()) as *mut _IO_FILE);
    }

    let mut chip_ptr: *const sensors_chip_name;
    let mut i: i32 = 0;

    println!("Looking for chips...");
    unsafe {
        chip_ptr = sensors_get_detected_chips(ptr::null(), &mut i);

        while chip_ptr != ptr::null() {
            let chip = *chip_ptr;
            println!("Chip found: {:?}", &chip);


            let path = CString::from_raw((&chip).path).into_string().unwrap();
            let prefix = CString::from_raw((&chip).prefix).into_string().unwrap();



            fan_list.insert(&Label::new(Some(&path)), -1);

            i = i + 1;
            chip_ptr = sensors_get_detected_chips(ptr::null(), &mut i);
        }
    }
    println!("done");






    window.show_all();
    gtk::main();
}