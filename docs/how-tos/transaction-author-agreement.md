## Transaction Author Agreement

Due to legal nuances Indy network should support flow to receive explicit confirmation from any transaction author that he accepts the following reality. The ledger is public and immutable and by writing data to the ledger the user will not be able to exercise the right to be forgotten, so no personal data should be published to the ledger. For some instances this flow may be must have, but not required for other instances at all. So this functionality should be implemented on Indy side as optional.

Actors and Terms: 
* Transaction Author (TA) - Any person who would like to write any data to the ledger
* Transaction Author Agreement (TAA) - The text of agreement between network users and government
* Acceptance mechanism list (AML) - Description of the ways how the user may accept TAA

IndySDK provides API to:
* Build request for setting TAA and AML on the Ledger.
* Build request for getting TAA and AML set on the Ledger.
* Build request for disabling all TAAs on the Ledger.
* Append TAA acceptance data to request before signing and submitting of them to the Ledger.

### Libindy

#####  Trustee set up AML on the Ledger
```
acceptance_mechanisms_request = build_acceptance_mechanisms_request(.., '{"example label":"example description"}', "1.0", ...)
sign_and_submit_request(.., acceptance_mechanisms_request)
```

Please NOTE that AML should be set on the Ledger before setting a TAA.

##### Trustee set TAA on the Ledger

* Indy Node <= 1.12.0
    ```
    txn_author_agreement_request = build_txn_author_agreement_request(.., "example TAA text", "1.0", null, null)
    sign_and_submit_request(.., txn_author_agreement_request)
    ```

    Please note that `ratification_ts` and `retirement_ts` parameters must be omitted.

* Indy Node > 1.12.0
    ```
    txn_author_agreement_request = build_txn_author_agreement_request(.., "example TAA text", "1.0", 123456789, null)
    sign_and_submit_request(.., txn_author_agreement_request)
    ```
    Please note that:

    * `ratification_ts` must be specified in case of adding a new TAA
    * `retirement_ts` must be omitted in case of adding a new (latest) TAA.
    
##### Trustee change TAA on the Ledger
* Indy Node <= 1.12.0
    1. Send a new TAA:
    ```
    txn_author_agreement_request = build_txn_author_agreement_request(.., "new TAA text", "2.0", null, null)
    sign_and_submit_request(.., txn_author_agreement_request)
    ```
    Note that it is impossible to change an existing TAA. The new one must be create with different version.

* Indy Node > 1.12.0
    1. Send a new TAA:
    ```
    new_txn_author_agreement_request = build_txn_author_agreement_request(.., "new TAA text", "2.0", 345679890, null)
    sign_and_submit_request(.., new_txn_author_agreement_request)
    ```
    
    2. Update the current TAA to specify a retirement date:
    ```
    update_txn_author_agreement_request = build_txn_author_agreement_request(.., null, "1.0", null, 654321)
    sign_and_submit_request(.., update_txn_author_agreement_request)
    ```
    Please note that:
    
    * `retirement_ts` should be used for updating (deactivating) non-latest TAA on the ledger.
    * `ratification_ts` can be omitted in case of updating an existing TAA

##### Trustee disable TAA on the Ledger
* Indy Node <= 1.12.0
    ```
    txn_author_agreement_request = build_txn_author_agreement_request(.., "", "3.0", null, null)
    sign_and_submit_request(.., txn_author_agreement_request)
    ```
* Indy Node > 1.12.0
    ```
    disable_all_txn_author_agreements_request = indy_build_disable_all_txn_author_agreements_request(..)
    sign_and_submit_request(.., disable_all_txn_author_agreements_request)
    ```

##### Get AML and TAA set on the Ledger
1. Get AML set on the Ledger:
```
get_acceptance_mechanisms_request = build_get_acceptance_mechanisms_request(.., current_timestamp, ...)
response = submit_request(.., get_acceptance_mechanisms_request)
aml = response["result"]["data"]["aml"]
```
Result data is a map where keys are AML labels and values are their descriptions.    
You should choose one label for future usage. 

