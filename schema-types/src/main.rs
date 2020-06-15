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


    let fdu = InfraFDU {
        uuid : Some(String::from("")),
        fdu_id : Some(String::from("")),
        status : Some(serde_json::Value::String(String::from("DEFINE"))),
        image : None,
        command : None,
        computation_requirements : None,
        geographical_requirements : None,
        energy_requirements : None,
        configuration : None,
        interfaces : None,
        storage : None,
        hypervisor : Some(serde_json::Value::String(String::from("LXD"))),
        migration_kind : Some(serde_json::Value::String(String::from("COLD"))),
        connection_points : None,
        io_ports : None,
        depends_on : None,
        error_code : None,
        error_msg : None,
        migration_properties : None,
        hypervisor_info : None,
    };

    let fdu_js = serde_json::to_string(&fdu).unwrap();
    println!("Serialized JSON:\n{}\n", fdu_js);
    let fdu_des : InfraFDU = serde_json::from_str(&fdu_js).unwrap();
    println!("Deserialized JSON: {:?}\n", fdu_des);


    let fdu_yml = serde_yaml::to_string(&fdu).unwrap();
    println!("Serialized YAML:\n{}\n", fdu_yml);
    let fdu_ydes : InfraFDU = serde_yaml::from_str(&fdu_yml).unwrap();
    println!("Deserialized YAML : {:?}", fdu_ydes);


}
