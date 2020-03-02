use rand::Rng;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::collections::HashMap;
use std::ops::Deref;
use std::ops::DerefMut;

use error::prelude::*;

pub struct ObjectCache<T> {
    pub store: Mutex<HashMap<u32, Mutex<T>>>,
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
    fn _lock_store(&self) -> VcxResult<MutexGuard<HashMap<u32, Mutex<T>>>> {
        match self.store.lock() {
            Ok(g) => Ok(g),
            Err(e) => {
                error!("Unable to lock Object Store: {:?}", e);
                Err(VcxError::from_msg(VcxErrorKind::Common(10), format!("Unable to lock Object Store: {:?}", e)))
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

    pub fn get<F, R>(&self, handle: u32, closure: F) -> VcxResult<R>
        where F: Fn(&T) -> VcxResult<R> {
        let store = self._lock_store()?;
        match store.get(&handle) {
            Some(m) => match m.lock() {
                Ok(obj) => closure(obj.deref()),
                Err(_) => Err(VcxError::from_msg(VcxErrorKind::Common(10), "Unable to lock Object Store")) //TODO better error
            },
            None => Err(VcxError::from_msg(VcxErrorKind::InvalidHandle, format!("Object not found for handle: {}", handle)))
        }
    }

    pub fn get_mut<F, R>(&self, handle: u32, closure: F) -> VcxResult<R>
        where F: Fn(&mut T) -> VcxResult<R> {
        let mut store = self._lock_store()?;
        match store.get_mut(&handle) {
            Some(m) => match m.lock() {
                Ok(mut obj) => closure(obj.deref_mut()),
                Err(_) => Err(VcxError::from_msg(VcxErrorKind::Common(10), "Unable to lock Object Store")) //TODO better error
            },
            None => Err(VcxError::from_msg(VcxErrorKind::InvalidHandle, format!("Object not found for handle: {}", handle)))
        }
    }

    pub fn add(&self, obj: T) -> VcxResult<u32> {
        let mut store = self._lock_store()?;

        let mut new_handle = rand::thread_rng().gen::<u32>();
        loop {
            if !store.contains_key(&new_handle) {
                break;
            }
            new_handle = rand::thread_rng().gen::<u32>();
        }

        match store.insert(new_handle, Mutex::new(obj)) {
            Some(_) => Ok(new_handle),
            None => Ok(new_handle)
        }
    }

    pub fn insert(&self, handle: u32, obj: T) -> VcxResult<()> {
        let mut store = self._lock_store()?;

        match store.insert(handle, Mutex::new(obj)) {
            _ => Ok(()),
        }
    }

    pub fn release(&self, handle: u32) -> VcxResult<()> {
        let mut store = self._lock_store()?;
        match store.remove(&handle) {
            Some(_) => Ok(()),
            None => Err(VcxError::from_msg(VcxErrorKind::InvalidHandle, format!("Object not found for handle: {}", handle)))
        }
    }

    pub fn drain(&self) -> VcxResult<()> {
        let mut store = self._lock_store()?;
        Ok(store.clear())
    }
}

#[cfg(test)]
mod tests {
    use object_cache::ObjectCache;
    use utils::devsetup::SetupDefaults;

    #[test]
    fn create_test() {
        let _setup = SetupDefaults::init();

        let _c: ObjectCache<u32> = Default::default();
    }

    #[test]
    fn get_closure() {
        let _setup = SetupDefaults::init();

        let test: ObjectCache<u32> = Default::default();
        let handle = test.add(2222).unwrap();
        let rtn = test.get(handle, |obj| Ok(obj.clone()));
        assert_eq!(2222, rtn.unwrap())
    }

    #[test]
    fn to_string_test() {
        let _setup = SetupDefaults::init();

        let test: ObjectCache<u32> = Default::default();
        let handle = test.add(2222).unwrap();
        let string: String = test.get(handle, |_| {
            Ok(String::from("TEST"))
        }).unwrap();

        assert_eq!("TEST", string);
    }

    #[test]
    fn mut_object_test() {
        let _setup = SetupDefaults::init();

        let test: ObjectCache<String> = Default::default();
        let handle = test.add(String::from("TEST")).unwrap();

        test.get_mut(handle, |obj| {
            obj.to_lowercase();
            Ok(())
        }).unwrap();

        let string: String = test.get(handle, |obj| {
            Ok(obj.clone())
        }).unwrap();

        assert_eq!("TEST", string);
    }
}
