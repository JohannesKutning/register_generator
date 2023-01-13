use std::error::Error;
use std::fs;
use std::path::Path;
use serde_derive::Deserialize;
use serde_json_schema::Schema;

#[derive(Deserialize, Debug)]
pub struct ModuleDescription {
    pub name : String,
    pub brief : String,
    #[serde(default)]
    pub details : String,
    #[serde(default)]
    pub offset : u64,
    #[serde(default)]
    pub alignment : u64,
    #[serde(default)]
    pub register_size : u64,
    // pub registers     : Vec< RegisterDescription >,
}

impl ModuleDescription {
    pub fn new() -> ModuleDescription {
        ModuleDescription { name : String::new(), brief : String::new(), details : String::new(),
                offset : 0, alignment : 0, register_size : 0 }
    }

    pub fn with_file( file : & Path ) -> Result< ModuleDescription, Box< dyn Error > > {
        let schema = ModuleDescription::read_schema()?;
        let module = ModuleDescription::read_and_validate_description( file, & schema )?;
        Ok( module )
    }

    fn read_schema() -> Result< Schema, Box< dyn Error > > {
        let schema_file_name = "data/schema/module_description.json";
        println!( "Reading schema file {:?}", schema_file_name );
        let schema_str = fs::read_to_string( schema_file_name )?;
        let schema = Schema::try_from( schema_str )?;
        Ok( schema )
    }

    fn read_and_validate_description( file : & Path, schema : & Schema )
            -> Result< ModuleDescription, Box< dyn Error > > {
        println!( "Reading description file {:?}", file );
        let module_str = fs::read_to_string( file )?;
        let module_json : serde_json::Value = serde_json::from_str( & module_str )?;
        match schema.validate( & module_json ) {
            Ok(_)   => {},
            Err( err ) => { eprintln!( "Failed to validate the file {:?}", err ) },
        };

        let description : ModuleDescription = serde_json::from_str( & module_str )?;
        Ok( description )
    }
}

//------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_schema() -> Result< (), Box< dyn Error > > {
        ModuleDescription::read_schema()?;
        Ok(())
    }
}

