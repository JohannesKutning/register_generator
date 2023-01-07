use std::error::Error;

pub struct Generator {
    filename : String
}

impl Generator {
    pub fn new( filename : & str ) -> Generator {
        Generator { filename : filename.to_string() }
    }

    pub fn parse( & mut self ) -> Result< (), Box< dyn Error > > {
        println!( "Generator::parse {:?}", self.filename );

        Ok(())
    }

    pub fn generate_and_write_vhdl( & self, filename : & str ) -> Result< (), Box< dyn Error > > {
        println!( "TODO implement Generator::generate_and_write_vhdl {:?}", filename );

        Ok(())
    }

    pub fn generate_and_write_c( & self, filename : & str ) -> Result< (), Box< dyn Error > > {
        println!( "TODO implement Generator::generate_and_write_c {:?}", filename );

        Ok(())
    }

    pub fn generate_and_write_cpp( & self, filename : & str ) -> Result< (), Box< dyn Error > > {
        println!( "TODO implement Generator::generate_and_write_cpp {:?}", filename );

        Ok(())
    }

    pub fn generate_and_write_latex( & self, filename : & str ) -> Result< (), Box< dyn Error > > {
        println!( "TODO implement Generator::generate_and_write_latex {:?}", filename );

        Ok(())
    }

    pub fn generate_and_write_rest( & self, filename : & str ) -> Result< (), Box< dyn Error > > {
        println!( "TODO implement Generator::generate_and_write_rest {:?}", filename );

        Ok(())
    }

    pub fn generate_and_write_md( & self, filename : & str ) -> Result< (), Box< dyn Error > > {
        println!( "TODO implement Generator::generate_and_write_md {:?}", filename );

        Ok(())
    }
}
