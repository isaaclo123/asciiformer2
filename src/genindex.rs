use super::entities::{Entity, EntitySync};

use super::helpers::{unlock, wrap};
use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Clone)]
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

    pub fn alloc_index_len(&self) -> usize {
        self.alloc_indexes.len()
    }

    pub fn free_index(&mut self, index: usize) -> Result<(), &str> {
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

        Ok(())
    }

    pub fn free_index_len(&self) -> usize {
        self.free_indexes.len()
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
    // pub fn to_vec(&self) -> Vec<&T> {
    //     let mut result = Vec::new();

    //     for index in &self.alloc_indexes {
    //         let data_opt = self.data.get(*index).unwrap();
    //         if let Some(d) = data_opt {
    //             result.push(d);
    //         }
    //     }

    //     result
    // }
}

impl GenIndex<EntitySync> {
    pub fn alloc_entity(&mut self, entity: EntitySync) -> Result<usize, &str> {
        if self.free_indexes.len() <= 0 {
            return Err("No avaiable indexes");
        }

        let index = self.free_indexes.pop().unwrap();

        unlock(&entity).set_id(index as i16);
        self.alloc_indexes.push(index);
        self.data[index] = Some(entity);

        Ok(index)
    }
}

pub struct GenIndexIntoIterator<T: Clone> {
    gen_index: GenIndex<T>,
    index: usize,
}

impl<T: Clone> IntoIterator for GenIndex<T> {
    type Item = T;
    type IntoIter = GenIndexIntoIterator<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        GenIndexIntoIterator {
            gen_index: self,
            index: 0,
        }
    }
}

impl<T: Clone> Iterator for GenIndexIntoIterator<T> {
    // we will be counting with usize
    type Item = T;

    // next() is the only required method
    fn next(&mut self) -> Option<Self::Item> {
        // Increment our count. This is why we started at zero.
        // Check to see if we've finished counting or not.
        if self.index < self.gen_index.alloc_indexes.len() {
            let alloc_i = self.gen_index.alloc_indexes.get(self.index).unwrap();
            let data_result = self.gen_index.data.get(*alloc_i);

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

// `std::sync::MutexGuard<'_, genindex::GenIndex<std::sync::Arc<std::sync::Mutex<(dyn entities::entity::Entity + 'static)>>>>` is not an iterator

#[derive(Clone)]
pub struct GenIndexSync<T: Clone> {
    gen_index: Arc<Mutex<GenIndex<T>>>,
}

// impl<T: Clone> Clone for GenIndexSync<T> {
//     fn clone(&self) -> GenIndexSync<T> {
//         GenIndexSync {
//             gen_index: Arc::clone(&self.gen_index),
//         }
//     }
// }

impl<T: Clone> GenIndexSync<T> {
    pub fn new(max_size: usize) -> Self {
        Self {
            gen_index: wrap(GenIndex::new(max_size)),
        }
    }

    pub fn alloc_index(&mut self, val: T) -> Result<usize, &str> {
        let mut gen_index = unlock(&self.gen_index);
        let result = gen_index.alloc_index(val);

        match result {
            Ok(r) => Ok(r.clone()),
            Err(e) => Err("An Error Arose"),
        }
    }

    pub fn alloc_index_len(&self) -> usize {
        unlock(&self.gen_index).alloc_index_len()
    }

    pub fn free_index(&mut self, index: usize) -> Result<(), &str> {
        match unlock(&self.gen_index).free_index(index) {
            Ok(_) => Ok(()),
            Err(e) => Err("Issue arose"),
        }
    }

    pub fn free_index_len(&self) -> usize {
        unlock(&self.gen_index).free_index_len()
    }

    pub fn get(&self, index: usize) -> Option<T> {
        unlock(&self.gen_index).get(index)
    }
}

impl GenIndexSync<EntitySync> {
    pub fn alloc_entity(&mut self, entity: EntitySync) -> Result<usize, &str> {
        let mut gen_index = unlock(&self.gen_index);
        let id = gen_index.alloc_index(entity);

        match id {
            Ok(i) => {
                let entity = gen_index.get(i).unwrap();
                unlock(&entity).set_id(i as i16);
                Ok(i.clone())
            }
            Err(e) => Err("An Error Arose"),
        }
    }
}

impl<T: Clone> IntoIterator for GenIndexSync<T> {
    type Item = T;
    type IntoIter = GenIndexIntoIterator<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        unlock(&self.gen_index).clone().into_iter()
    }
}
