pub enum JobPriority {
    Lowest,
    Low,
    Medium,
    High,
    Highest,
}

impl JobPriority {
    pub fn to_string(&self) -> String {
        let priority = match self {
            Self::Lowest => "lowest",
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::Highest => "highest",
        };
        priority.to_string()
    }
}
