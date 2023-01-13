use std::error::Error;
use std::path::PathBuf;
use crate::module_description::ModuleDescription;
use crate::vhdl_generator::VhdlGenerator;

pub struct ModuleGenerator {
    description : ModuleDescription,
}

impl ModuleGenerator {
    pub fn new() -> ModuleGenerator {
        ModuleGenerator { description : ModuleDescription::new() }
    }

    pub fn parse( & mut self, filename : & str ) -> Result< (), Box< dyn Error > > {
        self.description = ModuleDescription::with_file( & PathBuf::from( filename ) )?;
        Ok(())
    }

    pub fn generate_and_write_vhdl( & self, dirname : & str ) -> Result< (), Box< dyn Error > > {
        println!( "TODO implement ModuleGenerator::generate_and_write_vhdl {:?}", dirname );
        let mut generator = VhdlGenerator::new();
        generator.create_source_code( & self.description )?;
        generator.write_source_files( dirname )?;
        Ok(())
    }

    pub fn generate_and_write_c( & self, filename : & str ) -> Result< (), Box< dyn Error > > {
        println!( "TODO implement ModuleGenerator::generate_and_write_c {:?}", filename );
        Ok(())
    }

    pub fn generate_and_write_cpp( & self, filename : & str ) -> Result< (), Box< dyn Error > > {
        println!( "TODO implement ModuleGenerator::generate_and_write_cpp {:?}", filename );
        Ok(())
    }

    pub fn generate_and_write_latex( & self, filename : & str ) -> Result< (), Box< dyn Error > > {
        println!( "TODO implement ModuleGenerator::generate_and_write_latex {:?}", filename );
        Ok(())
    }

    pub fn generate_and_write_rest( & self, filename : & str ) -> Result< (), Box< dyn Error > > {
        println!( "TODO implement ModuleGenerator::generate_and_write_rest {:?}", filename );
        Ok(())
    }

    pub fn generate_and_write_md( & self, filename : & str ) -> Result< (), Box< dyn Error > > {
        println!( "TODO implement ModuleGenerator::generate_and_write_md {:?}", filename );
        Ok(())
    }
}

//------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_simple_module_description() -> Result< (), Box< dyn Error > > {
        let mut generator = ModuleGenerator::new();
        generator.parse( "tests/simple_module.json" )?;
        Ok(())
    }

    #[test]
    fn parse_avalon_module_description() -> Result< (), Box< dyn Error > > {
        let mut generator = ModuleGenerator::new();
        let _generator = generator.parse( "tests/avalon_module.json" )?;
        Ok(())
    }

    #[test]
    fn missing_input_file() -> Result< (), Box< dyn Error > > {
        let mut generator = ModuleGenerator::new();
        let result = generator.parse( "tests/missing.json" );
        assert!( result.is_err() );
        Ok(())
    }

    #[test]
    fn json_syntax_error() -> Result< (), Box< dyn Error > > {
        let mut generator = ModuleGenerator::new();
        let result = generator.parse( "tests/syntax_error.json" );
        assert!( result.is_err() );
        Ok(())
    }

    #[test]
    fn generate_vhdl_from_simple_module_description() -> Result< (), Box< dyn Error > > {
        let mut generator = ModuleGenerator::new();
        generator.parse( "tests/simple_module.json" )?;
        generator.generate_and_write_vhdl( "tests/simple_module" )?;

        Ok(())
    }
}


