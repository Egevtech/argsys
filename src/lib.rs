pub struct Arg 
{
    pub template: String,
    pub short: Option<String>,
    pub descript: String
}

pub fn build_arg( template: &str, short: Option<&str>, descript: &str ) -> Arg 
{
    Arg {

        template: template.to_string(),
        short: if short == None { None } else { Some(short.unwrap().to_string()) },
        descript: descript.to_string()

    }
}