type Action = fn(Vec<String>);
type HelpAction = fn(Vec<String>, Vec<Template>);


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

pub fn build_template(template: &str, tshort: Option<&str>, descript: &str, pnum: usize) -> Template {
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

#[derive(Clone)]
struct TrigAction
{
    pub trigger: Template,
    pub action: Action
}

pub struct ArgHandler {
    templates: Vec<Template>,
    trigs_and_actions: Vec<TrigAction>,

    help_template: Option<Template>,
    help_action: Option<HelpAction>,

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
            skip_first: false,

            help_template: None,
            help_action: None
        }
    }
}

impl ArgHandler {

    pub fn add_action(&mut self, template: Template, action: Action) {
        self.templates.push(template.clone());
        self.trigs_and_actions.push(TrigAction {trigger: template, action: action});
    }

    pub fn add_help_action(&mut self, help_template: Template, help_action: HelpAction) {
        self.templates.push(help_template.clone());
        
        self.help_template = Some(help_template);
        self.help_action = Some(help_action);       
    }

    pub fn init(&mut self, args: Vec<String>, skip_first: bool)
    {
        self.args = args.clone();
        self.skip_first = skip_first;
    }

    pub fn run(&mut self) {

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

                if self.help_template == None || self.help_action == None {
                    break;
                }

                if self.help_template.clone().unwrap().template == arg ||
                    self.help_template.clone().unwrap().short == Some(arg.clone()) {
                        let mut param_vec: Vec<String> = vec!();
                        
                        for _ in arg_number..arg_number + self.help_template.clone().unwrap().param_num {
                            arg_number += 1;
                            param_vec.push(self.args[arg_number].clone());
                        }

                        (self.help_action.unwrap())(param_vec, self.templates.clone());
                    }
                
            }
        } 
    }
}
