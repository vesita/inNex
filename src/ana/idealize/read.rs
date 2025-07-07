use std::fs::File;
use std::io::{BufRead, BufReader};

/*
参数
类型： 字符串
描述： 文件名

返回值
类型： [字符串]
描述： 一般为文件的内容，否则为空字符串
*/
fn read(filename: &str) {
    let mut file = File::open(filename).expect("file not found");
    let reader = BufReader::new(file);

    let contents = vec::new();
    for line in reader.lines() {
        contents.push(line.unwrap());
    }
    return contents;
}

/*
参数

返回值
*/
fn read_key(file: &str) {
    let file = Wopol::new();
}
