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
    pub fn print(&self, ctx: &Context) -> String {
        let mut json = Template::convert(&self.content, ctx);
        if let Some(template) = json.as_object_mut() {
            if let Some(jiras) = template.entry("jiras")
                .or_insert_with(|| Json::Array(vec![]))
                .as_array_mut() {
                for jira in &ctx.jiras {
                    jiras.push(Json::String(jira.to_owned()))
                }
            }
            if let Some(wip_jiras) = template.entry("wip-jiras")
                .or_insert_with(|| Json::Array(vec![]))
                .as_array_mut() {
                for jira in &ctx.wip_jiras {
                    wip_jiras.push(Json::String(jira.to_owned()))
                }
            }
        }
        format!("{}", json)
    }

    fn convert(toml: &Toml, ctx: &Context) -> Json {
        match toml {
            Toml::String(s) => Json::String(Template::interpolate(s.to_owned(), ctx)),
            Toml::Integer(i) => Json::Number((*i).into()),
            Toml::Float(f) => {
                let n = serde_json::Number::from_f64(*f).expect("float infinite and nan not allowed");
                Json::Number(n)
            }
            Toml::Boolean(b) => Json::Bool(*b),
            Toml::Array(arr) => Json::Array(arr.iter().map(|v| Template::convert(v, ctx)).collect()),
            Toml::Table(table) => {
                Json::Object(table.into_iter().map(|(k, v)| (k.to_owned(), Template::convert(v, ctx))).collect())
            }
            Toml::Datetime(dt) => Json::String(dt.to_string()),
        }
    }

    fn interpolate(text: String, ctx: &Context) -> String {
        text.replace("{now-version}", ctx.current_version.as_str())
            .replace("{next-version}", ctx.next_version.as_str())
            .replace("{tweet}", ctx.tweet.as_str())
            .replace("{pvt-line-range}", ctx.pvt_line_range.as_str())
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
