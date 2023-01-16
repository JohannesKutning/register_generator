use std::error::Error;
use source_generator::vhdl::vhdl_file::VhdlFile;
use source_generator::element::Element;
use source_generator::vhdl::entity::Entity;
use source_generator::vhdl::architecture::Architecture;
use crate::module_description::ModuleDescription;
use crate::register_description::RegisterDescription;

pub struct VhdlGenerator<'a> {
    files : Vec< VhdlFile >,
    description : &'a ModuleDescription,
}

impl<'a> VhdlGenerator<'a> {
    pub fn new( description : &'a ModuleDescription ) -> VhdlGenerator {
        VhdlGenerator { files : Vec::new(), description : description }
    }

    pub fn create_source_code( & mut self )
            -> Result< (), Box< dyn Error > > {
        self.create_register_source_code()?;
        self.create_module_source_code()?;
        Ok(())
    }

    pub fn write_source_files( & self, _dirname : & str )
            -> Result< (), Box< dyn Error > > {

        //for file in self.files {
        //    file.write_to_folder( dirname )?;
        //}
        Ok(())
    }

    fn create_register_source_code( & mut self ) -> Result< (), Box< dyn Error > > {
        let mut file = VhdlFile::new( & self.get_register_entity_name() );
        file.add_entity( self.create_register_entity() );
        file.add_architecture( self.create_register_architecture() );
        self.files.push( file );
        Ok(())
    }

    fn create_register_entity( & self ) -> Entity {
        let entity = Entity::new( & self.get_register_entity_name() );
        return entity;
    }

    fn create_register_architecture( & self ) -> Architecture {
        let architecture = Architecture::new( "rtl", & self.get_register_entity_name() );
        return architecture;
    }

    fn get_register_entity_name( & self ) -> String {
        format!( "{}_registers", self.description.name )
    }

    fn create_module_source_code( & mut self ) -> Result< (), Box< dyn Error > > {
        let mut file = VhdlFile::new( & self.description.name );
        file.add_entity( self.create_module_entity() );
        self.files.push( file );
        Ok(())
    }

    fn create_module_entity( & self ) -> Entity {
        let mut entity = Entity::new( & self.description.name );
        let entity_description = self.create_module_entity_description();
        if ! entity_description.is_empty() {
            entity.add_description( & entity_description )
        }

        return entity;
    }

    fn create_module_entity_description( & self ) -> String {
        let mut entity_description = String::new();
        if ! self.description.brief.is_empty() {
            entity_description.push_str( & format!( "{}\n", self.description.brief ) );
        }
        if ! self.description.details.is_empty() {
            entity_description.push_str( & self.description.details );
        }

        return entity_description;
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
        let mut generator = VhdlGenerator::new( & description );
        generator.create_source_code()?;
        let expected = concat!( "-- Brief simple module description\n",
            "-- Detailed simple module description\n",
            "-- with two lines\n",
            "entity test is\n",
            "begin\n",
            "end entity test;\n\n" );

        assert_eq!( expected, generator.files[ 1 ].to_source_code( 0 ) );
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

        let mut generator = VhdlGenerator::new( & description );
        generator.create_source_code()?;
        let expected = concat!( "-- Brief simple module description\n",
            "-- Detailed simple module description\n",
            "-- with two lines\n",
            "entity test is\n",
            "begin\n",
            "end entity test;\n\n" );

        assert_eq!( expected, generator.files[ 1 ].to_source_code( 0 ) );
        Ok(())
    }
}

