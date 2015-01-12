
pub enum Priority
{
    Lowest = -2,
    Low = -1,
    Normal = 0,
    High = 1,
    Emergency = 2,
}

impl Priority {
    pub fn stringify(&self) -> String {
        return match *self{
            Priority::Lowest => "-2".to_string(),
            Priority::Low => "-1".to_string(),
            Priority::Normal => "0".to_string(),
            Priority::High => "1".to_string(),
            Priority::Emergency => "2".to_string(),
        };
    }
}

