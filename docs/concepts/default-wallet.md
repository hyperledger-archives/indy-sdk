# Indy-SDK Default Wallet Implementation
The purpose of this implementation is to provide a default encrypted wallet for **indy-sdk**.

The **indy-sdk** default wallet implementation uses hardened version of [SQLCipher](https://www.zetetic.net/sqlcipher/):

* HMAC-SHA256 instead of HMAC-SHA1.
* PBKDF2 100K rounds for passphrase key data instead of 64K.
* PBKDF2 10 rounds for HMAC key derivation instead of 2.
* Page size 2K instead of 1K.

## Key Credentials
The default wallet allows an optional passphrase to used for encrypting the data.
If no passphrase is provided either by leaving the key blank or omitting the key, the wallet will not be encrypted but stored in [SQLite3](https://www.sqlite.org/index.html) format.
The passphrase to open the wallet is stored outside of **indy-sdk** and is left to the consumer's security preference such as HSMs, TEEs, or offline methods.


**indy-sdk** supports a JSON parameter, *credentials*, for opening or creating a wallet:

```
{
   "key": "<passphrase>"
   "rekey": "<passphrase>"
}
```

If the *credentials* parameter is omitted or if *key* is an empty string, the wallet is left unencrypted.

*key* is the passphrase for opening the wallet and will be run through 100K rounds of PBKDF2.

If *rekey* is provided, the wallet will be opened using *key* and change the passphrase to the *rekey* value for future open calls.
*rekey* is only required for an existing wallet and throws an error when attempting to create a new wallet.

If *rekey* is included with [_null_, _""_], the wallet will be decrypted.

If *key* is [_null_, _""_] and *rekey* contains a non-empty value, the wallet will now be encrypted.

#### NOTE
*rekey* is only necessary when changing *key*. Otherwise it should be omitted.

## Cases 

### Normal wallet / No passphrase
To create a non-encrypted wallet, *credentials* can be empty or not specified. *key* may also be [_null_, _""_].

### Encrypted wallet / A Key
Encrypted wallets require a passphrase to be specified in the *key* field.
Passphrase can be any non-blank value.

```
{
   "key": "Th1sIsArEALLY$3cuR3PassW0RD"
}
```

### Normal wallet to encrypted wallet / Adding a Key
To add encryption to an existing non-encrypted wallet, *key* must be set to [_null_, _""_] and *rekey* must be set to a valid passphrase.
Then wallet open calls are the same as the *Encrypted Wallet* section.

```
{
   "key": null,
   "rekey": "il0V3MyN3WpA$SworD"
}
```

### Encrypted wallet to normal wallet / Removing a Key
To remove encryption from an existing encrypted wallet, *key* must be set to the current value and *rekey* must be set to a blank value [_null_, _""_].
Then wallet open calls are the same as the *Normal wallet* section.

```
{
   "key": "Th1sIsArEALLY$3cuR3PassW0RD",
   "rekey": null
}
```

### Updating passphrase / Changing a Key
Rotating wallet passphrases is recommended. *key* must be set to the current value and *rekey* must be set to the new value.
Then wallet open calls are the same as the *Encrypted wallet* section.

```
{
   "key": "Th1sIsArEALLY$3cuR3PassW0RD",
   "rekey": "s8c0R31tYi$hARd"
}
```
