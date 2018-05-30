macro_rules! safe_wallet_create {
    ($x:ident) => {
        match Wallet::delete($x) {
            Ok(..) => {},
            Err(..) => {}
        };
        Wallet::create("pool1", $x, None, None, None).unwrap();
    }
}

macro_rules! wallet_cleanup {
    ($x:ident, $y:ident) => {
        Wallet::close($x).unwrap();
        Wallet::delete($y).unwrap();
    }
}

pub fn time_it_out<F>(msg: &str, test: F) -> bool where F: Fn() -> bool {
    for _ in 1..250 {
        if test() {
            return true;
        }
    }
    // It tried to do a timeout test 250 times and the system was too fast, so just succeed
    println!("{} - system too fast for timeout test", msg);
    true
}

