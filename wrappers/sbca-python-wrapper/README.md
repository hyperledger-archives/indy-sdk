![logo2](./images/sbc-banner.png)

#   SBCA-Indy-Wrapper
>   [blockchain.swisscom.com](https://blockchain.swisscom.com/)


This project is a custom python wrapper for Hyperledger's Libindy library that aims to optimize and remove redundant code of the existing wrapper. We are activly using this wrapper in our uppcoming Hyperledger Indy project "Cloud Authenticator" where we gona provide a Solution of Identity Management based on Indy.


##  Setup
### Requirements
*   **Python 3.6 or greater**
*   **Libindy 1.8.1** ([Installation Instructions](https://github.com/hyperledger/indy-sdk#installing-the-sdk))
    *   The version of the wrapper and the Libindy library should always match!

### Installation
The wrapper is an installable pip-package. Download and install it by running the following command:
```bash
pip install sbca-indy-wrapper @ git+https://github.com/swisscom-blockchain/sbca-indy-wrapper.git@v1.8.1-pre
```

##  Usage

```python
from sbca_wrapper import Anoncreds, BlobStorage, Crypto, DID, Ledger, NonSecrets, Pairwise, Payment, Pool, Wallet
```

### Examples:
> Create Wallet
```python
from sbca_wrapper import  Wallet

# TODO: Check if it Work
await Wallet.create(parameters)

```

> Create DID
```python
from sbca_wrapper import  DID

# TODO: Check if it Work
await DID.create(parameters)

```

> Add new Libindy-Calls
```python
@staticmethod
    @libindy_command('my_custom_libindy_call')
    async def my_function_name(wallet_handle: int, signing_did: str,
                                 did_json: Union[dict, str]) -> str:
        pass
```

## Benefits of this Wrapper
*   No or less redundant Code than the current wrapper
*   Actual dict/list returns instead of strings
*   Easy to implement new Commands from Libindy

## Downsides of this Wrapper
*   Need of a deep understanding of Python 3+ to maintain
*   No Tests and minimal documentation

## Last Words
We dont expect that our wrapper will replace the current one. But we hope that you guys can maybe take some points and merge / implement them. 0


##  Authors
**Lead Development**
*   Roth Jeremy ([Skilletpan](https://github.com/Skilletpan))

**Additional Development**
*   Krell Jérôme ([JeromeK13](https://github.com/JeromeK13))

**Acknowledgments**
*   Alvarado Flores Jorge([alvaradojl](https://github.com/alvaradojl))
*   Riva Luigi([lrscbc](https://github.com/lrscbc))
