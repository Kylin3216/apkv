pub struct NamespaceStack {
    data: Vec<i32>,
    data_length: usize,
    count: i32,
    depth: i32,
}

impl NamespaceStack {
    pub fn new() -> NamespaceStack {
        NamespaceStack {
            data: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            data_length: 0,
            count: 0,
            depth: 0,
        }
    }
    pub fn get_current_count(&self) -> i32 {
        if self.data_length == 0 { 0 } else {
            let offset = self.data_length - 1;
            self.data[offset]
        }
    }
    pub fn get_accumulated_count(&self, mut depth: i32) -> i32 {
        if self.data_length == 0 || depth < 0 {
            0
        } else {
            if depth > self.depth {
                depth = self.depth;
            }
            let mut accumulated_count: i32 = 0;
            let mut offset: usize = 0;
            while depth != 0 {
                depth -= 1;
                let count = self.data[offset];
                accumulated_count += count;
                offset += 2 + (count as usize) * 2;
            }
            accumulated_count
        }
    }
    pub fn push(&mut self, prefix: i32, uri: i32) {
        if self.depth == 0 {
            self.increase_depth();
        }
        self.ensure_data_capacity(2);
        let offset = self.data_length - 1;
        let count = self.data[offset as usize];
        self.data[(offset - 1 - (count as usize) * 2)] = count + 1;
        self.data[offset] = prefix;
        self.data[offset + 1] = uri;
        self.data[offset + 2] = count + 1;
        self.data_length += 2;
        self.count += 1;
    }
    pub fn pop(&mut self) -> bool {
        if self.data_length == 0 { false } else {
            let mut offset = self.data_length - 1;
            let mut count = self.data[offset as usize];
            if count == 0 { false } else {
                count -= 1;
                offset -= 2;
                self.data[offset] = count;
                offset -= 1 + (count as usize) * 2;
                self.data[offset] = count;
                self.data_length -= 2;
                self.count -= 1;
                true
            }
        }
    }
    pub fn get_prefix(&self, index: i32) -> i32 {
        self.get(index, true)
    }
    pub fn get_uri(&self, index: i32) -> i32 {
        self.get(index, false)
    }
    pub fn find_prefix(&self, index: i32) -> i32 {
        self.find(index, false)
    }
    pub fn get_depth(&self) -> i32 {
        self.depth
    }
    pub fn increase_depth(&mut self) {
        self.ensure_data_capacity(2);
        let offset = self.data_length;
        self.data[offset] = 0;
        self.data[offset + 1] = 0;
        self.data_length += 2;
        self.depth += 1;
    }
    pub fn decrease_depth(&mut self) {
        if self.data_length != 0 {
            let offset = self.data_length - 1;
            let count = self.data[offset as usize];
            if (offset - 1 - (count as usize) * 2) != 0 {
                self.data_length -= 2 + (count as usize) * 2;
                self.count -= count;
                self.depth -= 1;
            }
        }
    }
    fn ensure_data_capacity(&mut self, capacity: usize) {
        let available = self.data.len() - self.data_length;
        if available > capacity { return; }
        let new_length = (self.data.len() + available) * 2;
        while self.data.len() < new_length {
            self.data.push(0)
        }
    }
    pub fn find(&self, index: i32, prefix: bool) -> i32 {
        if self.data_length == 0 { -1 } else {
            let mut offset: i32 = self.data_length as i32 - 1;
            let mut i = self.depth;
            while i != 0 {
                let mut count = self.data[offset as usize];
                offset -= 2;
                while count != 0 {
                    if prefix {
                        if self.data[offset as usize] == index {
                            return self.data[(offset as usize + 1)];
                        }
                    } else {
                        if self.data[(offset as usize + 1)] == index {
                            return self.data[offset as usize];
                        }
                    }
                    count -= 1;
                    offset -= 2;
                }
                i -= 1;
            }
            -1
        }
    }
    pub fn get(&self, mut index: i32, prefix: bool) -> i32 {
        if self.data_length == 0 || index < 0 { -1 } else {
            let mut offset: usize = 0;
            let mut i = self.depth;
            while i != 0 {
                i -= 1;
                let  count = self.data[offset] as i32;
                if index >= count {
                    index -= count;
                    offset += (2 + count * 2) as usize;
                    continue;
                }
                offset += (1 + index * 2) as usize;
                if !prefix {
                    offset += 1;
                }
                return self.data[offset] as i32;
            }
            -1
        }
    }
}