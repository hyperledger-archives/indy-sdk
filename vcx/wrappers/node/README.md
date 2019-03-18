# VCX NodeJS Wrapper

This is a NodeJS wrapper for VCX library. 
VCX is the open-source library on top of Libindy which fully implements the credentials exchange.

**Note**: This library is currently in experimental state.

## Contribution Guide

Make sure you have these packages installed:

* StandardJS
* Typescript
* TSLint


Also this has a dependency on:
* libvcx debian
Because it creates a symlink (/usr/lib/libvcx.so) 

Run this commands before submitting your PR:

```
npm run lint
```

## Documentation:
 Run these commands:
```
npm install
npm ci
npm run doc-gen
```
* A directory will be created locally `./docs` which contains an `index.html` file which can be used to navigate the 
generated documents.

## Run Demo
- The demo represents example how 2 actors, **Alice** and **Faber** institution, exchange credentials. 
- They consult may consult Indy blockchain (pool of Indy nodes)  to find out certain pieces of information. **Faber** 
and **Alice** are represented by 2 scripts `faber.js` and `alice.js` but you could imagine that there's a webserver 
running code alike what's inside `faber.js` and there's a perhaps smartphone or laptop running code
alike iin `alice.js`.
- **Faber** and **Alice** in the demo also don't exchange the credentials peer to peer. Instead, the exchange happens 
through intermediary service represented by **Dummy Cloud Agent**. The data **Alice** and **Faber** are exchanging over 
**Dummy Cloud Agent** are however encrypted and cannot be read by the **Dummy Cloud Agent**. The **Dummy Cloud Agent** 
is something like illiterate postman. He'll take a letter from one party and delivers it to the other party. But he's 
unable to read the messages he's handling. 


### Pre-requirements
##### Libraries
Before you'll be able to run demo, you need to make sure you've compiled 
[`libindy`](https://github.com/hyperledger/indy-sdk/tree/master/libindy), 
[`libvcx`](https://github.com/hyperledger/indy-sdk/tree/master/vcx) and 
[`libnullpay`](https://github.com/hyperledger/indy-sdk/tree/master/libnullpay) libraries and are available on your 
system. 
#### Indy pool
You'll also have to run pool of Indy nodes on your machine. You can achieve by simply running a docker container
which encapsulates multiple interconnected Indy nodes. 
[Instructions here](https://github.com/hyperledger/indy-sdk#how-to-start-local-nodes-pool-with-docker).

### Steps to run demo
- Install NodeJS dependencies 
```
npm install
```

- Compile LibVCX Wrapper
```
npm run compile
```
- Start [Dummy Cloud Agent](../../dummy-cloud-agent)
- Run Faber agent, representing an institution
```
npm run demo:faber
```
- Give it a few seconds, then run Alice's agent which will connect with Faber's agent
```
npm run demo:alice
```

