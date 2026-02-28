use csv;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PfasData {
    pub sample_id: String,
    pub pfas_compound: String,
    pub concentration: f64,
    pub age: u32,
    pub gender: String,
}

#[derive(Debug, Serialize)]
pub struct AgingRiskScore {
    pub sample_id: String,
    pub risk_score: f64,
    pub risk_level: String,
    pub compound_specific_score: f64,
}

#[derive(Debug)]
pub struct Analyzer {
    pub data: Vec<PfasData>,
}

impl Analyzer {
    pub fn new() -> Self {
        Analyzer {
            data: Vec::new(),
        }
    }

    pub fn load_from_csv(&mut self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut reader = csv::Reader::from_path(file_path)?;
        
        for result in reader.deserialize() {
            let record: PfasData = result?;
            self.data.push(record);
        }
        
        Ok(())
    }

    fn calculate_risk_score(&self, data: &PfasData) -> AgingRiskScore {
        // Base risk calculation based on concentration and age
        let base_score = data.concentration * 0.5 + (data.age as f64 * 0.1);
        
        // Compound-specific multipliers
        let compound_multiplier = match data.pfas_compound.as_str() {
            "PFNA" => 1.5,
            "PFOSA" => 1.3,
            "PFHxS" => 1.1,
            _ => 1.0,
        };
        
        let compound_specific_score = base_score * compound_multiplier;
        
        // Risk level classification
        let risk_level = if compound_specific_score >= 8.0 {
            "High".to_string()
        } else if compound_specific_score >= 4.0 {
            "Medium".to_string()
        } else {
            "Low".to_string()
        };
        
        AgingRiskScore {
            sample_id: data.sample_id.clone(),
            risk_score: compound_specific_score,
            risk_level,
            compound_specific_score,
        }
    }

    pub fn analyze_data(&self) -> Vec<AgingRiskScore> {
        self.data
            .iter()
            .map(|record| self.calculate_risk_score(record))
            .collect()
    }
}

pub fn new_analyzer() -> Analyzer {
    Analyzer::new()
}

pub fn analyze_data(analyzer: &Analyzer) -> Vec<AgingRiskScore> {
    analyzer.analyze_data()
}