use std::collections::{HashMap, HashSet};
use regex::Regex;
use crate::utils;

pub fn dump_batch_list(items: Vec<String>) {
    let sorted_vec = unified_node_list(items);
    println!("{}", sorted_vec.join(" "));
}

pub fn unified_node_list(items: Vec<String>) -> Vec<String> {
    let groups_map = read_configs(items);

    let mut unified_set: HashSet<String> = HashSet::new();
    for (_, nodes) in groups_map.iter() {
        for node in nodes {
            unified_set.insert(node.to_string());
        }
    }
    let mut sorted_vec: Vec<String> = unified_set.iter().cloned().collect();
    sorted_vec.sort();
    sorted_vec
}

pub fn dump_groups(items: Vec<String>, json: bool) {
    let groups_map = read_configs(items);
    if json {
        let json_string = serde_json::to_string_pretty(&groups_map).unwrap();
        println!("{}", json_string);
    } else {
        for (group_name, nodes) in groups_map.iter() {
            println!("{}: {}", group_name, nodes.join(", "));
        }
    }
}

fn read_configs(items: Vec<String>) -> HashMap<String, Vec<String>> {
    let cfg_files = [
        format!("{}/.hostctl/hostctl.conf", env!("HOME")),
        format!("{}/hostctl.conf", env!("PWD")),
    ];

    let mut select_all = false;
    if items.contains(&"all".to_string()) {
        select_all = true;
    }

    let re = Regex::new(r"^([a-z0-9-]+)\s*:\s*([a-z0-9-,\s]+)(#.*)?").unwrap();

    let mut groups_map = std::collections::HashMap::new();

    for cfg_file in &cfg_files {
        if let Ok(lines) = utils::read_lines(cfg_file) {
            for line in lines {
                if let Ok(host_line) = line {
                    if let Some(captures) = re.captures(&*host_line) {
                        let group_name = captures.get(1).map_or("", |m| m.as_str());
                        let members_str = captures.get(2).map_or("", |m| m.as_str());
                        let nodes = members_str.split(',').map(|s| s.trim().to_string()).collect::<Vec<_>>();

                        if select_all || items.contains(&group_name.to_string()) {
                            let group = groups_map.entry(group_name.to_string()).or_insert(Vec::new());
                            group.extend(nodes);
                        }
                    }
                }
            }
        }
    }
    groups_map
}
