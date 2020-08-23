use std::process::Command;

fn get_x_resource(resource_name: &str) -> Option<String> {
    let output = match Command::new("xgetres").arg(resource_name).output() {
        Ok(o) => o,
        Err(e) => return None,
    };

    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn parse_hex_color(color: &str) -> Option<u32> {
    match color.len() {
        7 => {
            if color.chars().nth(0).unwrap() == '#' {
                u32::from_str_radix(&color[1..], 16).ok()
            } else {
                None
            }
        }
        6 => u32::from_str_radix(color, 16).ok(),
        _ => None,
    }
}

macro_rules! get_x_color {
    ($name:expr, $fallback:expr) => {
        parse_hex_color(&get_x_resource($name).unwrap_or($fallback.to_string()))
            .unwrap_or_else(|| parse_hex_color($fallback).unwrap())
    };
}
