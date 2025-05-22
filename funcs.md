# ArgSys v0.0.3-beta

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