extern crate dotenv;

use serde::Deserialize;
use dotenv::dotenv;

#[derive(Deserialize, Debug)]
struct User {
    id: i32,
    can_access_closed: bool,
    is_closed: bool,
    can_invite_to_chats: bool,
    first_name: String,
    last_name: String,
    #[serde(default)]
    bdate: String
}

#[derive(Deserialize, Debug)]
struct Answer {
    response: Vec<User>
}

struct VkRequest {
    token: String,
    method: String,
    params: Vec<(String, String)>
}

impl VkRequest {
    fn new(token: String, method: String, params: Vec<(String, String)>) -> VkRequest {
        VkRequest{token, method, params}
    }

    async fn run<T: for<'de> Deserialize<'de>>(self) -> Result<T, reqwest::Error> {
        let params_string: String = self.params
                                        .iter()
                                        .map(|(name, value)| format!("{}={}", name, value))
                                        .collect::<Vec<String>>()
                                        .join("&");
                                        
        let request = format!("https://api.vk.com/method/{}?{}&access_token={}&v=5.131", self.method, params_string, self.token);

        Ok(reqwest::get(request)
        .await?
        .json::<T>()
        .await?)
    }
}

// https://api.vk.com/method/METHOD?PARAMS&access_token=TOKEN&v=V

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    dotenv().ok();

    let vk_token = dotenv::var("VK_TOKEN").unwrap();
    
    let a = VkRequest::new(vk_token, String::from("users.get"), vec!((String::from("user_ids"), String::from("1")), (String::from("fields"), String::from("bdate"))));

    let b = a.run::<Answer>().await?;

    println!("{:?}", b);

    Ok(())
}
