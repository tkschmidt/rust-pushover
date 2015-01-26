#![crate_type="lib"]
#![crate_name="pushover"]

//extern crate http;
//extern crate url;

//use http::client::RequestWriter;
//use http::method::Post;
//use url::Url;

pub mod priority;
pub mod message;

pub fn simple_send_message(message: message::Message) -> bool
{
//    let post_data = generate_request_data(message);
//    let url = Url::parse("https://api.pushover.net/a/messages.json").unwrap();

//    let mut request: RequestWriter = match RequestWriter::new(Post, url) {
//    Ok(request) => request,
//    _ => return false,
//    };

//    request.headers.content_length = Some(post_data.len());
//    let _ = request.write(post_data.as_bytes());

//    let _ = match request.read_response() {
//        Ok(response) => response,
//        _ => return false,
//    };

    return true;
}

//pub fn send_message(message: message::Message) ->

fn generate_request_part(key: &str, value: &str) -> String
{
    return format!("&{key}={value}", key=key, value=value);
}

fn generate_request_data(message: message::Message) -> String
{
    let mut data = format!("token={tok}&user={usr}&message={msg}",
                          tok = message.token,
                           usr = message.user,
                           msg = message.message);

    match message.title {
        Some(value) =>
        {
            let title = generate_request_part("title", value);
            data.push_str(title.as_slice());
        },
        None => (),
    };

    match message.device {
        Some(value) =>
        {
            let device = generate_request_part("device", value);
            data.push_str(device.as_slice());
        },
        None => (),
    };

    match message.priority {
        Some(value) =>
        {
            let priority = generate_request_part("priority", value.stringify());
            data.push_str(priority.as_slice());
        },
        None => (),
    };

    match message.retry {
        Some(value) =>
        {
            let retry = generate_request_part("retry", value.to_string().as_slice());
            data.push_str(retry.as_slice());
        },
        None => (),
    };

    match message.expire {
        Some(value) =>
        {
            let expire = generate_request_part("expire", value.to_string().as_slice());
            data.push_str(expire.as_slice());
        },
        None => (),
    };

    match message.callback {
        Some(_) => (),
        None => (),

    };
    return data;
}

#[cfg(test)]
mod test {
    use super::generate_request_part;
    use super::generate_request_data;
    use super::message;
    use super::priority;

#[test]
    fn test_generate_request_part()
    {
        let key = "key";
        let value = "value";
        let result = generate_request_part(key, value);
        assert_eq!("&key=value", result.as_slice())
    }

#[test]
    fn test_generate_minimal_message()
    {
        let test_message = message::Message{
            message: "message_val",
            user: "user_val",
            token: "token_val",
            title: None,
            device: None,
            priority: None,
            retry: None,
            expire: None,
            callback: None};

        let data = generate_request_data(test_message);

        assert_eq!("token=token_val&user=user_val&message=message_val", data.as_slice());
    }

#[test]
    fn test_generate_full_message()
    {
        let test_message = message::Message{
            message: "message_val",
            user: "user_val",
            token: "token_val",
            title: Some("title_val"),
            device: Some("device_val"),
            priority: Some(priority::Priority::Normal),
            retry: Some(30),
            expire: Some(300),
            callback: None};

        let data = generate_request_data(test_message);

        assert_eq!("token=token_val&user=user_val&message=message_val&title=title_val&device=device_val&priority=0&retry=30&expire=300", data.as_slice());
    }
}
