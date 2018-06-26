# Payment Interface

This design proposes the list of commands to Indy CLI to handle payments.

## Goals and ideas

* Indy CLI should provide ability to perform the main payments operations:
  * Creation of payment address
  * Listing of payment addresses
  * Getting list of UTXO for payment address
  * Sending payment transaction
  * Adding fees to transactions
  * Getting transactions fees amount
* Abstraction level should correspond to Indy SDK. For example, don't hide UTXO abstraction. In the future we can add
  new commands to increase abstractions level.

## New CLI commands

### Create payment address

Create payment address for specific payment method in the wallet

```indy-cli
indy> payment-address create payment_method=<payment-method> seed=[<seed>]
```

Returns:

* Success or error message

### List payment addresses

List payment addresses in the wallet

```indy-cli
indy> payment-address list
```

Returns:

* Table with columns: Payment Address, Payment Method

### Send GET_UTXO request

Send request to get list of UTXO for specified payment addresses

```indy-cli
indy> ledger get-utxo payment_address=<payment-address>
```

Returns:

* Table with columns: Txo, Payment Address, Amount, Extra

### Send PAYMENT transaction

Send payment transaction

```indy-cli
indy> ledger payment inputs=<utxo-1>,..,<utxo-n> outputs=(<pay-addr-0>,<amount>,<extra>),..,(<pay-addr-n>,<amount>,<extra>)
```

Returns:

* Success or error message

Note that "utxo-n" is identifier presented in "Input" column of ```ledger get-utxo``` command output

### Send GET_FEES request

Send request to get fees amount for ledger transactions

```indy-cli
indy> ledger get-fees payment_method=<payment_method>
```

Returns:

* Table with columns: Transaction, Amount

### Prepare MINT transaction

Prepare MINT transaction as json.

```indy-cli
indy> ledger mint-prepare outputs=(<pay-addr-0>,<amount-0>,<extra-0>),..,(<pay-addr-n>,<amount-n>,<extra-n>)
```

Returns:

* MINT transaction json

Sending MINT process is the following:

* Steward 1 calls ```ledger mint-prepare```
* Signs it by calling ```ledger sign-multi```
* Sends the request json to Steward 2 (now we have 1 signature)
* Second Steward signs it by calling ```ledger sign-multi```
* Sends the request json to Steward 3 (now we have 2 signature)
* All Stewards sign the request
* The latest Steward calls ```ledger send-custom``` to send request signed by all Stewards

### Prepare SET_FEES transaction

Prepare SET_FEES transaction as json.

```indy-cli
indy> ledger set-fees-prepare payment_method=<payment_method> fees=<txn-type-1>:<amount-1>,...,<txn-type-n>:<amount-n>
```

Returns:

* SET_FEES transaction json

Sending SET_FEES process is the following:

* Steward 1 calls ```ledger set-fees-prepare```
* Signs it by calling ```ledger sign-multi```
* Sends the request json to Steward 2 (now we have 1 signature)
* Second Steward signs it by calling ```ledger sign-multi```
* Sends the request json to Steward 3 (now we have 2 signature)
* All Stewards sign the request
* The latest Steward calls ```ledger send-custom``` to send request signed by all Stewards

### Sign the transaction (for multi-sign case)

Add signature (for multi-sign case) by current DID to transaction json.

```indy-cli
indy> ledger sign-multi txn=<txn-json>
```

Returns:

* Transaction json with added signature

## Existing commands update

All commands to send domain transactions require new optional params to add transactions fees:

```indy-cli
[fees_inputs=<utxo-1>,..,<utxo-n>] [fees_outputs=(<pay-addr-0>,<amount>,<extra>),..,(<pay-addr-n>,<amount>,<extra>)]
```

Note that "utxo-n" is identifier presented in "Input" column of ```ledger get-utxo``` command output