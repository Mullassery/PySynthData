use world_compiler::parser::SchemaParser;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <schema.yaml>", args[0]);
        std::process::exit(1);
    }

    match SchemaParser::from_yaml(&args[1]) {
        Ok(schema) => {
            println!("✓ Schema loaded successfully");
            println!("Entities: {}", schema.entities.len());
            for (name, _) in &schema.entities {
                println!("  - {}", name);
            }
            println!("Relationships: {}", schema.relationships.len());
        }
        Err(e) => {
            eprintln!("✗ Error: {}", e);
            std::process::exit(1);
        }
    }
}
