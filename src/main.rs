use clap::*;
use colored::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::SeekFrom;
use std::io::{self, prelude::*, BufReader, Seek};
use std::{thread, time};

const APP_VERSION: &str = "0.5.1";

pub enum PrintColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Normal,
}

pub enum PrintStyle {
    Normal,
    Bold,
    Underline,
    Italic,
    Dimmed,
}

pub enum LogLevel {
    Info,
    Debug,
    Warning,
    Error,
    Trace,
    Fatal,
}

pub enum DataType {
    Unknown,
    Timestamp,
    Level,
    Message,
}

pub struct Column {
    pub title: String,
    pub width: u32,
    pub data_type: DataType,
    pub print_style: PrintStyle,
}

fn main() -> std::io::Result<()> {
    let settings = App::new("Json Log Viewer")
        .version(APP_VERSION)
        .author("Dmitry Z. <dz64@protonmail.com>")
        .about("Tool for json logs visualization")
        .arg(Arg::with_name("follow").short("f").long("follow"))
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("num-lines")
                .long("num-lines")
                .short("n")
                .takes_value(true)
                .help("Number of lines to print from the end"),
        )
        .get_matches();

    let is_follow = settings.is_present("follow");
    let num_lines = match settings.is_present("num-lines") {
        true => match settings.value_of("num-lines") {
            Some(val) => val.parse::<u32>().unwrap(),
            None => 30,
        },
        false => 30,
    };

    let path = settings.value_of("INPUT").unwrap().to_string();

    read_file(&path, is_follow, num_lines);
    io::stdout().flush().unwrap();
    Ok(())
}

fn seek_to_end<R>(reader: &mut BufReader<R>, num_lines: u32)
    where
        R: Seek + Read,
{
    reader.seek(SeekFrom::End(0)).unwrap();

    let mut lines = 0;
    let mut buf = vec![0u8; 1];

    while reader.seek(SeekFrom::Current(-1)).unwrap() > 0 {
        reader.read(&mut buf).unwrap();
        if buf[0] == '\n' as u8 {
            lines += 1;
            if lines > num_lines {
                break;
            }
        }
        reader.seek(SeekFrom::Current(-1)).unwrap();
    }
}

fn read_file(path: &String, follow: bool, num_lines: u32) {
    let file = File::open(path);
    let actual_file;
    match file {
        Ok(f) => actual_file = f,
        Err(why) => {
            println!("Failed to open file! Reason {}", why);
            return;
        }
    }
    let mut reader = BufReader::new(actual_file);
    let mut header_shown: bool = false;
    let mut columns: HashMap<String, Column> = HashMap::new();
    let mut common_buffer: Vec<u8> = vec![];
    seek_to_end(&mut reader, num_lines);
    loop {
        let mut read_buffer: Vec<u8> = vec![];
        let res = reader.read_until('\n' as u8, &mut read_buffer);
        match res {
            Ok(length) => {
                if length == 0 {
                    if !follow {
                        return;
                    } else {
                        let poll_time = time::Duration::from_millis(10);
                        thread::sleep(poll_time);
                    }
                }

                common_buffer.extend(read_buffer[0..length].iter().cloned());
                let mut might_be_lines = true;
                while might_be_lines {
                    match try_get_line(&common_buffer) {
                        Some(index) => {
                            let line: String = std::str::from_utf8(&common_buffer[0..index])
                                .unwrap()
                                .to_string();

                            common_buffer.drain(0..index);

                            if length != 0 {
                                if !header_shown {
                                    let cols = extract_header(&line);
                                    print_header(&cols);
                                    for column in cols {
                                        columns.insert(column.title.clone(), column);
                                    }
                                    header_shown = true;
                                }
                                print_json_line(&line, &columns);
                            }
                        }
                        None => {
                            might_be_lines = false;
                        }
                    }
                }
            }
            Err(err) => println!("Error! {}", err),
        }
    }
}

fn try_get_line(byte_array: &Vec<u8>) -> Option<usize> {
    let length = byte_array.len();
    let mut found: bool = false;
    let mut index: usize = 0;
    for i in 0..length {
        if byte_array[i] == '\n' as u8 {
            found = true;
            index = i;
            break;
        }
    }
    if found {
        return Some(index + 1);
    } else {
        return None;
    }
}

