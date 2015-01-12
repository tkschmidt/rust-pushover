extern crate http;
extern crate url;
use self::url::Url;
use priority::Priority;

pub struct Message
{
    pub token: String,
    pub user: String,
    pub message: String,
    pub title: Option<String>,
    pub device: Option<String>,

    pub priority: Option<Priority>,
    pub retry: Option<i32>,
    pub expire: Option<i32>,
    pub callback: Option<Url>,
}

