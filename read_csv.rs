use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn open_and_read(path: &str) -> Result<Vec<u16>,std::io::Error> {
    let f = File::open(path)?;
    let file = BufReader::new(&f);
    let mut read_header = false;
    let mut width : i32;
    let mut height : i32;
    let mut data_buf = Vec::<u16>::new();
    let mut count = 0;
    for line in file.lines() {
        let buf = line?;
        let tokens : Vec<&str> = buf.split(',').collect();
        if !read_header {
            let hdr : Vec<i32> = tokens.into_iter().map(|t|{t.parse::<i32>().unwrap()}).collect();
            width = hdr[0];
            height = hdr[1];
            println!("{}x{}", width, height);
            data_buf.resize((width*height) as usize, 0);
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
    return Ok(data_buf);
}

fn main() {
    let data_buf = open_and_read("test.csv");
    println!("{:?}", data_buf.unwrap());
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