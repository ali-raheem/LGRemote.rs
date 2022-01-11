use reqwest::{Client, Response};
use std::collections::HashMap;
use std::env;
use std::process::exit;
use toml::Value;
use std::fs;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let commands = &fs::read_to_string("/etc/LGremote/commands.toml").unwrap().parse::<Value>().unwrap();

    if args.len() < 2 {
	println!("Usage:\n\t{} IP [KEY] [COMMAND]\n\nThe IP address must be provided.
If only an IP is provided the TV will be asked to display it's pairing code.
Subsequent requests will try and pair and runany command requested.", args[0]);
	   exit(1);
    }
    let ip = args[1].clone();
    let client = reqwest::Client::new();

    if args.len() < 4 {
	let res = display_pair_key(&ip, client.clone());
	if 200 == res.await.status() {
	   println!("Requested authkey to be displayed.");
	   exit(0);
	} else {
	  println!("Failed to request auth key. Check IP.");
	  exit(1);
	}
    }

    let key: u32 = args[2].parse().expect("Check Key it should be a number.");
    let command = commands[&args[3] as &str].as_integer().unwrap() as u32;
        
    let res = pair_with_key(&ip, client.clone(), key);
    if 200 != res.await.status() {
       println!("Auth failed check IP/Key..");
       exit(1);
    }

    let res = send_command(&ip, client.clone(), command);
    println!("Response status {}", res.await.status());
}

async fn display_pair_key(ip: &str, client: Client) -> Response {
    let display_pair_key = "
        <?xml version=\"1.0\" encoding=\"utf-8\"?>
        <auth>
            <type>AuthKeyReq</type>
        </auth>";
    let url = format!("http://{}:8080/roap/api/auth", ip);
    client
        .post(url)
        .header("Content-Type", "application/atom+xml")
        .body(display_pair_key)
        .send()
        .await
        .expect("Failed to display key check IP.")
}

async fn pair_with_key(ip: &str, client: Client, key: u32) -> Response {
    let auth_request = format!(
        "
        <?xml version=\"1.0\" encoding=\"utf-8\"?>
        <auth>
            <type>AuthReq</type>
            <value>{}</value>
        </auth>
   ",
        key
    );
    let url = format!("http://{}:8080/roap/api/auth", ip);
    client
        .post(url)
        .header("Content-Type", "application/atom+xml")
        .body(auth_request)
        .send()
        .await
        .expect("Failed to pair. Check IP.")
}

async fn send_command(ip: &str, client: Client, cmd_code: u32) -> Response {
    let send_command = format!(
        "
        <?xml version=\"1.0\" encoding=\"utf-8\"?>
        <command>
            <name>HandleKeyInput</name>
            <value>{}</value>
        </command>
   ",
        cmd_code
    );

    let url = format!("http://{}:8080/roap/api/command", ip);
    client
        .post(url)
        .header("Content-Type", "application/atom+xml")
        .body(send_command)
        .send()
        .await
        .expect("Failed to send command, check IP, Key and Command availability.")
}

