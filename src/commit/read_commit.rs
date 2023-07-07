use std::{path:: PathBuf, fs::File, io::Read};

use flate2::read::ZlibDecoder;

pub fn read_commit(commit_hash: &str) -> Result<String, String> {
    let (dir, file) = commit_hash.split_at(2);

    let mut target_path = PathBuf::new();
    target_path.push(".git/objects");
    target_path.push(dir);
    target_path.push(file);

    let mut buf = Vec::new(); 
    let mut f = match File::open(target_path){
			Ok(file) => file,
			Err(e) => return Err(e.to_string()),
		};
    match f.read_to_end(&mut buf){
			Ok(_) => {},
			Err(e) => return Err(e.to_string()),
		};

    let mut content:Vec<u8> = Vec::new(); // 伸長用のバッファはbyteのベクタで用意
    let mut decoder = ZlibDecoder::new(&buf[..]);
    match decoder.read_to_end(&mut content){
			Ok(_) => {},
			Err(e) => return Err(e.to_string()),
		};

    let mut original = content.splitn(2, |&x| x == b'\0');

    let _header = original.next().unwrap();

    // Rustっぽい書き方してみた
    let tree = original.next().and_then(|c| {
        c.split(|&x| x == b'\n')
            .filter(|x| !x.is_empty())
            .map(|x| String::from_utf8_lossy(x).to_string())
            .find_map(|x| x.split_whitespace().nth(1).map(|x| x.to_string()))
    });

    Ok(tree.unwrap())
}