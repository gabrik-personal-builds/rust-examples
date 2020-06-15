

extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

use serde::{Serialize, Deserialize};


schemafy::schemafy!(
    root : Jsenginerpm
    "vehicle/com.bosch.drx_CANengineRPM_1_0_2.schema.json"
);

schemafy::schemafy!(
    root : Jscangear
    "vehicle/com.bosch.drx_CANgear_1_0_1.schema.json"
);

schemafy::schemafy!(
    root : Jsrawcandata
    "vehicle/com.bosch.drx_RawCANdata_1_0_0.schema.json"
);

schemafy::schemafy!(
    root : Jsconnectivity
    "vehicle/org.eclipse.vorto_Connectivity_1_0_0.schema.json"
);


pub type CANEngineRPM = Jsenginerpm;
pub type CANGear = Jscangear;
pub type RawCANData = Jsrawcandata;
pub type Connectivity = Jsconnectivity;


// implementation of https://vorto.eclipse.org/#/details/com.bosch.drx:Vehicle:1.0.3
#[derive(Serialize, Deserialize, Debug)]
pub struct Vehicle {
    can_engine_rpm : CANEngineRPM,
    can_gear : CANGear,
    raw_can_data : RawCANData,
    connectivity : Connectivity,
}