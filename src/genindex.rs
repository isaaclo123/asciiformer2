pub struct GenIndex<T: Clone> {
    free_indexes: Vec<usize>,
    alloc_indexes: Vec<usize>,
    data: Vec<Option<T>>,
    max_size: usize,
    index: usize,
}

impl<T: Clone> GenIndex<T> {
    pub fn new(max_size: usize) -> Self {
        Self {
            free_indexes: (0..max_size).into_iter().collect(),
            alloc_indexes: vec![],
            data: vec![None; max_size],
            max_size,
            index: 0,
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

        let remove_index = self.alloc_indexes.iter().position(|x| *x == index);

        if let Some(i) = remove_index {
            self.alloc_indexes.remove(i);
        }

        self.data[index] = None;
        self.free_indexes.push(index);

        Ok(true)
    }

    pub fn get(&self, index: usize) -> Option<T> {
        if index >= self.max_size {
            return None;
        }
        let result = self.data.get(index);

        if let Some(r) = result {
            if let Some(x) = r {
                return Some(x.clone());
            }
        }
        None
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

impl<T: Clone> Iterator for GenIndex<T> {
    // we will be counting with usize
    type Item = T;

    // next() is the only required method
    fn next(&mut self) -> Option<Self::Item> {
        // Increment our count. This is why we started at zero.
        // Check to see if we've finished counting or not.
        if self.index < self.alloc_indexes.len() {
            let alloc_i = self.alloc_indexes.get(self.index).unwrap();
            let data_result = self.data.get(*alloc_i);

            if let Some(r) = data_result {
                if let Some(x) = r {
                    self.index += 1;
                    return Some(x.clone());
                }
            }
        }

        self.index = 0;
        None
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