In our case:
- AML looks: `{"example label":"example description"}`.
- We will use `example label` label later to append TAA data to request.

2. Get TAA set on the Ledger:
```
get_txn_author_agreement_request = build_get_txn_author_agreement_request(..)
response = submit_request(.., txn_author_agreement_request)
text = response["result"]["data"]["text"] // example TAA text
version = response["result"]["data"]["version"] //1.0
```

We will use `text` and `version` later to append TAA data to request.
Otherwise you can calculate `digest` as sha256 hash on concatenated strings: `version` || `text`.
In our case `digest` is sha256(`1.0example TAA text`)

#### User send write transactions to the Ledger
```
nym_req = indy_build_nym_request(...)
time_of_acceptance = get_current_timestamp
nym_req = indy_append_txn_author_agreement_acceptance_to_request(nym_req, null, null, "TAA digest", "example label", time_of_acceptance)
indy_sign_and_submit_request(.., nym_req)
```

**Note**: Instead of "TAA digest" parameter (4th), the plain text of the agreement and the version can be specified (2nd and 3rd parameters). For more details please see API documentation.

**Note**: The flow for sending of Payment transaction is different because plugin does signing internal.
You must pass TAA data as part of `extra` parameter.
```
extra = indy_prepare_payment_extra_with_acceptance_data(..., "example TAA text", "1.0", null, "example label", time_of_acceptance)
payment_req = indy_build_payment_req(..., extra)
submit_request(.., payment_req)
```

### Indy-CLI

CLI uses session-based approach to work with Transaction Author Agreement.

##### TAA setup workflow
* `ledger txn-acceptance-mechanisms` - send TAA Acceptance Mechanisms to the Ledger

    Example: `ledger txn-acceptance-mechanisms aml={"Click Agreement":"some description"} version=1` 
    
* `ledger txn-author-agreement` - send Transaction Author Agreement to the Ledger. 

    Example: `ledger txn-author-agreement text="Indy transaction agreement" version=1` 
    
##### TAA acceptance workflow
1. On CLI start: Pass a config JSON file containing a label of mechanism how a user is going to accept a transaction author agreement.
    `indy-cli --config <path-to-config-json-file>` where config looks like:
    ```
    {
      "taaAcceptanceMechanism": "Click Agreement",
      .....
    }
    ```
    The list of available acceptance mechanisms can be received by calling `ledger get-acceptance-mechanisms` command.
1. On `pool connect` command execution: User will be asked if he would like to accept TAA.
User either can accept it or skip and accept it later by `pool show-taa` command.
    
    Note that accepting TAA one time on `pool connect` or `pool show-taa` is a CLI-specific behavior that actually caches value for future uses.
    Actual TAA check will be performed on write requests sending only.

1. On write request sending to Ledger: TAA will be appended to requests.

### Libvcx

VCX stores TAA data into internal settings which defines library behaviour and use it write transaction sending.

There are two ways of setting TAA:

1. using `vcx_init_with_config` function.
This function is some kind of entry point which initializes library. 
It accepts `config` as a parameter, does it processing and stores some values into the internal state for future needs.
So, you can append the TAA data to this config:
```
{ 
 ...., 
 "author_agreement": "{
    \"taaDigest\": \"string\", 
    \"acceptanceMechanismType\":\ "string\", 
    \"timeOfAcceptance\": u64}" 
 },
 ...
}
```
This TAA data will be used for sending write transactions to Ledger. 
You either can specify `taaDigest` or combination of `text: "string"` and `version: "string"` fields.

Please NOTE that vcx config values must be represented as strings.

2. using `vcx_set_active_txn_author_agreement_meta` function at any time after initialization 
This function set transaction author agreement data as active (or change) which will be used for sending write transactions to Ledger. 


Use `vcx_get_ledger_author_agreement` function to get author agreement and acceptance mechanisms set on the Ledger.
