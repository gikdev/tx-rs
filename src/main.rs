mod case_converter;
mod component;
mod route;
mod structs;
mod templates;

use crate::structs::*;
use std::env;

const AVAILABLE_ACTIONS: [&str; 2] = ["create", "c"];
const AVAILABLE_KINDS: [&str; 4] = ["component", "c", "route", "r"];

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args).expect("Sth happened...");

    let is_action_available = AVAILABLE_ACTIONS.contains(&config.action.as_str());
    let is_kind_available = AVAILABLE_KINDS.contains(&config.kind.as_str());
    if !is_action_available || !is_kind_available {
        println!("Not supported action / kind");
        return;
    }

    match config.kind.as_str() {
        "component" | "c" => match config.action.as_str() {
            "create" | "c" => {
                component::create(&config.name).expect("Failed to create the file...")
            }
            _ => return,
        },
        "route" | "r" => match config.action.as_str() {
            "create" | "c" => route::create(&config.name).expect("Failed to create the file..."),
            _ => return,
        },
        _ => return,
    }

    println!(
        "Done `{}` of {} `{}`",
        config.action, config.kind, config.name
    );
}

fn parse_config(args: &[String]) -> Result<Config, &'static str> {
    let is_enough_amount_of_args = args.len() >= 4;
    if !is_enough_amount_of_args {
        return Err("Provide at least 3 args!");
    }

    let action = args[1].clone();
    let kind = args[2].clone();
    let name = args[3].clone();

    let is_action_available = AVAILABLE_ACTIONS.contains(&action.as_str());
    if !is_action_available {
        return Err("We currently only support the `create` command!");
    }

    let is_kind_available = AVAILABLE_KINDS.contains(&kind.as_str());
    if !is_kind_available {
        return Err("We currently only support `components` & `routes`!");
    }

    let config = Config { action, kind, name };
    Ok(config)
}
