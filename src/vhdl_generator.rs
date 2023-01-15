use std::error::Error;
use source_generator::vhdl::vhdl_file::VhdlFile;
use source_generator::element::Element;
use source_generator::vhdl::entity::Entity;
use crate::module_description::ModuleDescription;
use crate::register_description::RegisterDescription;

pub struct VhdlGenerator {
    sources : Vec< VhdlFile >
}

impl VhdlGenerator {
    pub fn new() -> VhdlGenerator {
        VhdlGenerator { sources : Vec::new() }
    }

    pub fn create_source_code( & mut self, description : & ModuleDescription )
            -> Result< (), Box< dyn Error > > {
        self.create_module_source_code( description )?;
        Ok(())
    }

    pub fn write_source_files( & self, _dirname : & str )
            -> Result< (), Box< dyn Error > > {
        Ok(())
    }

    fn create_module_source_code( & mut self, description : & ModuleDescription )
            -> Result< (), Box< dyn Error > > {
        let mut entity = Entity::new( & description.name );

        let mut entity_comment = String::new();
        if ! description.brief.is_empty() {
            entity_comment.push_str( & format!( "{}\n", description.brief ) );
        }
        if ! description.details.is_empty() {
            entity_comment.push_str( & description.details );
        }
        if ! entity_comment.is_empty() {
            entity.add_description( & entity_comment )
        }

        let mut file = VhdlFile::new( & description.name );
        file.add_entity( entity );
        self.sources.push( file );

        Ok(())
    }
}

//------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_simple_module_source_code() -> Result< (), Box< dyn Error > > {
        let description = ModuleDescription{ name : "test".to_string(),
            brief : "Brief simple module description".to_string(),
            details : "Detailed simple module description\nwith two lines".to_string(),
            alignment : 4,
            offset : 0,
            register_size : 4,
            registers : Vec::new() };
        let mut generator = VhdlGenerator::new();
        generator.create_source_code( & description )?;
        let expected = concat!( "-- Brief simple module description\n",
            "-- Detailed simple module description\n",
            "-- with two lines\n",
            "entity test is\n",
            "begin\n",
            "end entity test;\n\n" );

        assert_eq!( expected, generator.sources[ 0 ].to_source_code( 0 ) );
        Ok(())
    }

    #[test]
    fn create_module_source_code_with_read_only_register() -> Result< (), Box< dyn Error > > {
        let register = RegisterDescription {
            name : "reg1".to_string(),
            brief : "A brief register description".to_string(),
            details : "A detailed register description\nwith two lines".to_string(),
            offset : 0,
            size : 4,
        };
        let registers = vec![ register ];
        let description = ModuleDescription{ name : "test".to_string(),
            brief : "Brief simple module description".to_string(),
            details : "Detailed simple module description\nwith two lines".to_string(),
            alignment : 4,
            offset : 0,
            register_size : 4,
            registers : registers };

        let mut generator = VhdlGenerator::new();
        generator.create_source_code( & description )?;
        let expected = concat!( "-- Brief simple module description\n",
            "-- Detailed simple module description\n",
            "-- with two lines\n",
            "entity test is\n",
            "begin\n",
            "end entity test;\n\n" );

        assert_eq!( expected, generator.sources[ 0 ].to_source_code( 0 ) );
        Ok(())
    }
}

