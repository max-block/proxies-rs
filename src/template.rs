use std::collections::HashMap;

use actix_web::{error::ErrorConflict, web::Data, HttpResponse};
use chrono::DateTime;
use tera::{Context, Error as TeraError, Tera, Value};

use crate::{config::AppConfig, error::AppError};

pub type HttpResponseResult = Result<HttpResponse, AppError>;

pub fn render(tpl: Data<Tera>, name: &str, ctx: &Context) -> HttpResponseResult {
    let s = tpl.render(name, ctx).map_err(ErrorConflict)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

fn dt_filter(value: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    dbg!(&value);
    if value.is_string() {
        println!("z1");

        if let Ok(date) = DateTime::parse_from_rfc3339(value.as_str().unwrap()) {
            dbg!(&date);
            dbg!(Value::String(date.format("%Y-%m-%d %H:%M:%S").to_string()));
            //return Ok(Value::String(date.format("%Y-%m-%d %H:%M:%S").to_string()));
            println!("z2");
            return Ok(Value::Bool(true));
            println!("z3");
        }
        println!("z4");
    }  
    println!("z5");
    Ok(Value::String("Z".to_string()))
}

fn app_name_function(_: &HashMap<String, Value>) -> tera::Result<Value> {
    let config = AppConfig::get();
    Ok(Value::String(config.app_name))
}

fn app_version_function(_: &HashMap<String, Value>) -> tera::Result<Value> {
    let config = AppConfig::get();
    Ok(Value::String(config.cargo_pkg_version))
}

pub fn init_tera() -> anyhow::Result<Tera> {
    let mut tera = Tera::new("templates/**/*.html")?;
    tera.register_function("app_name", app_name_function);
    tera.register_function("app_version", app_version_function);
    tera.register_filter("dt", dt_filter);
    Ok(tera)
}
