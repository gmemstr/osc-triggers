extern crate inputbot;

use std::{
    net::{SocketAddrV4, UdpSocket},
    str::FromStr,
};

use inputbot::{get_keybd_key, KeybdKey, KeybdKey::*};
use rosc::OscPacket;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Configuration struct with mapping of OSC addresses to keyboard keys, and a general server configuration.
#[derive(Deserialize, Serialize)]
struct Config {
    server: Option<ServerConfig>,
    // Mappings is called mapping in config for ease of use.
    #[serde(rename = "mapping")]
    mappings: Vec<EventKeyMapping>,
}
#[derive(Deserialize, Serialize)]
struct EventKeyMapping {
    event: String,
    key: String,
    value: Option<String>,
}

#[derive(Deserialize, Serialize)]
struct ServerConfig {
    port: u16,
}

struct EventCache {
    value: Option<String>,
    key: KeybdKey,
}

fn main() {
    let config = match load_config() {
        Ok(c) => c,
        Err(e) => {
            println!("Error loading config file: {}", e);
            return;
        }
    };

    let mut event_key_map = HashMap::new();
    for mapping in config.mappings {
        // Convert the key string to a KeybdKey. First try to convert it to an F key, then to a normal key by matching the char.
        let key = match to_fkey(&mapping.key) {
            Some(k) => k,
            None => match mapping.key.chars().next() {
                Some(c) => get_keybd_key(c).unwrap(),
                None => {
                    println!("Invalid key: {}", mapping.key);
                    return;
                }
            },
        };
        let ec = EventCache {
            value: mapping.value,
            key,
        };
        event_key_map.insert(mapping.event, ec);
    }

    let port = config.server.unwrap().port;
    println!("Starting OSC server on port {}...", port);
    let addr = match SocketAddrV4::from_str(format!("127.0.0.1:{}", port).as_str()) {
        Ok(addr) => addr,
        Err(_) => panic!("Unable to match address"),
    };
    let sock = UdpSocket::bind(addr).unwrap();
    println!("Listening to {}", addr);

    let mut buf = [0u8; rosc::decoder::MTU];

    loop {
        match sock.recv_from(&mut buf) {
            Ok((size, _addr)) => {
                let (_, packet) = rosc::decoder::decode_udp(&buf[..size]).unwrap();
                handle_packet(packet, &event_key_map);
            }
            Err(e) => {
                println!("Error receiving from socket: {}", e);
                break;
            }
        }
    }
}

fn load_config() -> Result<Config, String> {
    // If the config file isn't found, create one with default values
    if !std::path::Path::new("config.toml").exists() {
        let default_config = Config {
            server: Some(ServerConfig { port: 9001 }),
            mappings: vec![],
        };
        let toml = toml::to_string_pretty(&default_config).map_err(|e| e.to_string())?;
        std::fs::write("config.toml", toml).map_err(|e| e.to_string())?;
        println!("Created default config file at config.toml");
    }
    let config_file = std::fs::read_to_string("config.toml").map_err(|e| e.to_string())?;
    let mut config: Config = toml::from_str(&config_file).map_err(|e| e.to_string())?;
    // Provide a default ServerConfig if one is not set.
    if config.server.is_none() {
        config.server = Some(ServerConfig { port: 9001 });
    }
    Ok(config)
}

fn handle_packet(packet: OscPacket, mappings: &HashMap<String, EventCache>) {
    match packet {
        OscPacket::Message(msg) => {
            let reaction = match mappings.get(&msg.addr) {
                Some(k) => k,
                None => {
                    return;
                }
            };
            if reaction.value.is_some() {
                if msg.args.len() != 1 {
                    return;
                }
                if msg.args[0].clone().string() != reaction.value {
                    return;
                }
            }
            // Press key
            println!("Pressing key: {:?}", reaction.key);
            reaction.key.press();
            reaction.key.release();
        }
        OscPacket::Bundle(_bundle) => {}
    }
}

// Translate "F1", "F2", "F3" etc to corresponding KeybdKey.
// inputbot doesn't currently support this natively.
fn to_fkey(key: &str) -> Option<KeybdKey> {
    match key {
        "F1" => Some(F1Key),
        "F2" => Some(F2Key),
        "F3" => Some(F3Key),
        "F4" => Some(F4Key),
        "F5" => Some(F5Key),
        "F6" => Some(F6Key),
        "F7" => Some(F7Key),
        "F8" => Some(F8Key),
        "F9" => Some(F9Key),
        "F10" => Some(F10Key),
        "F11" => Some(F11Key),
        "F12" => Some(F12Key),
        "F13" => Some(F13Key),
        "F14" => Some(F14Key),
        "F15" => Some(F15Key),
        "F16" => Some(F16Key),
        "F17" => Some(F17Key),
        "F18" => Some(F18Key),
        "F19" => Some(F19Key),
        "F20" => Some(F20Key),
        "F21" => Some(F21Key),
        "F22" => Some(F22Key),
        "F23" => Some(F23Key),
        "F24" => Some(F24Key),
        _ => None,
    }
}

mod tests {
    // Test to_fkey
    #[test]
    fn test_to_fkey() {
        assert_eq!(super::to_fkey("F1"), Some(super::F1Key));
        assert_eq!(super::to_fkey("F2"), Some(super::F2Key));
        assert_eq!(super::to_fkey("F25"), None);
    }
}
