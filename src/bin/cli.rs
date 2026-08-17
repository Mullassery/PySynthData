use pysynthdata::generator::WorldGenerator;
use pysynthdata::parser::SchemaParser;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <schema.yaml> [num_records]", args[0]);
        std::process::exit(1);
    }

    let num_records: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(5);

    match SchemaParser::from_yaml(&args[1]) {
        Ok(schema) => {
            println!("Schema loaded successfully");
            println!("Entities: {}", schema.entities.len());
            for name in schema.entities.keys() {
                println!("  - {}", name);
            }
            println!("Relationships: {}", schema.relationships.len());

            let generator = WorldGenerator::new(schema);
            match generator.generate(num_records, 42) {
                Ok(world) => {
                    let report = generator.evaluate(&world);
                    println!(
                        "\nGenerated {} total rows in {}ms (fidelity_score={:.3}, violations={})",
                        world.metadata.record_count,
                        world.metadata.generation_time_ms,
                        report.fidelity_score,
                        report.total_violations()
                    );
                    if let Ok(json) = world.to_json() {
                        println!("\nSample output:\n{}", json);
                    }
                }
                Err(e) => {
                    eprintln!("Generation error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
