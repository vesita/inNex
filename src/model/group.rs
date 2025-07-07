use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
pub struct Group<T> {
    the: Vec<T>,
    size: i64,
    fast: HashMap<String, i64>,  // ID -> index
    defast: HashMap<i64, String>,  // index -> ID
    free: VecDeque<i64>,          // 空闲索引队列
}

impl<T> Group<T> {
    // 创建新Group
    pub fn new() -> Self {
        Group {
            the: Vec::new(),
            size: 0,
            fast: HashMap::new(),
            defast: HashMap::new(),
            free: VecDeque::new(),
        }
    }

    // 注册元素到指定索引
    fn register_item(&mut self, id: String, index: i64) {
        // 检查ID和索引是否已存在
        if self.fast.contains_key(&id) || self.defast.contains_key(&index) {
            return;
        }
        
        self.fast.insert(id.clone(), index);
        self.defast.insert(index, id);
        self.size += 1;
    }

    // 通过ID注销元素
    fn unregister_item_by_id(&mut self, id: &str) -> bool {
        if !self.fast.contains_key(id) {
            return false;
        }
        
        let index = self.fast[id];
        self.fast.remove(id);
        self.defast.remove(&index);
        self.free.push_back(index);
        self.size -= 1;
        true
    }

    // 通过索引注销元素
    fn unregister_item_by_index(&mut self, index: i64) -> bool {
        if !self.defast.contains_key(&index) {
            return false;
        }
        
        let id = self.defast[&index].clone();
        self.defast.remove(&index);
        self.fast.remove(&id);
        self.free.push_back(index);
        self.size -= 1;
        true
    }

    // 缩短数组
    fn shorten(&mut self) {
        while self.size > 0 && !self.defast.contains_key(&(self.size - 1)) {
            self.size -= 1;
        }
        if self.size < self.the.len() as i64 {
            self.the.truncate(self.size as usize);
        }
    }

    // 获取空闲索引
    fn get_free(&mut self) -> i64 {
        while let Some(front) = self.free.front() {
            if front >= &self.size {
                self.free.pop_front();
            } else {
                return *front;
            }
        }
        -1
    }

    // 插入/覆盖元素
    pub fn put(&mut self, tar: &mut T) {
        let position = self.get_free();
        
        if position == -1 {
            let index = self.the.len() as i64;
            self.register_item(tar.id.clone(), index);
            self.the.push(tar.clone());
        } else {
            let index = position;
            self.free.pop_front();
            self.register_item(tar.id.clone(), index);
            self.the[index as usize] = tar.clone();
        }
    }

    // 提交新元素
    pub fn submit(&mut self, tar: &mut T) -> bool {
        if self.fast.contains_key(&tar.id) {
            false
        } else {
            self.put(tar);
            true
        }
    }

    // 通过ID获取元素拷贝
    pub fn get_by_id(&self, id: &str) -> T 
    where T: Clone {
        self.the[self.fast[id] as usize].clone()
    }

    // 通过索引获取元素拷贝
    pub fn get_by_index(&self, index: i64) -> T 
    where T: Clone {
        self.the[index as usize].clone()
    }

    // 取出并删除指定ID的元素
    pub fn take_by_id(&mut self, id: &str) -> T 
    where T: Clone {
        if !self.fast.contains_key(id) {
            return T::clone(&self.the[0]);  // 返回默认值
        }
        
        let result = self.get_by_id(id);
        let index = self.fast[id];
        self.free.push_back(index);
        self.unregister_item_by_id(id);
        result
    }

    // 取出并删除指定索引的元素
    pub fn take_by_index(&mut self, index: i64) -> T 
    where T: Clone {
        if index >= self.the.len() as i64 || !self.defast.contains_key(&index) {
            return T::clone(&self.the[0]);  // 返回默认值
        }
        
        let result = self.get_by_index(index);
        self.free.push_back(index);
        self.unregister_item_by_index(index);
        result
    }

    // 强制追加元素到末尾
    pub fn push(&mut self, tar: T) {
        let index = self.the.len() as i64;
        self.register_item(tar.id.clone(), index);
        self.the.push(tar);
    }

    // 获取最后一个有效元素（不删除）
    pub fn pick(&mut self) -> T 
    where T: Clone {
        debug_assert!(self.size > 0, "尝试获取空集合的元素");
        self.shorten();
        if self.size == 0 {
            return T::clone(&self.the[0]);
        }
        self.take_by_index(self.size - 1)
    }

    // 弹出最后一个有效元素
    pub fn pock(&mut self) {
        debug_assert!(self.size > 0, "尝试获取空集合的元素");
        debug_assert!(!self.the.is_empty(), "尝试获取空集合的元素");
        self.size = self.the.len() as i64 - 1;
        self.shorten();
    }

    // 获取空闲块数量
    pub fn free_size(&self) -> usize {
        self.free.len()
    }

    // 判断是否为空
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    // 获取元素数量
    pub fn size(&self) -> i64 {
        self.size
    }
}

// 实现索引访问（可变版本）
impl<T> std::ops::Index<&str> for Group<T> {
    type Output = T;
    
    fn index(&self, id: &str) -> &Self::Output {
        debug_assert!(self.fast.contains_key(id), "ID 不存在");
        &self.the[self.fast[id] as usize]
    }
}

impl<T> std::ops::Index<i64> for Group<T> {
    type Output = T;
    
    fn index(&self, index: i64) -> &Self::Output {
        debug_assert!(index < self.the.len() as i64, "访问的索引不存在");
        &self.the[index as usize]
    }
}

// 实现索引访问（不可变版本）
impl<T> std::ops::IndexMut<&str> for Group<T> {
    fn index_mut(&mut self, id: &str) -> &mut Self::Output {
        debug_assert!(self.fast.contains_key(id), "ID 不存在");
        let index = self.fast[id];
        &mut self.the[index as usize]
    }
}

impl<T> std::ops::IndexMut<i64> for Group<T> {
    fn index_mut(&mut self, index: i64) -> &mut Self::Output {
        debug_assert!(index < self.the.len() as i64, "访问的索引不存在");
        &mut self.the[index as usize]
    }
}