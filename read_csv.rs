use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

#[derive(Debug)]
struct RawData {
    pub width: i32,
    pub height: i32,
    pub data: Vec<u16>,
}

fn open_and_read(path: &str) -> Result<RawData,std::io::Error> {
    let f = File::open(path)?;
    let file = BufReader::new(&f);
    let mut read_header = false;
    let mut rd = RawData { width: -1, height: -1, data: Vec::<u16>::new()};
    let mut data_buf = Vec::<u16>::new();
    let mut count = 0;
    for line in file.lines() {
        let buf = line?;
        let tokens : Vec<&str> = buf.split(',').collect();
        if !read_header {
            let hdr : Vec<i32> = tokens.into_iter().map(|t|{t.parse::<i32>().unwrap()}).collect();
            rd.width = hdr[0];
            rd.height = hdr[1];
            println!("{}x{}", rd.width, rd.height);
            data_buf.resize((rd.width*rd.height) as usize, 0);
            read_header = true;
        } else {
            let values : Vec<u16> = tokens.into_iter().map(|t|{t.parse::<u16>().unwrap()}).collect();
            println!("{:?}", values);
            for v in values {
                data_buf[count] = v;
                count += 1;
            }
        }
    }
    rd.data = data_buf;
    return Ok(rd);
}

fn main() {
    let raw_data = open_and_read("test.csv");
    println!("{:?}", raw_data.unwrap());
    // let mut buf = String::new();
    // let size : usize;
    // {
    //     size = file.read_line(&mut buf).unwrap();
    //     if size <= 0 {
    //         println!("File is empty!");
    //         return;
    //     }
    // }
    // let width : i32;
    // let height : i32;
    // {
    //     let header : Vec<&str> = buf.split(',').collect();
    //     println!("{:?}", header);
    //     width = header[0].parse::<i32>().unwrap();
    //     height = header[1].parse::<i32>().unwrap();
    // }
    // println!("{}x{}", width, height);
    // while file.read_line(&mut buf).unwrap() > 0 {
    //     let items : Vec<&str> = buf.split(',').collect();
    //     // println!("{}", );
    // }
}