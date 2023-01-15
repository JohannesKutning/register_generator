use std::error::Error;
use clap::{arg, command, Command};

use crate::module_generator::ModuleGenerator;

mod module_generator;
mod module_description;
mod register_description;
mod vhdl_generator;

fn cli() -> Command {
    command!()
        .arg(arg!( -i --input <FILE> "A register description .json file." ).required( true ) )
        .arg(arg!( --vhdl <FILE> "A VHDL output file for register unit hardware generation." ) )
        .arg(arg!( --c <FILE> "A C programing language output file for register unit defines." ) )
        .arg(arg!( --cpp <FILE> "A C++ programing language output file for register unit defines." ) )
        .arg(arg!( --latex <FILE> "A LaTex output file containting the register unit description." ) )
        .arg(arg!( --rest <FILE> "A reStructuredText output file containting the register unit description." ) )
        .arg(arg!( --md <FILE> "A markdown output file containting the register unit description." ) )
}

fn main() -> Result< (), Box< dyn Error > >{
    let matches = cli().get_matches();
    let input = matches.get_one::< String >( "input" ).expect( "required" );
    let mut generator = ModuleGenerator::new();
    let generator : ModuleGenerator = match generator.parse( & input ) {
        Ok(()) => generator,
        Err( error )  => {
            println!( "{:?}", error );
            return Err( error );
        }
    };

    if let Some( output ) =  matches.get_one::< String >( "vhdl" ) {
        generator.generate_and_write_vhdl( output )?;
    }

    if let Some( output ) =  matches.get_one::< String >( "c" ) {
        generator.generate_and_write_c( output )?;
    }

    if let Some( output ) =  matches.get_one::< String >( "cpp" ) {
        generator.generate_and_write_cpp( output )?;
    }

    if let Some( output ) =  matches.get_one::< String >( "latex" ) {
        generator.generate_and_write_latex( output )?;
    }

    if let Some( output ) =  matches.get_one::< String >( "rest" ) {
        generator.generate_and_write_rest( output )?;
    }

    if let Some( output ) =  matches.get_one::< String >( "md" ) {
        generator.generate_and_write_md( output )?;
    }

    Ok(())
}

