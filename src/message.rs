use priority::Priority;

pub struct Message<'m>
{
    pub token: &'m str,
    pub user: &'m str,
    pub message: &'m str,
    pub title: Option<&'m str>,
    pub device: Option<&'m str>,

    pub priority: Option<Priority>,
    pub retry: Option<i32>,
    pub expire: Option<i32>,
    pub callback: Option<&'m str>,
}

