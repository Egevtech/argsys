use Result::Err;

#[derive(Clone)]
pub struct Error
{
    pub code: i32,
    pub descript: String
}

pub fn build_error( code: i32, descript: &str ) -> Error
{
    Error {
        code: code,
        descript: descript.to_string()
    }
}

pub fn ret( err: Error, msg: Option<String> ) -> Result<(), i32>
{
    print!("{}\nError {}", 
            if msg == None { "".to_string() } 
                else { format!("{}{}", msg.unwrap(), "\n") },
            err.code);
 
    Err(err.code)
}