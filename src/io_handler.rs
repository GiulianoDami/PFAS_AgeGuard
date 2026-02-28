use csv;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct PFASData {
    pub sample_id: String,
    pub pfna_concentration: f64,
    pub pfosa_concentration: f64,
    pub age: u32,
    pub gender: String,
}

#[derive(Debug, Serialize)]
pub struct ReportData {
    pub sample_id: String,
    pub aging_risk_score: f64,
    pub pfna_level: f64,
    pub pfosa_level: f64,
}

pub fn read_csv(file_path: &str) -> Result<Vec<PFASData>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let mut reader = csv::Reader::from_reader(file);
    let mut records = Vec::new();

    for result in reader.deserialize() {
        let record: PFASData = result?;
        records.push(record);
    }

    Ok(records)
}

pub fn write_report(
    data: &[ReportData],
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(output_path)?;
    let mut writer = csv::Writer::from_writer(BufWriter::new(file));

    for record in data {
        writer.serialize(record)?;
    }

    writer.flush()?;
    Ok(())
}

pub fn validate_input(data: &[PFASData]) -> bool {
    for record in data {
        if record.pfna_concentration < 0.0 || record.pfosa_concentration < 0.0 {
            return false;
        }
        if record.age == 0 {
            return false;
        }
    }
    true
}