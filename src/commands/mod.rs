mod set_did;

pub trait Command {
    fn execute(&self);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_creation_is_possible() {
        use std::sync::Arc;
        use std::sync::Mutex;

        struct Command1 {
            cb: Box<Fn(String)>
        }

        impl Command for Command1 {
            fn execute(&self) {
                (self.cb)("Command1 result".to_string())
            }
        }

        let arc = Arc::new(Mutex::new("".to_string()));
        let arc2 = arc.clone();

        let command1 = Command1 {
            cb: Box::new(move |result| {
                let mut val = arc2.lock().unwrap();
                *val = result;
            })
        };

        command1.execute();

        assert_eq!("Command1 result", *arc.lock().unwrap());
    }
}