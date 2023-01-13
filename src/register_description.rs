
#[derive(Debug)]
pub struct RegisterDescription {
    name        : String,
    brief       : String,
    details     : String,
    offset      : u64,
    size        : u64,
    fields      : Vec< Field >,
}

