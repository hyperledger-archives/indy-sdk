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
        * run regular faber/alice demo to issue a credential for Alice.
        * find Alice config json in output and save it. It will be needed for `vc-auth-oidc/alice-vc-auth.py` later to skip the preparation steps.
        * now Alice is ready to pass `VC-AuthN OIDC` challenge.
1. Go to keycloaks `Login In` page and click `Verified Credential Access`. QR and URL should be showed.
    * run `vc-auth-oidc/alice-vc-auth.py` script.
    * it will ask for Alice config json
    * next it will ask for URL printed below QR code.
    * proof will be automatically generated and send to VC-AuthN provider.
1. On success authentication: `Login In` page will be redirected to `Update Account Information` page.

## Steps to pass OIDC challenge using VCX
1. Scan QR code -> send GET request to encoded URL -> take `location` header of response.
2. Take base64 string after `?m=` -> decode string -> presentation request
3. handle presentation request using VCX usual way