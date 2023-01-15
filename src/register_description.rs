use serde_derive::Deserialize;
//use self::field::Field;

#[derive(Deserialize, Debug)]
pub struct RegisterDescription {
    pub name : String,
    pub brief : String,
    #[serde(default)]
    pub details : String,
    #[serde(default)]
    pub offset : u64,
    #[serde(default)]
    pub size : u64,
//    fields      : Vec< Field >,
}

