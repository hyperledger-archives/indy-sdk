use libindy::IndyHandle;

use std::cell::RefCell;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct ApplicationContext {
    main_prompt: RefCell<String>,
    sub_prompts: RefCell<BTreeMap<usize, String>>,
    is_exit: RefCell<bool>,
}

impl ApplicationContext {
    pub fn new() -> ApplicationContext {
        ApplicationContext {
            main_prompt: RefCell::new("indy".to_owned()),
            sub_prompts: RefCell::new(BTreeMap::new()),
            is_exit: RefCell::new(false),
        }
    }

    pub fn set_main_prompt(&self, prompt: &str) {
        *self.main_prompt.borrow_mut() = prompt.to_owned();
    }

    pub fn set_sub_prompt(&self, pos: usize, value: &str) {
        self.sub_prompts.borrow_mut().insert(pos, value.to_owned());
    }

    pub fn unset_sub_prompt(&self, pos: usize) {
        self.sub_prompts.borrow_mut().remove(&pos);
    }

    pub fn get_prompt(&self) -> String {
        let mut prompt = String::new();

        for (_key, value) in self.sub_prompts.borrow().iter() {
            prompt.push_str(value);
            prompt.push_str(":");
        }

        prompt.push_str(&self.main_prompt.borrow());
        prompt.push_str("> ");
        prompt
    }

    pub fn set_exit(&self) {
        *self.is_exit.borrow_mut() = true;
    }

    pub fn is_exit(&self) -> bool {
        *self.is_exit.borrow()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn new_works() {
        let cnxt = ApplicationContext::new();
        assert_eq!("indy> ", &cnxt.get_prompt());
        assert_eq!(false, cnxt.is_exit());
    }

    #[test]
    pub fn set_main_prompt_works() {
        let cnxt = ApplicationContext::new();
        cnxt.set_main_prompt("main_prompt");
        assert_eq!("main_prompt> ", &cnxt.get_prompt());
        assert_eq!(false, cnxt.is_exit());
    }

    #[test]
    pub fn set_sub_prompt_works() {
        let cnxt = ApplicationContext::new();

        cnxt.set_main_prompt("main_prompt");
        cnxt.set_sub_prompt(1, "sub_prompt1");
        cnxt.set_sub_prompt(3, "sub_prompt3");
        cnxt.set_sub_prompt(2, "sub_prompt2");

        assert_eq!("sub_prompt1:sub_prompt2:sub_prompt3:main_prompt> ", &cnxt.get_prompt());
        assert_eq!(false, cnxt.is_exit());
    }

    #[test]
    pub fn unset_sub_prompt_works() {
        let cnxt = ApplicationContext::new();

        cnxt.set_main_prompt("main_prompt");
        cnxt.set_sub_prompt(1, "sub_prompt1");
        cnxt.set_sub_prompt(3, "sub_prompt3");
        cnxt.set_sub_prompt(2, "sub_prompt2");

        cnxt.unset_sub_prompt(2);

        assert_eq!("sub_prompt1:sub_prompt3:main_prompt> ", &cnxt.get_prompt());
        assert_eq!(false, cnxt.is_exit());
    }

    #[test]
    pub fn set_exit_works() {
        let cnxt = ApplicationContext::new();

        cnxt.set_exit();

        assert_eq!("indy> ", &cnxt.get_prompt());
        assert_eq!(true, cnxt.is_exit());
    }
}
