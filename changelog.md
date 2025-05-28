# ArgSys v0.2.0
## New features
- Added Error struct and it's functions
``` rust
pub struct Error
{
    pub code: i32,
    pub descript: String
}

// Just more simple way to create Error
pub fn build_error( code: i32, descript: &str ) -> Error

// Displays msg if not None and returns err.code
pub fn ret( err: Error, msg: Option<String> ) -> i32

// You can use it like this:
return ret(no_such_arg_errorm, format!("No such arg: {}", args[i]))
/* Will print if args[i] == "--help"(for example):
No such arg: --help
Error -10
*/

```
- Added changelog. *You're reading it right now!\*
- Added library destribution, now it works thus:
``` rust
use argsys::arg::*;
// or
use argsys::err::*;
// or all together
use argsys::{arg::*, err::*};
```