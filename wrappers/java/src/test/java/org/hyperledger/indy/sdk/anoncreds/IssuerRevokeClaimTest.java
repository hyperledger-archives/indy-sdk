package org.hyperledger.indy.sdk.anoncreds;

import static org.junit.Assert.*;

import java.util.concurrent.ExecutionException;

import org.hyperledger.indy.sdk.anoncreds.AnoncredsResults.IssuerCreateClaimResult;
import org.hyperledger.indy.sdk.utils.StorageUtils;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.json.JSONArray;
import org.json.JSONObject;
import org.junit.*;
import static org.hamcrest.CoreMatchers.isA;

public class IssuerRevokeClaimTest extends AnoncredsIntegrationTest
{
    private Wallet issuerWallet;
    private final String walletName = "issuerWallet";
    private final int userRevocIndex = 1;
    private final String proofReqJson = "{" +
                                   "\"nonce\":\"123432421212\"," +
                                   "\"name\":\"proof_req_1\"," +
                                   "\"version\":\"0.1\"," +
                                   "\"requested_attrs\":{\"attr1_uuid\":{\"schema_seq_no\":1,\"name\":\"name\"}}," +
                                   "\"requested_predicates\":{}" +
                                "}";    
    
    private final String requestedClaimsJsonTemplate = "{" +
                                                  "\"self_attested_attributes\":{}," +
                                                  "\"requested_attrs\":{\"attr1_uuid\":[\"%s\", true]}," +
                                                  "\"requested_predicates\":{}" +
                                                "}";
    private String claimDefJson;
    private IssuerCreateClaimResult claimResult;

    @Before
    public void before() throws Exception  {
    	 StorageUtils.cleanupStorage();
    	 
        //1. Create Issuer wallet, get wallet handle
         Wallet.createWallet("default", walletName, "default", null, null).get();
        issuerWallet = Wallet.openWallet(walletName, null, null).get();

        //2. Issuer create claim definition
        claimDefJson = Anoncreds.issuerCreateAndStoreClaimDef(issuerWallet, issuerDid, schema, null, true).get();

        //3. Issuer create revocation registry
        Anoncreds.issuerCreateAndStoreRevocReg(issuerWallet, issuerDid, 1, 5).get();

        //4. Prover create Master Secret
        Anoncreds.proverCreateMasterSecret(issuerWallet, masterSecretName).get();

        //5. Prover store Claim Offer received from Issuer
        String claimOfferJson = String.format(claimOfferTemplate, issuerDid, 1);
        Anoncreds.proverStoreClaimOffer(issuerWallet, claimOfferJson).get();

        //6. Prover create Claim Request
        String proverDid = "BzfFCYk";
        String claimReq = Anoncreds.proverCreateAndStoreClaimReq(
            issuerWallet,
            proverDid,
            claimOfferJson,
            claimDefJson,
            masterSecretName).get();

        //7. Issuer create Claim
        String claimJson = "{" +
            "\"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"]," +
           "\"name\":[\"Alex\",\"1139481716457488690172217916278103335\"]," +
           "\"height\":[\"175\",\"175\"]," +
           "\"age\":[\"28\",\"28\"]" +
        "}";

        
        claimResult = Anoncreds.issuerCreateClaim(issuerWallet, claimReq, claimJson, userRevocIndex).get();

        //8. Prover store received Claim
        Anoncreds.proverStoreClaim(issuerWallet, claimResult.getClaimJson()).get();
    }

    @After
    public void after() throws Exception {
         issuerWallet.closeWallet().get();
        StorageUtils.cleanupStorage();
    }

    @Test
    public void testAnoncredsWorksForClaimRevokedBeforeProofCreated() throws Exception {
    	thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(ClaimRevokedException.class));
		
        //9. Issuer revoke claim
        String revocRegUpdateJson =  Anoncreds.issuerRevokeClaim(
            issuerWallet,
            issuerDid,
            1,
            1).get();

        //10. Prover gets Claims for Proof Request
        String claimsJson =  Anoncreds.proverGetClaimsForProofReq(issuerWallet, proofReqJson).get();
        JSONObject claims = new JSONObject(claimsJson);
        JSONArray claimsForAttr1 = claims.getJSONObject("attrs").getJSONArray("attr1_uuid"); 
        String claimUuid = claimsForAttr1.getJSONObject(0).getString("claim_uuid");

        //11. Prover create Proof
        String requestedClaimsJson = String.format(requestedClaimsJsonTemplate, claimUuid);

        String schemasJson = String.format("{\"%s\":%s}", claimUuid, schema);
        String claimDefsJson = String.format("{\"%s\":%s}", claimUuid, claimDefJson);
        String revocRegsJsons = String.format("{\"%s\":%s}", claimUuid, revocRegUpdateJson);

        Anoncreds.proverCreateProof(
            issuerWallet,
            proofReqJson,
            requestedClaimsJson,
            schemasJson,
            masterSecretName,
            claimDefsJson,
            revocRegsJsons).get();
    }

    @Test
    public void testAnoncredsWorksForClaimRevokedAfterProofCreated() throws Exception {
        //9. Prover gets Claims for Proof Request
        String claimsJson =  Anoncreds.proverGetClaimsForProofReq(issuerWallet, proofReqJson).get();
        JSONObject claims = new JSONObject(claimsJson);
        JSONArray claimsForAttr1 = claims.getJSONObject("attrs").getJSONArray("attr1_uuid"); 
        String claimUuid = claimsForAttr1.getJSONObject(0).getString("claim_uuid");

        //10. Prover create Proof
        String requestedClaimsJson = String.format(requestedClaimsJsonTemplate, claimUuid);

        String schemasJson = String.format("{\"%s\":%s}", claimUuid, schema);
        String claimDefsJson = String.format("{\"%s\":%s}", claimUuid, claimDefJson);
        String revocRegsJsons = String.format("{\"%s\":%s}", claimUuid, claimResult.getRevocRegUpdateJson());

        String proofJson =  Anoncreds.proverCreateProof(
            issuerWallet,
            proofReqJson,
            requestedClaimsJson,
            schemasJson,
            masterSecretName,
            claimDefsJson,
            revocRegsJsons).get();

        //11. Issuer revoke prover claim
        String revocRegUpdateJson =  Anoncreds.issuerRevokeClaim(
            issuerWallet,
            issuerDid,
            1,
            1).get();

        //12. Verifier verify proof
        String updatedRevocRegsJsons = String.format("{\"%s\":%s}", claimUuid, revocRegUpdateJson);

        boolean valid =  Anoncreds.verifierVerifyProof(
            proofReqJson,
            proofJson,
            schemasJson,
            claimDefsJson,
            updatedRevocRegsJsons).get();

        assertFalse(valid);
    }
}
