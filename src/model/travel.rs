use std::io::BufRead;
use std::ops::Index;

use crate::model::pano::Pano;
use crate::utils::wordcode::wordtype;

enum TravelMode {
    Word,
    Line,
    Token,
}

// 原始文本的基础抽象
struct Content {
    pn: Pano,
    content: Vec<Vec<char>>,
    // 记录每行终止位置的抽象索引
    innk: Vec<usize>,
}

// 基于游标的行为抽象
struct Travel {
    mode: TravelMode,

    content: Content,

    begin: usize,
    end: usize,
    now: usize,
}

impl Index<usize> for Content {
    type Output = char;

    fn index(&self, index: usize) -> &Self::Output {
        let line_index = self
            .index_belong(index)
            .expect("超出内容范围，来自Content.index()");
        match line_index {
            0 => &self.content[0][index],
            _ => {
                let char_index = index - self.content[line_index - 1].len();
                assert!(char_index < self.content[line_index].len());
                &self.content[line_index][char_index]
            }
        }
    }
}

impl Content {
    fn new() -> Self {
        Content {
            pn: Pano::On,
            content: Vec::new(),
            innk: Vec::new(),
        }
    }

    // 给定所属的行索引
    fn index_belong(&self, index: usize) -> Option<usize> {
        if self.innk.is_empty() {
            return None;
        }

        match self.innk.binary_search(&index) {
            Ok(pos) => Some(pos), // 精确匹配到某行末尾
            Err(insert_pos) => {
                if insert_pos == 0 {
                    Some(0) // 小于第一行末尾
                } else if insert_pos < self.innk.len() {
                    Some(insert_pos - 1) // 落在 insert_pos - 1 行内
                } else {
                    None // 超出最大索引
                }
            }
        }
    }

    pub fn read_file(&mut self, file_path: &str) {
        let file = std::fs::File::open(file_path).expect("无法打开文件");
        let reader = std::io::BufReader::new(file);
        let mut index = 0;
        for line in reader.lines() {
            let line = line.expect("无法读取行");
            index += line.len();
            self.content.push(line.chars().collect());
            self.innk.push(index);
        }
    }

    // 通过字索引获取行
    pub fn get_line_inside(&self, index: usize) -> Option<String> {
        let line_index = self.index_belong(index)?;
        let line_chars = &self.content[line_index];
        Some(line_chars.iter().collect())
    }

    // 通过行索引获取行
    pub fn get_line_outside(&self, index: usize) -> Option<&Vec<char>> {
        self.content.get(index)
    }

    // 获取多个连续词
    pub fn get_mul(&self, head: usize, tail: usize) -> Option<String> {
        let mut result = String::new();
        for idx in head..tail {
            result.push(self[idx])
        }
        Some(result)
    }

    // 根据类型差异划分token
    pub fn get_token(&self, index: usize) -> Option<String> {
        if index >= self.content.len() {
            return None;
        }

        if self[index] == ' ' {
            return None;
        }

        let type_memory = wordtype::word_type(self[index]);
        let mut result = String::new();

        for idx in index..self.content.len() {
            if wordtype::word_type(self[idx]) != type_memory {
                break;
            }
            result.push(self[idx]);
        }

        Some(result)
    }

    pub fn len(&self) -> usize {
        self.innk.last().copied().unwrap_or(0)
    }
}

impl Travel {
    fn new() -> Self {
        let content = Content::new();

        Travel {
            mode: TravelMode::Word,
            content,
            begin: 0,
            end: 0,
            now: 0,
        }
    }

    fn read_file(&mut self, file_path: &str) {
        self.content.read_file(file_path);
        self.end = self.content.len();
    }

    pub fn get_next(&mut self) -> Option<String> {
        match self.mode {
            TravelMode::Word => {
                let result = Some(self.content[self.now].to_string());
                self.now += 1;
                return result;
            }
            TravelMode::Line => {
                // 不可用
                let result = self
                    .content
                    .get_line_inside(self.now)
                    .map(|chars| chars.chars().collect());
                return result;
            }
            TravelMode::Token => self._get_token_(),
        }
    }

    fn _get_token_(&mut self) -> Option<String> {
        assert!(self.now < self.content.len());

        if self.content[self.now] == ' ' {
            return None;
        }

        let type_memory = wordtype::word_type(self.content[self.now]);
        let mut result = String::new();

        while self.now < self.content.len() {
            if wordtype::word_type(self.content[self.now]) != type_memory {
                break;
            }
            result.push(self.content[self.now]);
            self.now += 1;
        }

        Some(result)
    }
}
