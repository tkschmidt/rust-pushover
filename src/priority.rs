
pub enum Priority
{
    Lowest = -2,
    Low = -1,
    Normal = 0,
    High = 1,
    Emergency = 2,
}

impl Priority {
    pub fn stringify(&self) -> &'static str {
        return match *self{
            Priority::Lowest => "-2",
            Priority::Low => "-1",
            Priority::Normal => "0",
            Priority::High => "1",
            Priority::Emergency => "2",
        };
    }
}

