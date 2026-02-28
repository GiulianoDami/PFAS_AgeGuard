use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PfasSample {
    pub id: String,
    pub sample_type: String,
    pub pfas_compound: String,
    pub concentration: f64,
    pub collection_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskReport {
    pub sample_id: String,
    pub risk_category: RiskCategory,
    pub risk_score: f64,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskCategory {
    Low,
    Medium,
    High,
}