use csv;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PfasData {
    pub id: String,
    pub pfna_level: f64,
    pub pfosa_level: f64,
    pub age: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RiskCategory {
    Low,
    Moderate,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RiskScore {
    pub score: f64,
    pub category: RiskCategory,
}

pub fn calculate_risk_score(data: &PfasData) -> RiskScore {
    // Base risk calculation using PFNA and PFOSA levels
    let pfna_contribution = data.pfna_level * 0.7;
    let pfosa_contribution = data.pfosa_level * 0.3;
    
    // Age factor - higher age increases risk
    let age_factor = (data.age as f64 / 100.0).max(0.0);
    
    // Calculate base risk score
    let mut score = pfna_contribution + pfosa_contribution + age_factor;
    
    // Normalize score to reasonable range
    score = score.min(100.0);
    
    // Classify risk category
    let category = classify_risk_category(score);
    
    RiskScore { score, category }
}

pub fn classify_risk_category(score: f64) -> RiskCategory {
    match score {
        s if s < 10.0 => RiskCategory::Low,
        s if s < 30.0 => RiskCategory::Moderate,
        s if s < 60.0 => RiskCategory::High,
        _ => RiskCategory::VeryHigh,
    }
}