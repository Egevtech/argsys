# ArgSys 0.0.3-beta

- github - [here](https://github.com/egevtech/argsys)
- crates.io - [here](https://crates.io/crates/argsys)

At first, you need to create templates:
``` rust
let mut templates: Vec<Arg> = Vec::new();

templates.push( build_arg("--help", Some("-h"), "Displays help") ); // example
// Trigger will be on --help or on -h

templates.push( build_arg("--list", None, "Displays configurations") ); // example
// Trigger will be on --list only
```
Next, you can get template index in array:
``` rust

let arg_index: usize = 
    get_arg_index( templates.clone(), "--help".to_string() );

```

or:

``` rust

use std::env;

let args: Vec<String> = env::args().collect();

for i in 1..args.len() { // Skip first argument
    let current: String = args[i].clone();

    match get_arg_index( templates.clone(), current.clone() ) {

        Some(0) => {} // --help
        Some(1) => {} // --list

        None => {}
        _ => {}
    }
}

```
Description you can get by special function:

```rust
get_arg_description( templates, 0 );
```
or
```rust
for i in 0..args.len() {
    get_arg_description( templates, i );
}
```

And full example:
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
Also try [new version of this README](funcs.md)

###### ArgSys help - v0.0.1-0.0.2