pub mod lib;

extern crate serde;
extern crate serde_json;
extern crate serde_yaml;


use lib::types::*;


fn main() {

    let vehicle = Vehicle {
        can_engine_rpm : CANEngineRPM {value : 500.0, error: None, unit: None},
        can_gear : CANGear {value : String::from("5"), error : None},
        raw_can_data : RawCANData {data : String::from("")},
        connectivity : Connectivity {last_seen : 1.0, rssi : None, snr : None, status : None}
    };


    let js = serde_json::to_string(&vehicle).unwrap();
    println!("Serialized JSON:\n{}\n", js);
    let des : Vehicle = serde_json::from_str(&js).unwrap();
    println!("Deserialized JSON: {:?}\n", des);


    let yml = serde_yaml::to_string(&vehicle).unwrap();
    println!("Serialized YAML:\n{}\n", yml);
    let ydes : Vehicle = serde_yaml::from_str(&yml).unwrap();
    println!("Deserialized YAML : {:?}", ydes);

}
