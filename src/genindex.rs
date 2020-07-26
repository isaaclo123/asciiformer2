pub struct GenIndex<T: Clone> {
    free_indexes: Vec<usize>,
    alloc_indexes: Vec<usize>,
    data: Vec<Option<T>>,
    max_size: usize,
}

impl<T: Clone> GenIndex<T> {
    pub fn new(max_size: usize) -> Self {
        Self {
            free_indexes: (0..max_size).into_iter().collect(),
            alloc_indexes: vec![],
            data: vec![None; max_size],
            max_size,
        }
    }

    pub fn alloc_index(&mut self, val: T) -> Result<usize, &str> {
        if self.free_indexes.len() <= 0 {
            return Err("No avaiable indexes");
        }

        let index = self.free_indexes.pop().unwrap();

        self.alloc_indexes.push(index);
        self.data[index] = Some(val);

        Ok(index)
    }

    pub fn free_index(&mut self, index: usize) -> Result<bool, &str> {
        if self.alloc_indexes.len() == 0 {
            return Err("No Indexes to free");
        }
        if self.data[index].is_none() {
            return Err("Index was not allocated");
        }

        self.data[index] = None;
        self.free_indexes.push(index);

        Ok(true)
    }

    pub fn get(&self, index: usize) -> &Option<T> {
        if index >= self.max_size {
            return &None;
        }
        let result = self.data.get(index);

        if let Some(r) = result {
            return r;
        }
        &None
    }

    // change to using iter later
    pub fn to_vec(&self) -> Vec<&T> {
        let mut result = Vec::new();

        for index in &self.alloc_indexes {
            let data_opt = self.data.get(*index).unwrap();
            if let Some(d) = data_opt {
                result.push(d);
            }
        }

        result
    }
}

// impl<T> IntoIterator for GenIndex<T> {
//     type Item = i32;
//     type IntoIter = std::vec::IntoIter<Self::Item>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         self.0.into_iter()
//     }
// }
