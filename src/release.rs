#[derive(Debug)]
pub struct Context {
    pub name: String,
    pub current_version: String,
    pub next_version: String,
    pub tweet: String,
    pub pvt_line_range: String,
    pub jiras: Vec<String>,
    pub wip_jiras: Vec<String>,
    _secret: (),
}

impl Context {
    pub fn new(name: String,
               current_version: String,
               next_version: String,
               tweet: String,
               pvt_line_range: String,
               jiras: Vec<String>,
               wip_jiras: Vec<String>) -> Context {
        Context { name, current_version, next_version, tweet, pvt_line_range, jiras, wip_jiras,
            _secret: () }
    }
}

pub struct Template {}

pub struct Release {
    name: String,
}

impl Release {
    pub fn new(name: &str) -> Release {
        Release { name: name.to_string() }
    }

    pub fn templates(&self) -> Vec<Template> {
        vec![]
    }
}

pub trait Output {
    fn print(&self, templates: Vec<Template>, ctx: Context) -> Result<(), String>;
}

pub struct Console {}

impl Console {
    pub fn new() -> Console {
        Console {}
    }
}

impl Output for Console {
    fn print(&self, templates: Vec<Template>, ctx: Context) -> Result<(), String> {
//        for template in templates {
//            template.print(ctx);
//        }
        Ok(())
    }
}
