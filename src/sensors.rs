
extern crate libc;

use std::ptr;
use std::ffi::{CStr, CString};
use sensorsc as sc;

pub use sensorsc::{sensors_feature_type, sensors_subfeature_type};

#[derive(Debug)]
pub struct Sensor {
    pub prefix: String,
    pub bus_id: i16,
    pub bus_type: i16,
    pub addr: i32,
    pub path: String,
    pub features: Vec<Feature>,
}

#[derive(Debug)]
pub struct Feature {
    pub name: String,
    pub number: i32,
    pub type_: sensors_feature_type,
    pub subfeatures: Vec<SubFeature>,
}

#[derive(Debug)]
pub struct SubFeature {
    pub name: String,
    pub number: i32,
    pub type_: sensors_subfeature_type,
    pub mapping: i32,
    pub flags: u32,
}

pub fn subfeat_path(sensor: &Sensor, feature: &Feature, subfeature: &SubFeature) -> String {
    sensor.path.clone() + "/" + &subfeature.name
}


pub fn get_sensors(conf_file: &str) -> Vec<Sensor> {
    let mut sensors: Vec<Sensor> = vec![];

    unsafe {
        sc::sensors_init(libc::fopen(CString::new(conf_file)
                                         .unwrap()
                                         .into_raw(),
                                     CString::new("r")
                                         .unwrap()
                                         .into_raw()) as *mut sc::_IO_FILE);


        let mut chip_idx = 0;
        let mut chip_ptr = sc::sensors_get_detected_chips(ptr::null(), &mut chip_idx);

        while chip_ptr != ptr::null() {
            sensors.push(Sensor {
                prefix: CStr::from_ptr((*chip_ptr).prefix).to_str().unwrap().to_string(),
                bus_id: (*chip_ptr).bus.nr,
                bus_type: (*chip_ptr).bus.type_,
                addr: (*chip_ptr).addr,
                path: CStr::from_ptr((*chip_ptr).path).to_str().unwrap().to_string(),
                features: vec![],
            });
            let features = &mut sensors.last_mut().unwrap().features;

            let mut feat_idx = 0;
            let mut feat_ptr = sc::sensors_get_features(chip_ptr, &mut feat_idx);
            while feat_ptr != ptr::null() {

                features.push(Feature {
                    name: CStr::from_ptr((*feat_ptr).name).to_str().unwrap().to_string(),
                    number: (*feat_ptr).number,
                    type_: (*feat_ptr).type_,
                    subfeatures: vec![],
                });
                let subfeatures = &mut features.last_mut().unwrap().subfeatures;

                let mut subfeat_idx = (*feat_ptr).first_subfeature;
                let mut subfeat_ptr =
                    sc::sensors_get_all_subfeatures(chip_ptr, feat_ptr, &mut subfeat_idx);
                while subfeat_ptr != ptr::null() {
                    subfeatures.push(SubFeature {
                        name: CStr::from_ptr((*subfeat_ptr).name).to_str().unwrap().to_string(),
                        number: (*subfeat_ptr).number,
                        type_: (*subfeat_ptr).type_,
                        mapping: (*subfeat_ptr).mapping,
                        flags: (*subfeat_ptr).flags,
                    });

                    subfeat_ptr =
                        sc::sensors_get_all_subfeatures(chip_ptr, feat_ptr, &mut subfeat_idx);
                }


                feat_ptr = sc::sensors_get_features(chip_ptr, &mut feat_idx);
            }

            chip_ptr = sc::sensors_get_detected_chips(ptr::null(), &mut chip_idx);
        }


        sc::sensors_cleanup();
    }

    sensors
}