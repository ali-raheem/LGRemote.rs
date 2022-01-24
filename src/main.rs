use std::env;
use std::process::exit;
use LGremote::{COMMAND_CODES, LGTV};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!(
            "Usage:\n\t{} IP [KEY] [COMMAND]\n\nThe IP address must be provided.
If only an IP is provided the TV will be asked to display it's pairing code.
Subsequent requests will try and pair and runany command requested.",
            args[0]
        );
        exit(1);
    }
    let ip = args[1].clone();

    let mut tv = LGTV::new(ip);

    if args.len() == 2 {
        let res = tv.display_pair_key().await.unwrap();
        if 200 == res.status() {
            println!("Requested authkey to be displayed.");
            exit(0);
        } else {
            println!("Failed to request auth key. Check IP.");
            exit(1);
        }
    }

    let key: u32 = args[2].parse().expect("Check Key it should be a number.");
    let res = tv.pair_with_key(key).await.unwrap();
    if 200 != res.status() {
        println!("Auth failed check IP/Key..");
        exit(1);
    } else {
        println!("Paired with TV. You can now start issuing commands.");
    }

    if args.len() == 4 {
        let command = match COMMAND_CODES.get(&args[3].to_uppercase() as &str) {
            Some(c) => c.clone(),
            None => {
                println!("Command not found.");
                exit(1)
            }
        };

        let res = tv.send_command(command).await.unwrap();
        println!("Sending command {}", command);
        if 200 == res.status() {
            println!("Command recieved.");
            exit(0);
        } else {
            println!("TV did not get command.");
            exit(1);
        }
    }
}
