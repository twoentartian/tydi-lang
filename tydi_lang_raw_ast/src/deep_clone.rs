use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock};

pub trait DeepClone {
    fn deep_clone(&self) -> Self;
}

impl<T> DeepClone for Arc<RwLock<T>> where T: DeepClone+Clone {
    fn deep_clone(&self) -> Self {
        return Arc::new(RwLock::new(self.read().unwrap().deep_clone()));
    }
}

impl DeepClone for String {
    fn deep_clone(&self) -> Self {
        return self.clone();
    }
}

impl<T> DeepClone for Vec<T> where T: DeepClone {
    fn deep_clone(&self) -> Self {
        let mut output = vec![];
        for target in self {
            output.push(target.deep_clone());
        }
        output
    }
}

impl<T> DeepClone for Option<T> where T: DeepClone {
    fn deep_clone(&self) -> Self {
        return match self {
            None => { None }
            Some(target) => { Some(target.deep_clone()) }
        }
    }
}

impl <K,V> DeepClone for HashMap<K,V> where K : Eq + Hash + DeepClone + Clone, V: DeepClone + Clone {
    fn deep_clone(&self) -> Self {
        let mut output = HashMap::new();
        for (name, rela) in self {
            output.insert(name.deep_clone(), rela.deep_clone());
        }
        output
    }
}