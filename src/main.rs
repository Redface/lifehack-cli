use std::io::stdin;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct JokeResponse {
    setup: String,
    delivery: String,
}

#[derive(Deserialize, Debug)]
struct ActivityResponse {
    activity: String,
}

async fn call_api<T>(url: String) -> reqwest::Result<T>
    where T: serde::de::DeserializeOwned
{
    let response = reqwest::get(url).await;
    let result = match response {
        Ok(data) => data.json::<T>().await,
        Err(err) => Err(err)
    };
    result
}

async fn call_joke() -> reqwest::Result<JokeResponse> {
    call_api(String::from("https://v2.jokeapi.dev/joke/Any")).await
}

async fn call_activity() -> reqwest::Result<ActivityResponse> {
    call_api(String::from("https://www.boredapi.com/api/activity")).await
}

fn display_menu() {
    println!("Menu: ");
    println!("1. Random daily joke ");
    println!("2. Activity recommendation ");
    println!("q: Quit");
}

fn handle_ok_match<T>(result: reqwest::Result<T>, cb: fn(T)) {
    match result {
        Ok(data) => {
            cb(data);
        }
        Err(err) => {
            println!("Api error: {}", err);
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut menu_buffer = String::new();
    loop {
        match menu_buffer.trim().to_lowercase().as_str() {
            "1" => {
                let result = call_joke().await;
                let ok_cb = |data: JokeResponse| {
                    println!("{}", data.setup);
                    println!("{}", data.delivery);
                };
                handle_ok_match(result, ok_cb);
            }
            "2" => {
                let result = call_activity().await;
                let ok_cb = |data: ActivityResponse| {
                    println!("{}", data.activity);
                };
                handle_ok_match(result, ok_cb);
            }
            "q" => {
                println!("Bye!");
                break;
            }
            _ => {}
        }
        menu_buffer.clear();
        display_menu();

        stdin().read_line(&mut menu_buffer).expect("read line err");
    }
    Ok(())
}
