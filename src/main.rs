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

#[derive(Deserialize, Debug)]
struct ChuckNorrisResponse {
    value: String,
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
    call_api(String::from("https://bored-api.appbrewery.com/random")).await
}

async fn call_chuck_norris_fact() -> reqwest::Result<ChuckNorrisResponse> {
    call_api(String::from("https://api.chucknorris.io/jokes/random")).await
}

fn display_menu() {
    println!("Menu: ");
    println!("1. Random daily joke");
    println!("2. Activity recommendation");
    println!("3. Chuck Norris Facts");
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
            "3" => {
                let result = call_chuck_norris_fact().await;
                let ok_cb = |data: ChuckNorrisResponse| {
                    println!("{}", data.value);
                };
                handle_ok_match(result, ok_cb);
            }
            "q" => {
                println!("Bye!");
                break;
            }
            _ => {
                println!("Selected menu does not exist");
            }
        }
        menu_buffer.clear();
        display_menu();

        stdin().read_line(&mut menu_buffer).expect("read line err");
    }
    Ok(())
}
