
pub enum Priority
{
    Lowest = -2,
    Low = -1,
    Normal = 0,
    High = 1,
    Emergency = 2,
}

pub trait StringVersion {
    fn stringify(&self) -> String;
}

impl Priority {
    pub fn stringify(&self) -> String {
        return match *self{
            Lowest => "-2".to_string(),
            Low => "-1".to_string(),
            Normal => "0".to_string(),
            High => "1".to_string(),
            Emergency => "2".to_string(),
        };
    }
}

