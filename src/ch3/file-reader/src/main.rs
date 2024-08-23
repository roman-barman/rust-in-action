use rand::prelude::*;

fn one_in(denominator: u32) -> bool {
    thread_rng().gen_ratio(1, denominator)
}

#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed
        }
    }

    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File not opened"));
        }

        let mut tmp = self.data.clone();
        let read_len = tmp.len();
        save_to.reserve(read_len);
        save_to.append(&mut tmp);
        Ok(read_len)
    }
}

fn open(mut f: File) -> Result<File, String> {
    if one_in(10_000) {
        let err_msg = String::from("Permission denied");
        return Err(err_msg);
    }

    f.state = FileState::Open;
    Ok(f)
}

fn close(mut f: File) -> Result<File, String> {
    if one_in(100_000) {
        let err_msg = String::from("Interrupted by signal!");
        return Err(err_msg);
    }

    f.state = FileState::Closed;
    Ok(f)
}

fn main() {
    let f_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut file = File::new_with_data("data.txt", &f_data);

    let mut buffer: Vec<u8> = vec![];

    file = open(file).unwrap();
    let f_length = file.read(&mut buffer).unwrap();
    file = close(file).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", file);
    println!("{} is {} bytes long", file.name, f_length);
    println!("{}", text);
}
