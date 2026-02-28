use csv;
use serde::{Deserialize, Serialize};
use std::env;
use std::process;

#[derive(Debug, Deserialize)]
struct PFASData {
    sample_id: String,
    pfna_concentration: f64,
    pfosa_concentration: f64,
    age: u32,
    gender: String,
}

#[derive(Debug, Serialize)]
struct AgingRiskReport {
    sample_id: String,
    risk_score: f64,
    recommendation: String,
}

fn parse_args() -> Result<String, &'static str> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err("Usage: PFAS_AgeGuard <input.csv>");
    }
    Ok(args[1].clone())
}

fn calculate_risk_score(data: &PFASData) -> f64 {
    let pfna_score = data.pfna_concentration * 0.5;
    let pfosa_score = data.pfosa_concentration * 0.7;
    let age_factor = (data.age as f64) / 100.0;
    pfna_score + pfosa_score + age_factor
}

fn generate_recommendation(risk_score: f64) -> String {
    match risk_score {
        s if s > 5.0 => "High risk detected. Consult a healthcare professional.".to_string(),
        s if s > 3.0 => "Moderate risk. Consider reducing exposure sources.".to_string(),
        _ => "Low risk. Maintain current exposure levels.".to_string(),
    }
}

fn process_file(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = csv::Reader::from_path(filename)?;
    let mut report_writer = csv::Writer::from_writer(std::io::stdout());

    for result in reader.deserialize() {
        let record: PFASData = result?;
        let risk_score = calculate_risk_score(&record);
        let recommendation = generate_recommendation(risk_score);
        
        let report = AgingRiskReport {
            sample_id: record.sample_id.clone(),
            risk_score,
            recommendation,
        };
        
        report_writer.serialize(report)?;
    }

    report_writer.flush()?;
    Ok(())
}

fn main() {
    let filename = match parse_args() {
        Ok(name) => name,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = process_file(&filename) {
        eprintln!("Error processing file: {}", e);
        process::exit(1);
    }
}