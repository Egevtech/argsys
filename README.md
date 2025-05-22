# ArgSys v0.1.0

Simple app arguments handler library

## Links
- [github](https://github.com/egevtech/argsys)
- [crates.io](https://crates.io/crates/argsys)


### Functions description:
``` rust
pub struct Arg 
{
    pub template: String,
    pub short: Option<String>,
    pub descript: String
}

// Just more simple way to create Arg
pub fn build_arg( template: &str, short: Option<&str>, descript: &str ) -> Arg;

// Returns arg index in array or None if not exists
pub fn get_arg_index( args: Vec<Arg>, template: String ) -> Option<usize>;

// Returns 'descript' field of Arg by index
pub fn get_arg_description( args: Vec<Arg>, index: usize) -> String;
```

### Simple example:
``` rust
use argsys::*;
use std::env;

fn main() 
{
    // Create an array of templates
    let mut templates: Vec<Arg> = Vec::new();
    
    // Create templates
    templates.push( build_arg( "--help", Some("-h"), "Displays help" ) );
    templates.push( build_arg( "--list", None, "Displays configurations") );

    // Push args in array
    let args: Vec<String> = env::args().collect();

    for i in 1..args.len() {
        let current: String = args[i],clone();

        // Returns index of current in array or 'None' if not found
        match get_arg_index( templates.clone(), current.clone() ) {
            Some(0) => { println!("ArgSys help\n..."); }

            None => { println!("Not found: {}", current.clone()); }
            _ => {} 
        }
    }
}
```

I'll be grateful for the star on [github](https://github.com/egevtech/argsys)