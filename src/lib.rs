#[derive(Clone)]
pub struct Arg 
{
    pub template: String,
    pub short: Option<String>,
    pub descript: String,
    pub param_num: usize
}

pub fn build_arg( template: &str, short: Option<&str>, descript: &str, param_num: usize ) -> Arg 
{
    Arg {

        template: template.to_string(),
        short: if short == None { None } else { Some(short.unwrap().to_string()) },
        descript: descript.to_string(),
        param_num: param_num
    }
}

pub fn get_arg_index( args: Vec<Arg>, template: String ) -> Option<usize>
{

    for i in  0..args.len() {
        let current = args[i].clone();
        if [current.template.clone(),
            if current.short.clone() == None { "".to_string() } else
                { current.short.unwrap().clone() }].contains(&template.clone()) {
            return Some(i);
        }
    }


    None
}

pub fn get_arg_description( args: Vec<Arg>, index: usize) -> String
{
    args[index].descript.clone()
}
