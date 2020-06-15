extern crate serde;
// extern crate schemafy_core;
extern crate serde_json;
extern crate serde_yaml;

use serde::{Serialize, Deserialize};


schemafy::schemafy!(
    root : CANEngineRPM
    "vehicle/com.bosch.drx_CANengineRPM_1_0_2.schema.json"
);

schemafy::schemafy!(
    root : CANGear
    "vehicle/com.bosch.drx_CANgear_1_0_1.schema.json"
);

schemafy::schemafy!(
    root : RawCANData
    "vehicle/com.bosch.drx_RawCANdata_1_0_0.schema.json"
);

schemafy::schemafy!(
    root : Connectivity
    "vehicle/org.eclipse.vorto_Connectivity_1_0_0.schema.json"
);


#[derive(Serialize, Deserialize, Debug)]
struct Vehicle {
    can_engine_rpm : CanengineRPM,
    can_gear : Cangear,
    raw_can_data : RawCANData,
    connectivity : Connectivity,
}





fn main() {

    let vehicle = Vehicle {
        can_engine_rpm : CanengineRPM {value : 500.0, error: None, unit: None},
        can_gear : Cangear {value : String::from("5"), error : None},
        raw_can_data : RawCANData {data : String::from("")},
        connectivity : Connectivity {last_seen : 1.0, rssi : None, snr : None, status : None}
    };


    let js = serde_json::to_string(&vehicle).unwrap();
    println!("Serialized JSON:\n{}\n", js);
    let des : Vehicle = serde_json::from_str(&js).unwrap();
    println!("Deserialized JSON: {:?}", des);


    let yml = serde_yaml::to_string(&vehicle).unwrap();
    println!("Serialized YAML:\n{}\n", yml);
    let ydes : Vehicle = serde_yaml::from_str(&yml).unwrap();
    println!("Deserialized YAML : {:?}", ydes);

}
