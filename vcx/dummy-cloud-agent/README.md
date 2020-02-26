# Dummy Cloud Agent

This is simple implementation of **VCX compatible Cloud Agent with Aries support**. 

The main purpose of this implementation is VCX testing, demos and documentation of VCX protocol. 

## Build VCX Agency
The most common reason for extreme slowness in Rust is building in the default, debug mode. Make sure to run 
with `--release`.

    cargo build --release
    
## Run VCX Agency 
    cargo run <path-to-config>
    
Two sample configs are provided in [./config](./config) directory.

## Run with PostgreSQL wallet

The agency is by default using IndySDK builtin wallet stored in file. To run with higher stability and performance,
run agency with postgresql wallet. See sample postgresql [configuration](config/pgsql-config.json). More details
on using postgresql [here](docs/postgres-wallet.md)   

## Using Admin API
When troubleshooting, it's handy to be able quickly find out more information about the state of agency and the 
entities in it. For that, you enable "Admin API" in agency configuration and query the Agency via HTTP. More details 
[here](./docs/admin-api.md).s
    
# What is agency?
Cloud agency is a little bit like mail server, but secure. It can receive messages on your behalf and you can download
your messages on demand. You can also use it to route messages to other recipients and it knows how to do that. 

Using email analogy, you first need to create an email, some sort of account to be able to do anything. So this is the
typical agency protocol flow:
1. Create `Forward Agent Connection`. This sort of like initial pairwise connection with agency 
(exchange unique public keys with each other).
2. Create `Agent`. The `Agent` will have its own identity with DID and public key. Agent is sort of like your email account.
Because agency is (hopefully) always online, it can receive messages from outside word into your cloud agent.  
3. Create `Agent Connections` for your cloud agent. These represent unique relationship you have with other agents from
outside word. 

# How is implemented
You can read more about how agency works [here](./docs/architecture.md).

