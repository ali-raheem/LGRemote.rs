use reqwest::{Client, Response};
use std::collections::HashMap;
use std::env;
use std::process::exit;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let COMMAND_CODES: HashMap<&str, u32> = HashMap::from([
        ("POWER", 1),
        ("NUM_0", 2),
        ("NUM_1", 3),
        ("NUM_2", 4),
        ("NUM_3", 5),
        ("NUM_4", 6),
        ("NUM_5", 7),
        ("NUM_6", 8),
        ("NUM_7", 9),
        ("NUM_8", 10),
        ("NUM_9", 11),
        ("UP", 12),
        ("DOWN", 13),
        ("LEFT", 14),
        ("RIGHT", 15),
        ("OK", 20),
        ("HOME", 21),
        ("MENU", 22),
        ("BACK", 23),
        ("VOLUME_UP", 24),
        ("VOLUME_DOWN", 25),
        ("MUTE", 26),
        ("CHANNEL_UP", 27),
        ("CHANNEL_DOWN", 28),
        ("BLUE", 29),
        ("GREEN", 30),
        ("RED", 31),
        ("YELLOW", 32),
        ("PLAY", 33),
        ("PAUSE", 34),
        ("STOP", 35),
        ("FF", 36),
        ("REW", 37),
        ("SKIP_FF", 38),
        ("SKIP_REW", 39),
        ("REC", 40),
        ("REC_LIST", 41),
        ("LIVE", 43),
        ("EPG", 44),
        ("INFO", 45),
        ("ASPECT", 46),
        ("EXT", 47),
        ("PIP", 48),
        ("SUBTITLE", 49),
        ("PROGRAM_LIST", 50),
        ("TEXT", 51),
        ("MARK", 52),
        ("3D", 400),
        ("3D_LR", 401),
        ("DASH", 402),
        ("PREV", 403),
        ("FAV", 404),
        ("QUICK_MENU", 405),
        ("TEXT_OPTION", 406),
        ("AUDIO_DESC", 407),
        ("NETCAST", 408),
        ("ENERGY_SAVE", 409),
        ("AV", 410),
        ("SIMPLINK", 411),
        ("EXIT", 412),
        ("RESERVED", 413),
        ("PIP_CHANNEL_UP", 414),
        ("PIP_CHANNEL_DOWN", 415),
        ("PIP_SWITCH", 416),
        ("APPS", 417),
    ]);

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
    let command = match COMMAND_CODES.get(&args[3] as &str) {
	Some(c) => c.clone(),
	None => {
	    println!("Command not found.");
	    exit(1)
	}
    };
        
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

