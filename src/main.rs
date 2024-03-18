use crate::structs::replay_structs::Replay;
use crate::structs::replay_structs::ReplayData;
use dirs;
use lzma_rs::lzma_decompress;
use std::fs::File;
use std::io::Cursor;
use std::io::Read;
use std::io::{self, Write};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::vec;

pub mod structs;

impl Replay {
    fn new() -> Replay {
        Replay {
            mode: 0,
            version: 0,
            osu_md5: String::new(),
            player_name: String::new(),
            replay_md5: String::new(),
            count_300: 0,
            count_100: 0,
            count_50: 0,
            count_geki: 0,
            count_katu: 0,
            count_miss: 0,
            score: 0,
            greatest_combo: 0,
            perfect_combo: false,
            mods: 0,
            life_bar_graph: String::new(),
            timestamp: 0,
            compressed_replay_length: 0,
            compressed_replay_data: Vec::new(),
            online_score_id: 0,
        }
    }
}

fn read_replay(file_path: &str) -> Replay {
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");
    let mut replay = Replay::new();
    let mut index = 0;
    replay.mode = buffer[index];

    if replay.mode != 0 {
        println!("This is not an osu!standard replay");
        std::process::exit(1);
    }

    index += 1;
    replay.version = u32::from_le_bytes([
        buffer[index],
        buffer[index + 1],
        buffer[index + 2],
        buffer[index + 3],
    ]);

    index += 4;
    let mut string = String::new();

    if buffer[index] == 0x0b {
        index += 1;
        let mut length = 0;
        let mut shift = 0;
        loop {
            let byte = buffer[index];
            length |= (byte & 0x7F) << shift;
            if byte & 0x80 == 0 {
                break;
            }
            shift += 7;
            index += 1;
        }
        index += 1;
        let length_usize = length as usize;
        string = String::from_utf8(buffer[index..index + length_usize].to_vec()).unwrap();
        index += length_usize;
    }
    replay.osu_md5 = string;

    let mut string = String::new();
    if buffer[index] == 0x0b {
        index += 1;
        let mut length = 0;
        let mut shift = 0;
        loop {
            let byte = buffer[index];
            length |= (byte & 0x7F) << shift;
            if byte & 0x80 == 0 {
                break;
            }
            shift += 7;
            index += 1;
        }
        index += 1;
        let length_usize = length as usize;
        string = String::from_utf8(buffer[index..index + length_usize].to_vec()).unwrap();
        index += length_usize;
    }
    replay.player_name = string;

    let mut string = String::new();
    if buffer[index] == 0x0b {
        index += 1;
        let mut length = 0;
        let mut shift = 0;
        loop {
            let byte = buffer[index];
            length |= (byte & 0x7F) << shift;
            if byte & 0x80 == 0 {
                break;
            }
            shift += 7;
            index += 1;
        }
        index += 1;
        let length_usize = length as usize;
        string = String::from_utf8(buffer[index..index + length_usize].to_vec()).unwrap();
        index += length_usize;
    }
    replay.replay_md5 = string;

    replay.count_300 = u16::from_le_bytes([buffer[index], buffer[index + 1]]);
    index += 2;
    replay.count_100 = u16::from_le_bytes([buffer[index], buffer[index + 1]]);
    index += 2;
    replay.count_50 = u16::from_le_bytes([buffer[index], buffer[index + 1]]);
    index += 2;
    replay.count_geki = u16::from_le_bytes([buffer[index], buffer[index + 1]]);
    index += 2;
    replay.count_katu = u16::from_le_bytes([buffer[index], buffer[index + 1]]);
    index += 2;
    replay.count_miss = u16::from_le_bytes([buffer[index], buffer[index + 1]]);
    index += 2;
    replay.score = u32::from_le_bytes([
        buffer[index],
        buffer[index + 1],
        buffer[index + 2],
        buffer[index + 3],
    ]);
    index += 4;
    replay.greatest_combo = u16::from_le_bytes([buffer[index], buffer[index + 1]]);
    index += 2;
    replay.perfect_combo = buffer[index] == 0x01;
    index += 1;
    replay.mods = u32::from_le_bytes([
        buffer[index],
        buffer[index + 1],
        buffer[index + 2],
        buffer[index + 3],
    ]);
    index += 4;

    let mut string = String::new();
    if buffer[index] == 0x0b {
        index += 1;
        let mut length: u16 = 0;
        let mut shift = 0;
        loop {
            let byte = buffer[index];
            length |= (byte as u16 & 0x7F) << shift;
            if byte & 0x80 == 0 {
                break;
            }
            shift += 7;
            index += 1;
        }
        index += 1;
        let length_usize = length as usize;
        string = String::from_utf8(buffer[index..index + length_usize].to_vec()).unwrap();
        index += length_usize;
    }
    replay.life_bar_graph = string;

    replay.timestamp = u64::from_le_bytes([
        buffer[index],
        buffer[index + 1],
        buffer[index + 2],
        buffer[index + 3],
        buffer[index + 4],
        buffer[index + 5],
        buffer[index + 6],
        buffer[index + 7],
    ]);
    index += 8;
    replay.compressed_replay_length = u32::from_le_bytes([
        buffer[index],
        buffer[index + 1],
        buffer[index + 2],
        buffer[index + 3],
    ]);
    index += 4;

    replay.compressed_replay_data = vec![0; replay.compressed_replay_length as usize];
    replay
        .compressed_replay_data
        .copy_from_slice(&buffer[index..index + replay.compressed_replay_length as usize]);
    index += replay.compressed_replay_length as usize;
    replay.online_score_id = u64::from_le_bytes([
        buffer[index],
        buffer[index + 1],
        buffer[index + 2],
        buffer[index + 3],
        buffer[index + 4],
        buffer[index + 5],
        buffer[index + 6],
        buffer[index + 7],
    ]);
    return replay;
}

