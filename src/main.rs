use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

const BUF_SIZE: usize = 10000;

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let reader = BufReader::new(File::open(file_path)?);

    let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
    let mut ptr = 0;
    let mut loop_vec = Vec::new();

    for result in reader.lines()
    {
        let li = result?;
        let li_size = li.len();
        let mut index = 0;
        while index < li_size
        {
            match &li[index..index + 18]
            {
                "ズンズンズン" =>
                {
                    if ptr < BUF_SIZE
                    {
                        ptr += 1;
                    }
                    else
                    {
                        eprintln!("ポインタが大きすぎます");
                    }
                },
                "ズンズンドコ" =>
                {
                    if ptr > 0
                    {
                        ptr -= 1;
                    }
                    else
                    {
                        eprintln!("ポインタが負になりました");
                    }
                },
                "ズンドコズン" =>
                {
                    buf[ptr] += 1;
                },
                "ズンドコドコ" =>
                {
                    buf[ptr] -= 1;
                },
                "ドコズンズン" =>
                {
                    match buf[ptr]
                    {
                        0 =>
                        {
                            let mut w_count = 1;
                            index += 18;
                            while w_count > 0
                            {
                                if index >= li_size
                                {
                                    eprintln!("ドコズンドコがありません");
                                }

                                if &li[index..index + 18] == "ドコズンズン"
                                {
                                    w_count += 1;
                                }
                                else if &li[index..index + 18] == "ドコズンドコ"
                                {
                                    w_count -= 1;
                                }
                                index += 18;
                            }
                        },
                        _ =>
                        {
                            loop_vec.push(index);
                        },
                    }
                },
                "ドコズンドコ" =>
                {
                    match buf[ptr]
                    {
                        0 =>
                        {
                            loop_vec.pop();
                        },
                        _ =>
                        {
                            index = loop_vec.pop().unwrap();
                            loop_vec.push(index);
                        },
                    }
                },
                "ドコドコズン" =>
                {
                    print!("{}", buf[ptr] as char);
                },
                "ドコドコドコ" =>
                {
                    let mut w_input = [0u8; 1];
                    let _ = std::io::stdin().read(&mut w_input);
                    buf[ptr] = w_input[0];
                },
                "キ・ヨ・シ！" =>
                {
                    print!("Hello, World!");
                },
                _ =>
                {
                    eprintln!("ズンドコできません");
                },
            }
            index += 18;
        }
    }
    Ok(())
}
