// Create the Error, ErrorKind, ResultExt, and Result types
error_chain! {
    links {
        Indy(::indy::errors::Error, ::indy::errors::ErrorKind);
    }
}

