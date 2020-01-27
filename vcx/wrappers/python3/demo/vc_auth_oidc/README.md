# Authentication with VC-AuthN OIDC

This is the script which shows how Vcx can be used to pass [VC-AuthN OIDC Demo](https://github.com/bcgov/vc-authn-oidc#a-quick-demo).

## Steps to Run
1. Network - run local Pool Ledger or use any available.
    * Update VCX demo genesis transactions (`demo/docker.txn` file) to point on the pool.
1. Run [VC-AuthN OIDC Demo](https://github.com/bcgov/vc-authn-oidc#a-quick-demo) environment with dependencies.
    * It requires `von-network` instance to be run.
     Take note that `von-network` must use the same Pool Ledger genesis transactions as VCX demo (`demo/docker.txn` file).
    * `VC-AuthN OIDC Demo` troubleshooting: it may be needed to replace all usages of `localhost` on IP address of your machine.
1. Perform `VC-AuthN OIDC Demo` preparation steps:
    * post Presentation Request schema
    * Issue Credential for Alice:
        * start Dummy Cloud Agent.
        * modify `demo/faber.py` script to issue a credential with required attributes (`schema` and `schema_attrs` structures).
        * run regular faber/alice demo to issue a credential for Alice. (After connection is established - choose option `1` from Faber and Alice scripts to issue a credential)
        * Since credential is issued Alice is ready to pass `VC-AuthN OIDC` challenge. Keep Alice running.
1. Go to keycloaks `Login In` page and click `Verified Credential Access`. QR and URL should be showed.
    * go to Alice and choose option `3`. It will ask for URL.
    * proof will be automatically generated and send to VC-AuthN provider.
1. On success authentication: `Login In` page will be redirected to `Update Account Information` page.

## Steps to pass OIDC challenge using VCX
1. Scan QR code -> send GET request to encoded URL -> take `location` header of response.
2. Take base64 string after `?m=` -> decode string -> presentation request
3. handle presentation request using VCX usual way. (`connection` object on `send_proof` function isn't required here)