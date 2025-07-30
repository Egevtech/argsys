// Type aliasing
type Action = fn(Vec<String>);


// Template struct
#[derive(Clone, PartialEq)]
pub struct Template
{
    pub template: String,
    pub short: Option<String>,
    pub descript: String,
    pub param_num: usize
}

impl std::fmt::Display for Template
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}({}) <{}> - {}", self.template.clone(),
                if self.short == None { "None".to_string() } else { self.short.clone().unwrap() },
                self.param_num, self.descript.clone())
    }
}

pub fn build_template(template: &str, tshort: Option<&str>, descript: &str, pnum: usize) -> Template { // Just more simple way to create Template structure object
    Template {
        template: template.to_string(),
        short: match tshort {
            Some(s) => Some(s.to_string()),
            None => None
        },
        descript: descript.to_string(),
        param_num: pnum
    }
}


// TrigAction struct
#[derive(Clone)]
struct TrigAction
{
    pub trigger: Template,
    pub action: Action
}


// ArgHandler struct
pub struct ArgHandler {
    templates: Vec<Template>,
    trigs_and_actions: Vec<TrigAction>,

    args: Vec<String>,
    skip_first: bool
}

impl Default for ArgHandler {
    fn default() -> Self
    {
        ArgHandler {
            templates: vec!(),
            trigs_and_actions: vec!(),
            args: vec!(),
            skip_first: false
        }
    }
}

impl ArgHandler {

    pub fn add_template(&mut self, template: Template) { // Add template to templates list
        self.templates.push(template);
    }

    pub fn init(&mut self, args: Vec<String>, skip_first: bool) // Init arghandler by arguments
    {
        self.args = args.clone();
        self.skip_first = skip_first;
    }

    pub fn add_trigger(&mut self, trigger: Template, action: Action) // Add trigger and action for this trigger
    {
        self.trigs_and_actions.push(TrigAction{trigger, action});
    }

    pub fn run(&mut self) { // Run handler

        for mut arg_number in self.skip_first as usize..self.args.len() {
            let arg: String = self.args[arg_number].clone();

            for template in self.templates.clone () {
                if arg == template.template || Some(arg.clone()) == template.short {
                    for trigact in self.trigs_and_actions.clone() {
                        if trigact.trigger == template  {
                            let mut param_vec: Vec<String> = vec!();
                            for _ in arg_number..arg_number+template.param_num.clone() {
                                arg_number += 1;
                                param_vec.push(self.args[arg_number].clone());
                            }

                            (trigact.action)(param_vec);
                        }
                    }
                }
            }
        } 
    }
}
