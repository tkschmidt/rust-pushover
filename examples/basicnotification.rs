extern crate pushover;

use pushover::message::Message;

fn main()
{
    let token = String::from_str("aYAtEHZm1obaiV8MbXFDRpW44jRUxx");
    let user = String::from_str("uVED84FSEzbcgHhxQgoQXFZufC1idQ");
    let msg = String::from_str("This is a test message");

    let message = Message{
        message: msg,
        user: user,
        token: token,
        title: None,
        device: None,
        priority: None,
        retry: None,
        expire: None,
        callback: None};

    let x = pushover::simple_send_message(message);

    if x
    {
        println!("{}", x);
    }
    else
    {
        println!("{}", x);
    }
}
