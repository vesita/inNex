use std::collections::HashMap;
use std::io::BufRead;
use std::ops::Index;

use crate::model::fast::bin_search;

enum TravelMode {
    Word,
    Line,
    Token,
}

struct Content {
    content: Vec<Vec<char>>,
    // 记录每行起始的抽象索引
    indexes: Vec<usize>,
    // 抽象索引 -> 行索引
    innk: HashMap<usize, usize>,
}

struct Travel {
    content: Content,

    begin: usize,
    end: usize,
    now: usize,
}

impl Index<usize> for Content {
    type Output = char;

    fn index(&self, index: usize) -> &Self::Output {
        // 找到对应的行
        let line =
            bin_search::nomore_tar(&self.indexes, index).expect("索引超出范围，未找到对应的行");

        // 计算字符在行中的位置
        let char_index = index - self.indexes[line];

        // 获取该行字符串并返回对应字符的引用
        &self.content[line][char_index]
    }
}

impl Content {
    fn new() -> Self {
        Content {
            content: Vec::new(),
            indexes: Vec::new(),
            innk: HashMap::new(),
        }
    }

    fn read_file(&mut self, file_path: &str) {
        let file = std::fs::File::open(file_path).expect("无法打开文件");
        let reader = std::io::BufReader::new(file);
        let mut index = 0;
        for (line_num, line) in reader.lines().enumerate() {
            let line = line.expect("无法读取行");
            self.content.push(line.chars().collect());
            self.indexes.push(index);
            self.innk.insert(index, line_num);
            index += line.len();
        }
    }
}

impl Travel {
    fn new() -> Self {
        let content = Content::new();

        Travel {
            content,
            begin: 0,
            end: 0,
            now: 0,
        }
    }

    fn read_file(&mut self, file_path: &str) {
        self.content.read_file(file_path);
        self.end = self.content.indexes.last().copied().unwrap_or(0);
        self.end += self.content.content.len();
    }
}
