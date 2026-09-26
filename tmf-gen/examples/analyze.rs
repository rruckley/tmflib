//! Diagnostic: report dangling and unusual $ref targets in a spec's
//! components.schemas, plus schema constructs typify may struggle with.
use std::collections::BTreeSet;

use tmf_gen::load_spec;

fn walk(value: &serde_json::Value, refs: &mut Vec<String>, stats: &mut Stats) {
    match value {
        serde_json::Value::Object(map) => {
            for (key, val) in map {
                match key.as_str() {
                    "$ref" => {
                        if let serde_json::Value::String(s) = val {
                            refs.push(s.clone());
                        }
                    }
                    "allOf" => stats.all_of += 1,
                    "anyOf" => stats.any_of += 1,
                    "oneOf" => stats.one_of += 1,
                    "enum" => stats.enum_ += 1,
                    "additionalProperties" => {
                        if !val.is_boolean() {
                            stats.additional_props += 1;
                        }
                    }
                    _ => {}
                }
                walk(val, refs, stats);
            }
        }
        serde_json::Value::Array(items) => {
            for item in items {
                walk(item, refs, stats);
            }
        }
        _ => {}
    }
}

#[derive(Default)]
struct Stats {
    all_of: u32,
    any_of: u32,
    one_of: u32,
    enum_: u32,
    additional_props: u32,
}

fn main() {
    let path = std::env::args().nth(1).expect("usage: analyze <spec>");
    let openapi = load_spec(std::path::Path::new(&path)).expect("parse failed");
    let schemas = &openapi.schemas;
    let names: BTreeSet<String> = schemas.keys().cloned().collect();

    let value = serde_json::to_value(schemas).unwrap();
    let mut refs = Vec::new();
    let mut stats = Stats::default();
    walk(&value, &mut refs, &mut stats);

    let mut missing: BTreeSet<String> = BTreeSet::new();
    let mut unusual: BTreeSet<String> = BTreeSet::new();
    for r in &refs {
        let tail = r.rsplit('/').next().unwrap_or(r);
        if !names.contains(tail) {
            missing.insert(r.clone());
        }
        if !r.starts_with("#/components/schemas/") {
            unusual.insert(r.clone());
        }
    }

    println!("schemas: {}", names.len());
    println!(
        "constructs: allOf={} anyOf={} oneOf={} enum={} additionalProps={}",
        stats.all_of, stats.any_of, stats.one_of, stats.enum_, stats.additional_props
    );
    println!("total refs: {}", refs.len());
    println!("\n-- refs with missing targets ({}):", missing.len());
    for r in &missing {
        println!("  {r}");
    }
    println!("\n-- refs not under #/components/schemas ({}):", unusual.len());
    for r in unusual.iter().take(20) {
        println!("  {r}");
    }
}