fn decompress_replay(replay: &Replay) -> Vec<ReplayData> {
    let mut buffer: Vec<u8> = Vec::new();
    let compressed = &replay.compressed_replay_data[..];

    let mut reader = Cursor::new(compressed);
    lzma_decompress(&mut reader, &mut buffer).unwrap();
    let mut replay_collection: Vec<ReplayData> = Vec::new();

    let mut file = File::create("decompressed.txt").expect("Failed to create file");
    file.write_all(&buffer).expect("Failed to write to file");

    let file_iter = File::open("decompressed.txt").unwrap();
    let reader = BufReader::new(file_iter);
    let mut time_since_last_action: i64 = 0;
    let mut x_position: f32 = 0.0;
    let mut y_position: f32 = 0.0;
    let mut keys_and_buttons: u32;
    let mut element_checker: i8 = 0;
    let mut temp_string: String = String::new();

    for line in reader.lines() {
        let line = line.unwrap();

        for ch in line.chars() {
            if ch == '|' && element_checker == 0 {
                time_since_last_action = temp_string.parse::<i64>().unwrap();
                element_checker += 1;
                temp_string = String::new();
            } else if ch == '|' && element_checker == 1 {
                x_position = temp_string.parse::<f32>().unwrap();
                element_checker += 1;
                temp_string = String::new();
            } else if ch == '|' && element_checker == 2 {
                y_position = temp_string.parse::<f32>().unwrap();
                element_checker += 1;
                temp_string = String::new();
            } else if ch == ',' && element_checker == 3 {
                keys_and_buttons = temp_string.parse::<u32>().unwrap();
                let replay_data = ReplayData {
                    time_since_last_action,
                    x_position,
                    y_position,
                    keys_and_buttons,
                };
                replay_collection.push(replay_data);
                element_checker = 0;
                temp_string = String::new();
            } else {
                temp_string.push(ch);
            }
        }
    }
    std::fs::remove_file("decompressed.txt").unwrap();
    return replay_collection;
}

fn write_replay(replay: &Replay, replay_data: Vec<ReplayData>) {
    let mut path = dirs::download_dir().unwrap_or(PathBuf::from("."));

    path.push(format!(
        "replay_{}_{}_data.txt",
        replay.player_name, replay.replay_md5
    ));

    let mut file = File::create(path).expect("Failed to create file");
    file.write_all(format!("Mode: {}\n", replay.mode).as_bytes())
        .expect("Failed to write to file");
    file.write_all(format!("Version: {}\n", replay.version).as_bytes())
        .expect("Failed to write to file");
    file.write_all(format!("osu! MD5: {}\n", replay.osu_md5).as_bytes())
        .expect("Failed to write to file");
    file.write_all(format!("Player Name: {}\n", replay.player_name).as_bytes())
        .expect("Failed to write to file");
    file.write_all(format!("Replay MD5: {}\n", replay.replay_md5).as_bytes())
        .expect("Failed to write to file");
    file.write_all(format!("300s: {}\n", replay.count_300).as_bytes())
        .expect("Failed to write to file");
    file.write_all(format!("100s: {}\n", replay.count_100).as_bytes())
        .expect("Failed to write to file");
    file.write_all(format!("50s: {}\n", replay.count_50).as_bytes())
        .expect("Failed to write to file");
    file.write_all(format!("Gekis: {}\n", replay.count_geki).as_bytes())
        .expect("Failed to write to file");
    file.write_all(format!("Katus: {}\n", replay.count_katu).as_bytes())
        .expect("Failed to write to file");
    file.write_all(format!("Misses: {}\n", replay.count_miss).as_bytes())
        .expect("Failed to write to file");
    file.write_all(format!("Score: {}\n", replay.score).as_bytes())
        .expect("Failed to write to file");
    file.write_all(format!("Greatest Combo: {}\n", replay.greatest_combo).as_bytes())
        .expect("Failed to write to file");
    file.write_all(format!("Perfect Combo: {}\n", replay.perfect_combo).as_bytes())
        .expect("Failed to write to file");
    file.write_all(format!("Mods: {}\n", replay.mods).as_bytes())
        .expect("Failed to write to file");
    file.write_all(format!("Life Bar Graph: {}\n", replay.life_bar_graph).as_bytes())
        .expect("Failed to write to file");
    file.write_all(format!("Timestamp: {}\n", replay.timestamp).as_bytes())
        .expect("Failed to write to file");
    file.write_all(
        format!(
            "Compressed Replay Length: {}\n",
            replay.compressed_replay_length
        )
        .as_bytes(),
    )
    .expect("Failed to write to file");
    file.write_all(format!("Online Score ID: {}\n", replay.online_score_id).as_bytes())
        .expect("Failed to write to file");
    for data in replay_data {
        file.write_all(
            format!(
                "ReplayEvent:{},{},{},{}\n",
                data.time_since_last_action,
                data.x_position,
                data.y_position,
                data.keys_and_buttons
            )
            .as_bytes(),
        )
        .expect("Failed to write to file");
    }
}

fn main() {
    let mut path = String::new();
    print!("Please enter the path of the file: ");

    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut path)
        .expect("Failed to read line");

    path = path.trim().to_string();

    if !Path::new(&path).exists() {
        println!("No file exists at the given path");
        return;
    }

    let replay = read_replay(&path);
    let replay_data = decompress_replay(&replay);
    write_replay(&replay, replay_data);

    println!(
        "File has been written to the download directory as replay_{}_{}_data.txt",
        replay.player_name, replay.replay_md5
    );

    println!("Press enter to exit");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut String::new())
        .expect("Failed to read line");
}
