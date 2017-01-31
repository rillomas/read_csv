use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

#[derive(Debug)]
struct RawData {
    width: i32,
    height: i32,
    data: Vec<u16>,
}

fn tokenize<T: std::str::FromStr>(line: std::string::String) -> Vec<T>
    where <T as std::str::FromStr>::Err: std::fmt::Debug {
    let delim = ',';
    let tokens : Vec<&str> = line.split(delim).collect();
    let t : Vec<T> = tokens.into_iter().map(|t|{t.parse::<T>().unwrap()}).collect();
    return t;
}

fn open_and_read(path: &str) -> Result<RawData,std::io::Error> {
    let f = File::open(path)?;
    let file = BufReader::new(&f);
    let mut rd = RawData { width: -1, height: -1, data: Vec::<u16>::new()};
    let mut data_buf = Vec::<u16>::new();
    let mut count = 0;
    let mut itr = file.lines();
    // read the header first
    let header = itr.next().unwrap()?;
    let hdr = tokenize::<i32>(header);
    rd.width = hdr[0];
    rd.height = hdr[1];
    println!("{}x{}", rd.width, rd.height);
    data_buf.resize((rd.width*rd.height) as usize, 0);
    // read the data
    for line in itr {
        let linestr = line.unwrap();
        let values = tokenize::<u16>(linestr);
        println!("{:?}", values);
        for v in values {
            data_buf[count] = v;
            count += 1;
        }
    }
    rd.data = data_buf;
    return Ok(rd);
}

fn main() {
    let raw_data = open_and_read("test.csv");
    println!("{:?}", raw_data.unwrap());
}