use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let reader = BufReader::new(File::open(file_path)?);

    let mut buf: [u8; 10000] = [0; 10000];
    let mut ptr = 0;
    let mut loop_vec = Vec::new();

    for result in reader.lines() {
        let li = result?;
        let mut index = 0;
        while index < li.len() {
            let order = &li[index..index + 18];
            if order == "ズンズンズン" {
                if ptr < buf.len() {
                    ptr += 1;
                } else {
                    eprintln!("ポインタが大きすぎます");
                }
            } else if order == "ズンズンドコ" {
                if ptr > 0 {
                    ptr -= 1;
                } else {
                    eprintln!("ポインタが負になりました");
                }
            } else if order == "ズンドコズン" {
                buf[ptr] += 1;
            } else if order == "ズンドコドコ" {
                buf[ptr] -= 1;
            } else if order == "ドコズンズン" {
                if buf[ptr] == 0 {
                    let mut w_count = 1;
                    index += 18;
                    while w_count > 0 {
                        if index >= li.len() {
                            eprintln!("ドコズンドコがありません");
                        }
                        if &li[index..index + 18] == "ドコズンズン" {
                            w_count += 1;
                        } else if &li[index..index + 18] == "ドコズンドコ" {
                            w_count -= 1;
                        }
                        index += 18;
                    }
                } else {
                    loop_vec.push(index);
                }
            } else if order == "ドコズンドコ" {
                if buf[ptr] != 0 {
                    index = loop_vec.pop().unwrap();
                    loop_vec.push(index);
                } else {
                    loop_vec.pop();
                }
            } else if order == "ドコドコズン" {
                print!("{}", buf[ptr] as char);
            } else if order == "ドコドコドコ" {
                let mut w_input = [0u8; 1];
                let _ = std::io::stdin().read(&mut w_input);
                buf[ptr] = w_input[0];
            } else if order == "キ・ヨ・シ！" {
                print!("Hello, World!");
            }
            index += 18;
        }
    }
    Ok(())
}
