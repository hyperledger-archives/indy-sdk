    log("10. Prover creates Proof for Proof Request")
    const credForAttr1 = credsForProofRequest["attrs"]["attr1_referent"]
    const referent = credForAttr1[0].cred_info.referent
    const requestedCredentials = {
        "self_attested_attributes": {},
        "requested_attributes": {
            "attr1_referent": {
                cred_id: referent,
                revealed: true
            }
        },
        "requested_predicates": {
            "predicate1_referent": {
                cred_id: referent
            }
        }
    }
    const schemas = {
        [schemaId]: schema
    }
    const credentialDefs = {
        [credDefId]: credDef
    }
    const revocRegs = {}
    const revRegs = {}
    const proof = await indy.proverCreateProof(proverWalletHandle, proofRequest, requestedCredentials, proverMasterSecret, schemas, credentialDefs, revocRegs)
