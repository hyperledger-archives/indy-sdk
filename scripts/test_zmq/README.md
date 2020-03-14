# Overview
This tool would be helpful for checking ZeroMQ connection from some machine to given validator's node.
It creates connection to given `zmq ip` and `zmq port` by using given `Target NYM` for calculating `CURVE_SERVERKEY`.
After creating connection, tool will try to send test message, like fake `LedgerStatus` and will except reply from server, 
that message was invalid. Because of ZeroMQ specific, we can't get information about connection status and can only introduce
some kind of assumptions. In this case, tool will try to send test's message for 3 times with 15 seconds time waiting (by default, this parameters can be changed)
and if will not be gotten any reply from server, then we can decide that there are ZeroMQ connection problems here.   

## Input parameters
### Required options here:
* `<Target NYM>` - it's Destination field from Node transaction.
* `<zmq ip>>` - it's IP address which is used by validator node for client's connection.
* `<zmq port>>` - it's port which is used by validator node for client's connection. 
### Optional parameters for debugging purposes:
* `--timeout` - it's used for response waiting for each tries. (15 seconds by default)
* `--tries-count` - it defines count of tries of test's message sending.

## Examples
Let use as example validator node from `BuilderNet` which named `riddleandcode` (seqNo: 44). This validator has the next params:
* dest: `5QDFnybgDHeQyBuaiKBsJ1o1Pxf83FNanaUPfRQp7N2d`
* zmq_ip: `109.70.97.20`
* zmq_port: `9780`

In this case command for checking would be looked like:
`check_zmq 5QDFnybgDHeQyBuaiKBsJ1o1Pxf83FNanaUPfRQp7N2d 109.70.97.20 9780`
and result in positive case:
```
Trying to connect to tcp://109.70.97.20:9780
Connection should be created
Successfully sent message: { "op": "LEDGER_STATUS", "txnSeqNo": 0, "merkleRoot": null, "ledgerId": 0, "ppSeqNo": null, "viewNo": null, "protocolVersion": 2}
Waiting for 15 seconds for getting reply from server
Got reply from validator
ZMQ CONNECTION IS POSSIBLE!!!
```

If, for example we change `zmp port` to `9785` (only for showing negative case), then output would be looked like:
```
Trying to connect to tcp://109.70.97.20:9785
Connection should be created
Successfully sent message: { "op": "LEDGER_STATUS", "txnSeqNo": 0, "merkleRoot": null, "ledgerId": 0, "ppSeqNo": null, "viewNo": null, "protocolVersion": 2}
Waiting for 15 seconds for getting reply from server
Make another try for checking
Waiting for 15 seconds for getting reply from server
Make another try for checking
Waiting for 15 seconds for getting reply from server
Make another try for checking
Error: Cannot connect to remote server
Looks like ZMQ connection to 109.70.97.20:9785 IS NOT POSSIBLE!!!
Don't panic.
Please check that address and port you provide are corrected.
Secondly, please check, that validator has exactly given dest: 5QDFnybgDHeQyBuaiKBsJ1o1Pxf83FNanaUPfRQp7N2d
Then, maybe you should to check firewall rules on validator's node, that can drop/reject incoming traffic

```

## Tool distribution.
There is binary of this tool for the next platforms:
* ubuntu 16.04
* ubuntu 18.04
* centos 7
* windows 10

Some remarks, regarding distributions. 
* On `Windows` zip archive is provided. All that you need is to unpack archive into some directory and call script from `cmd`
* On `Linux` we have assumption, that openssl package is installed. For most of Linux distributives this package is installed by default, but if not, that just install it and use `check_zmq` tool in a normal way.

All of binaries can be found and directly downloaded from:
`https://repo.sovrin.org/check/`
All the binaries are divided by platforms.
  
