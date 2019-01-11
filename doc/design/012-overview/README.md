# Overview

## Summary
At a high level the IndySDK is designed to be a software development kit that has many components to support and grow the Indy ecosystem. Two of the main functionalities that the IndySDK supports is communication with the Indy Ledger and formation of Indy Agents. At a deeper level, IndySDK contains a few libraries, but the primary focus of the IndySDK has been around libindy. Other additional libraries include vcx and libnullpay.

## libindy

Libindy is the primary library intended to serve as a foundation for all things Hyperledger Indy. The core functionality of the ledger includes functionality to:

* Use anonymous credentials
* Store arbitrary data in a wallet
* Core cryptographic functionality like encrypting and decrypting messages for other users
* Perform CRUD operations with both on ledger and pairwise (off-ledger) DIDs
* Communicate with the Indy Ledger
* Support payment functionality
* Support a standard wallet API to interact with different DBs

Since this library was written in rust with a C-Callable API layer, it's also supports easier porting to other languages that support calling a C interface such as Python, Java, Objective C, and Node.js.

At a high level the architecture has been designed in an extensible way to support the use of high level commands. A visual diagram of the architecture is provided below.

!(/indysdk-diagram.jpeg)

### Wrapper Layer

### API Layer

### Command Layer

### Service Layer

### Errors

### Domains

### Utils

## VCX

## libnullpay