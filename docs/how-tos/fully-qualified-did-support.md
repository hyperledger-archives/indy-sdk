## Fully-Qualified identifiers

General format of fully-qualified identifier is `<prefix>:<method>:<value>`.
* Prefix: specifies entity type:
    * `did` - DID
    * `schema` - Schema Id
    * `creddef` - Credential Definition Id
    * `revreg` - Revocation registry Id
* Method: specifies the network this entity belongs to.
* Value: the main part of identifier.

### Libindy

##### Creation

* use `indy_create_and_store_my_did` function with specifying of `method_name` field inside `did_info` parameter to create fully qualified DID. 
    ```
    (did, verkey) = indy_create_and_store_my_did(..., '{"method_name": "indy"}')
    ```

* use `indy_qualify_did` function to update DID stored in the wallet to make it fully qualified, or to do other DID maintenance.
This functions also updates all DID related entities stored in the wallet to point on new identifier.
    ```
    fully_qualified_did = indy_qualify_did(did)
    ```

Every time you use fully-qualified DID to create a dependent entity like `Schema`, `Credential Definition` or `Revocation Registry Definition` they will have fully qualified form also.   

#### Anoncreds workflow

As we have released Fully-Qualified identifiers, we can work with both identifier formats in a compatible way. 
There are some cases when Fully-Qualified entity must be converted to Unqualified form.
You can use `indy_to_unqualified` function for these cases.

This function can accept the following entities: 
* DID
* SchemaId 
* CredentialDefinitionId 
* RevocationRegistryId 
* Schema
* CredentialDefinition
* RevocationRegistryDefinition
* CredentialOffer
* CredentialRequest
* ProofRequest

Examples:
* DID:  
    ```
    indy_to_unqualified("did:sov:NcYxiDXkpYi6ov5FcYDi1e") == "NcYxiDXkpYi6ov5FcYDi1e"
    ```
* SchemaId:
    ```
    indy
    ```
* CredentialDefinitionId:
    ```
    indy_to_unqualified("creddef:sov:did:sov:NcYxiDXkpYi6ov5FcYDi1e:3:CL:schema:sov:did:sov:NcYxiDXkpYi6ov5FcYDi1e:2:gvt:1.0:tag") == "NcYxiDXkpYi6ov5FcYDi1e:3:CL:NcYxiDXkpYi6ov5FcYDi1e:2:gvt:1.0:tag"
    ```


Let's consider Credential Issuance and Proof Presentation for different cases.

* FQ - fully-qualified
* U - unqualified

##### Credential Issuance

* Issuer (FQ) - Holder (U) 
    * Issuer creates DID in the fully qualified way. 
      Schema and CredentialDefinition created based on this DID will be fully-qualified also.
      There are two cases here:
        * Issuer use Ledger to publish related transactions - these entities will be unqualified automatically. So, Prover and Verifier will get unqualified form from the Ledger.
        * Issuer don't use Ledger - he must call `indy_to_unqualified` with `cred_offer_json` to get unqualified form.
        
    * Issuer creates Credential Offer for Holder using fully-qualified Credential Definition Id. 
      The next step Issuer must call `indy_to_unqualified` with `cred_offer_json` to get unqualified form of Credential Offer. 
      Issuer must send this unqualified form to Holder and must use it later on credential creation.
      
    * The next steps from Issuer and Holder do as usual.
      Credential will contain unqualified identifiers.
      Holder can make unqualified Proofs.
    
* Issuer (FQ) - Holder (FQ)
    * All steps Issuer and Holder do as usual.
      All identifiers will be in fully-qualified form.
      Credential will contain fully-qualified identifiers. 
      Holder can make as unqualified as fully-qualified Proofs depend on Proof Request.
      
* Issuer (U) - Holder (U)
    * All steps Issuer and Holder do as usual.
      All identifiers will be in unqualified form.
      Credential will contain unqualified identifiers.
      Holder can make unqualified Proofs.
      
* Issuer (U) - Holder (FQ) 
    * All steps Issuer and Holder do as usual.
     Holder can handle unqualified identifiers as well.
     All identifiers will be in unqualified form.
     Credential will contain unqualified identifiers.
     Holder can make unqualified Proofs.

##### Proof Presentation

Proof Requests supports versioning (`ver` field). 
This field specifies whether proof must be full qualified or not. 
This also specifies whether proof request restrictions are full qualified or not:
- omit or set "1.0" to use unqualified identifiers. 
- set "2.0" to use fully qualified identifiers. 

    * All steps Issuer and Holder do as usual.



* Verifier (FQ) - Prover (U) 
    * Verifier should set `ver` field as '1.0' or omit it on Proof Request.
        * if restrictions are fully-qualified Verifier must call `indy_to_unqualified` function with `proof_request_json` to get unqualified form. 
        * if there are no restrictions or they are in unqualified form -- no additional steps are needed.
    
    * There are no changes from Prover side on Proof preparation.
    
    * Verifier -- proof verification -- must use *_id's (`schema_id`, `cred_def_id`, `rev_reg_id`) listed in `proof[identifiers]` as the keys for corresponding `schemas_json`, `credential_defs_json`, `rev_reg_defs_json`, `rev_regs_json` objects.

* Verifier (FQ) - Prover (FQ) 
    * Verifier can use as fully-qualified as unqualified format on ProofRequest preparation (set corresponded `ver`). 
        - omit or set "1.0" to get unqualified Proof
        - "2.0" to get fully-qualified Proof
    
    * There are no changes from Prover side on Proof preparation.

    * Verifier -- proof verification -- must use *_id's (`schema_id`, `cred_def_id`, `rev_reg_id`) listed in `proof[identifiers]` as the keys for corresponding `schemas_json`, `credential_defs_json`, `rev_reg_defs_json`, `rev_regs_json` objects.

* Verifier (U) - Prover (FQ) 
    * Verifier should set `ver` field as '1.0' or omit it on Proof Request preparation.
    
    * There are no changes from Prover side on Proof preparation. Generated Proof will be in unqualified form.
    
    * All steps Verifier and Prover do as usual.

* Verifier (U) - Prover (U) 
    * All steps Verifier and Prover do as usual.
      All identifiers will be if unqualified form.

##### Ledger API limitations
* You can create ledger requests with passing both Fully-Qualified and Unqualified entities. Out requests will be in unqualified form.  
* All Ledger API parsers return entities in unqualified form. 
* Cashing API return result in the same form as used ID.


### Indy-CLI

##### Creation
Use `did new` command with passing `method` parameter to create fully-qualified DID.

Example: `did new method=indy`

### Libvcx

##### Agent provisioning
1. Prepare provisioning config json. Append `did_method` field to this config.
2. Call `vcx_provision_agent` function with this config json. As result all generated DIDs will be in fully-qualified form.
3. Credential Issuance and Proof Presentation related entities will be generated automatically based on the type of remote DID.