use std::process::Command;

#[allow(dead_code)]
pub fn get_xresource(resource_name: &str) -> Option<String> {
    let output = Command::new("xgetres").arg(resource_name).output().ok()?;
    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

#[allow(dead_code)]
pub fn parse_hex_color(color: &str) -> Option<u32> {
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

#[allow(unused_macros)]
#[macro_export]
macro_rules! xcolor {
    ($name:expr) => {
        let resource = $crate::misc::get_xresource($name)
            .expect(&format!("Failed to get X resource: {:?}", $name));

        $crate::misc::parse_hex_color(resource).expect(&format!(
            "Failed to parse color from value {:?} of X resource {:?}",
            resource, $name
        ))
    };
    ($name:expr, $fallback:expr) => {
        match $crate::misc::get_xresource($name) {
            Some(resource) => $crate::misc::parse_hex_color(&resource).unwrap_or_else(|| {
                eprintln!(
                    "Using fallback value {:?} for X resource {:?} (invalid color: {:?})",
                    $fallback, $name, resource
                );

                $crate::misc::parse_hex_color($fallback).expect(&format!(
                    "Failed to parse color from fallback value {:?}",
                    $fallback
                ))
            }),
            None => $crate::misc::parse_hex_color($fallback).expect(&format!(
                "Failed to parse color from fallback value {:?}",
                $fallback
            )),
        }
    };
}