fn extract_header(first_line: &String) -> Vec<Column> {
    let parsed_json_result = json::parse(&first_line);
    match parsed_json_result {
        Ok(parsed_json) => {
            let entries = parsed_json.entries();
            let mut result: Vec<Column> = vec![];
            for (key, value) in entries {
                let title = key.to_string();
                let lowercase = title.to_ascii_lowercase();
                let mut data_type: DataType = DataType::Unknown;
                let print_style: PrintStyle;
                match &lowercase[..] {
                    "time" | "timestamp" | "t" | "date" | "datetime" => {
                        data_type = DataType::Timestamp;
                        print_style = PrintStyle::Dimmed;
                    }
                    "message" | "msg" | "m" | "value" | "payload" | "data" => {
                        data_type = DataType::Message;
                        print_style = PrintStyle::Normal;
                    }
                    "level" | "log_level" | "lvl" | "level_name" => {
                        data_type = DataType::Level;
                        print_style = PrintStyle::Dimmed;
                    }
                    _ => {
                        print_style = PrintStyle::Dimmed;
                    }
                }

                let column: Column = Column {
                    title,
                    width: (value.to_string().len()) as u32,
                    data_type,
                    print_style,
                };
                result.push(column);
            }
            return result;
        }
        Err(error) => {
            println!("Cannot parse header! {}", error);
            return vec![];
        }
    }
}

fn print_json_line(line: &String, columns: &HashMap<String, Column>) {
    if line.len() < 2 {
        return;
    }
    let parsed_json_result = json::parse(line);
    match parsed_json_result {
        Ok(parsed_json) => {
            let entries = parsed_json.entries();
            let mut print: Vec<ColoredString> = vec![];
            let mut log_level: LogLevel = LogLevel::Info;
            for (key, value) in entries {
                let column_result = columns.get(key);
                match column_result {
                    Some(column) => {
                        let mut print_value: String;
                        match column.data_type {
                            DataType::Level => {
                                log_level = get_log_level(&value.to_string());
                                print_value =
                                    value.to_string()[0..1].to_string().to_ascii_uppercase();
                            }
                            _ => {
                                print_value = value.to_string();
                            }
                        }

                        print_value = "[".to_string() + &print_value + "]";

                        let styled: ColoredString;
                        match column.print_style {
                            PrintStyle::Normal => styled = print_value.normal(),
                            PrintStyle::Bold => styled = print_value.bold(),
                            PrintStyle::Underline => styled = print_value.underline(),
                            PrintStyle::Italic => styled = print_value.italic(),
                            PrintStyle::Dimmed => styled = print_value.dimmed(),
                        }

                        print.push(styled);
                    }
                    None => print.push(
                        ("[".to_string() + &value.to_string() + "]")
                            .black()
                            .dimmed(),
                    ),
                }
            }

            for string in print {
                match log_level {
                    LogLevel::Debug => print!("{}", string.green()),
                    LogLevel::Error => print!("{}", string.red()),
                    LogLevel::Warning => print!("{}", string.yellow()),
                    LogLevel::Info => print!("{}", string.white()),
                    LogLevel::Trace => print!("{}", string.blue()),
                    LogLevel::Fatal => print!("{}", string.purple()),
                }
            }
            println!();
        }
        Err(error) => println!("Failed to parse, error: {} line: {}", error, line.red()),
    }
    io::stdout().flush().unwrap();
}

fn get_log_level(level: &String) -> LogLevel {
    let lowercase = level.to_ascii_lowercase();
    match &lowercase[..] {
        "info" | "i" => LogLevel::Info,
        "debug" | "d" => LogLevel::Debug,
        "warning" | "w" | "warn" => LogLevel::Warning,
        "error" | "e" | "err" => LogLevel::Error,
        "trace" | "t" => LogLevel::Trace,
        "fatal" | "f" => LogLevel::Fatal,
        _ => LogLevel::Info,
    }
}

fn print_header(columns: &Vec<Column>) {
    let mut header: String = String::new();

    for column in columns {
        let title = &column.title;
        header.push_str(&title.to_ascii_uppercase());
        header.push_str("\t");
    }

    println!("{}", header);
}
