extern crate rand;

use rand::Rng;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::collections::HashMap;
use std::ops::Deref;
use std::ops::DerefMut;
use utils::error;

pub struct ObjectCache<T>{
    store: Mutex<HashMap<u32, Mutex<T>>>,
}

impl<T> Default for ObjectCache<T> {
    fn default() -> ObjectCache<T>
    {
        ObjectCache {
            store: Default::default()
        }
    }
}

impl<T> ObjectCache<T> {

    fn _lock_store(&self) -> Result<MutexGuard<HashMap<u32, Mutex<T>>>, u32> {
        match self.store.lock() {
            Ok(g) => Ok(g),
            Err(e) => {
                error!("Unable to lock Object Store: {:?}", e);
                Err(10)
            }
        }
    }

    pub fn has_handle(&self, handle: u32) -> bool {
        let store = match self._lock_store() {
            Ok(g) => g,
            Err(_) => return false
        };
        store.contains_key(&handle)
    }

    pub fn get<F,R>(&self, handle:u32, closure: F) -> Result<R,u32>
        where F: Fn(&T) -> Result<R,u32> {

        let store = self._lock_store()?;
        match store.get(&handle) {
            Some(m) => match m.lock() {
                Ok(obj) => closure(obj.deref()),
                Err(err) => return Err(10) //TODO better error
            },
            None => return Err(error::INVALID_OBJ_HANDLE.code_num)
        }
    }

    pub fn get_mut<F, R>(&self, handle:u32, closure: F) -> Result<R,u32>
        where F: Fn(&mut T) -> Result<R,u32> {

        let mut store = self._lock_store()?;
        match store.get_mut(&handle) {
            Some(m) => match m.lock() {
                Ok(mut obj) => closure(obj.deref_mut()),
                Err(err) => return Err(10) //TODO better error
            },
            None => return Err(error::INVALID_OBJ_HANDLE.code_num)
        }
    }

    pub fn add(&self, obj:T) -> Result<u32, u32> {
        let mut store = self._lock_store()?;

        let mut new_handle = rand::thread_rng().gen::<u32>();
        loop {
            if !store.contains_key(&new_handle){
                break;
            }
            new_handle = rand::thread_rng().gen::<u32>();
        }

        match store.insert(new_handle, Mutex::new(obj)){
            Some(_) => Ok(new_handle),
            None => Ok(new_handle)
        }
    }

    pub fn release(&self, handle:u32) -> Result<(),u32> {
        let mut store = self._lock_store()?;
        match store.remove(&handle) {
            Some(_) => Ok(()),
            None => Err(error::INVALID_OBJ_HANDLE.code_num)
        }
    }

    pub fn drain(&self) -> Result<(), u32> {
        let mut store = self._lock_store()?;
        Ok(store.clear())
    }
}

#[cfg(test)]
mod tests{
    use object_cache::ObjectCache;

    #[test]
    fn create_test(){
        let c:ObjectCache<u32> = Default::default();
    }

    #[test]
    fn get_closure(){
        let test:ObjectCache<u32> = Default::default();
        let handle = test.add(2222).unwrap();
        let rtn = test.get(handle, |obj| Ok(obj.clone()));
        assert_eq!(2222, rtn.unwrap())
    }


    #[test]
    fn to_string_test() {
        let test:ObjectCache<u32> = Default::default();
        let handle = test.add(2222).unwrap();
        let string: String = test.get(handle, |obj|{
           Ok(String::from("TEST"))
        }).unwrap();

        assert_eq!("TEST", string);

    }

    fn mut_object_test(){
        let test:ObjectCache<String> = Default::default();
        let handle = test.add(String::from("TEST")).unwrap();

        test.get_mut(handle, |obj|{
            obj.to_lowercase();
            Ok(())
        }).unwrap();

        let string: String = test.get(handle, |obj|{
            Ok(obj.clone())
        }).unwrap();

        assert_eq!("test", string);
    }

}
