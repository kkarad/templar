use serde::Deserialize;
use serde_json::Value as Json;
use toml::Value as Toml;

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

#[derive(Deserialize, Debug)]
pub struct Template {
    id: String,
    content: Toml,
}

impl Template {
    pub fn print(&self, _ctx: &Context) -> String {
        format!("{}",Template::convert(&self.content))
    }

    fn convert(toml: &Toml) -> Json {
        match toml {
            Toml::String(s) => Json::String(s.to_owned()),
            Toml::Integer(i) => Json::Number((*i).into()),
            Toml::Float(f) => {
                let n = serde_json::Number::from_f64(*f).expect("float infinite and nan not allowed");
                Json::Number(n)
            }
            Toml::Boolean(b) => Json::Bool(*b),
            Toml::Array(arr) => Json::Array(arr.iter().map(Template::convert).collect()),
            Toml::Table(table) => {
                Json::Object(table.into_iter().map(|(k, v)| (k.to_owned(), Template::convert(v))).collect())
            }
            Toml::Datetime(dt) => Json::String(dt.to_string()),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Release {
    pub name: String,
    templates: Vec<Template>
}

impl Release {
    pub fn templates(&self) -> &Vec<Template> {
        &self.templates
    }
}

pub trait Output {
    fn print(&self, templates: &[Template], ctx: &Context) -> Result<(), String>;
}

pub struct Console {}

impl Console {
    pub fn new() -> Console {
        Console {}
    }
}

impl Output for Console {
    fn print(&self, templates: &[Template], ctx: &Context) -> Result<(), String> {
        let mut json = String::from("[");
        for template in templates {
            if json.len() != 1 { json.push_str(", "); }
            json.push_str(&template.print(ctx));
        }
        json.push(']');

        println!("{}", json);
        Ok(())
    }
}
