
use rss::Channel;
use serde::Serialize;

#[derive(Serialize)]
struct FeedItem {
    title: Option<String>,
    description: Option<String>,
}

#[tauri::command]
async fn fetch_feed(url: &str) -> Result<Vec<FeedItem>, String> {
    println!("Beginning to fetch RSS feed...");
    let content = reqwest::get(url)
        .await
        .map_err(|e| e.to_string())?
        .bytes()
        .await
        .map_err(|e| e.to_string())?;

    // println!("step one in RSS feed...");

    // println!("{}", String::from_utf8_lossy(&content)); //print all content


    let channel =
        Channel::read_from(&content[..]).map_err(|e| e.to_string())?;

    // println!("step two in RSS feed...");

    Ok(channel.items()
        .iter()
        .map(|i| FeedItem {
            title: i.title().map(|s| s.to_string()),
            description: i.description().map(|s| s.to_string()),
        })
        .collect())
        
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn fun_fact(name: &str) -> String {
    format!("Hey, {}! Here's a fun fact: Rust is a systems programming language!", name)
}






// async fn example_feed() -> Result<Channel, Box<dyn Error>> {
//     let content = reqwest::get("http://example.com/feed.xml")
//         .await?
//         .bytes()
//         .await?;
//     let channel = Channel::read_from(&content[..])?;
//     Ok(channel)
// }

// #[tauri::command]
// fn main() -> Result<(), Box<dyn Error>> {
//     // 1. Fetch the RSS content
//     let url = "https://www.rust-lang.org";
//     let content = reqwest::blocking::get(url)?.bytes()?;

//     // 2. Parse the content
//     let channel = Channel::read_from(&content[..])?;

//     // 3. Use the data
//     println!("Feed Title: {}", channel.title());
//     for item in channel.items() {
//         if let Some(title) = item.title() {
//             println!("Item: {}", title);
//         }
//     }

//     Ok(())
// }


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, fun_fact, fetch_feed])
        // .invoke_handler(tauri::generate_handler![fun_fact])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
