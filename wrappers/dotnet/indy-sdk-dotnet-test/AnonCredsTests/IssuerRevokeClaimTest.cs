using Hyperledger.Indy.AnonCredsApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.AnonCredsTests
{
    //TODO: Implement these tests.
    [TestClass]
    public class IssuerRevokeClaimTest
    {
        [TestMethod]
        public async Task anoncreds_works_for_claim_revoked_before_proof_created()
        {
            StorageUtils.CleanupStorage();

            var issuer_did = "NcYxiDXkpYi6ov5FcYDi1e";

            //1. Create Issuer wallet, get wallet handle
            await Wallet.CreateWalletAsync("default", "issuerWallet", "default", null, null);
            var issuerWallet = await Wallet.OpenWalletAsync("issuerWallet", null, null);

            //2. Issuer create claim definition
            var schema_seq_no = 1;
            var schema = string.Format("{{\"seqNo\":{0},\"data\":{{\"name\":\"gvt\",\"version\":\"1.0\",\"keys\":[\"age\",\"sex\",\"height\",\"name\"]}}}}", schema_seq_no);

            var claim_def_json = await AnonCreds.IssuerCreateAndStoreClaimDefAsync(issuerWallet, issuer_did, schema, null, true);

            //3. Issuer create revocation registry
            await AnonCreds.IssuerCreateAndStoreRevocRegAsync(issuerWallet, issuer_did, schema_seq_no, 5);

            //4. Prover create Master Secret
            var master_secret_name = "prover_master_secret";
            await AnonCreds.ProverCreateMasterSecretAsync(issuerWallet, master_secret_name);

            //5. Prover store Claim Offer received from Issuer
            var claim_offer_json = string.Format("{{\"issuer_did\":\"{0}\",\"schema_seq_no\":{1}}}", issuer_did, schema_seq_no);
            await AnonCreds.ProverStoreClaimOfferAsync(issuerWallet, claim_offer_json);

            //6. Prover create Claim Request
            var prover_did = "BzfFCYk";
            var claim_req = await AnonCreds.ProverCreateAndStoreClaimReqAsync(
                issuerWallet,
                prover_did,
                claim_offer_json,
                claim_def_json,
                master_secret_name);

            //7. Issuer create Claim
            var claim_json = "{" +
                "\"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"]," +
               "\"name\":[\"Alex\",\"1139481716457488690172217916278103335\"]," +
               "\"height\":[\"175\",\"175\"]," +
               "\"age\":[\"28\",\"28\"]" +
            "}";

            var user_revoc_index = 1;
            var claim_result = await AnonCreds.IssuerCreateClaimAsync(issuerWallet, claim_req, claim_json, user_revoc_index);

            //8. Prover store received Claim
            await AnonCreds.ProverStoreClaimAsync(issuerWallet, claim_result.ClaimJson);

            //9. Issuer revoke claim
            var revoc_reg_update_json = await AnonCreds.IssuerRevokeClaimAsync(
                issuerWallet,
                issuer_did,
                schema_seq_no,
                user_revoc_index);

            //10. Prover gets Claims for Proof Request
            var proof_req_json = string.Format("{{" +
                                       "\"nonce\":\"123432421212\"," +
                                       "\"name\":\"proof_req_1\"," +
                                       "\"version\":\"0.1\"," +
                                       "\"requested_attrs\":{{\"attr1_uuid\":{{\"schema_seq_no\":{0},\"name\":\"name\"}}}}," +
                                       "\"requested_predicates\":{{}}" +
                                    "}}", schema_seq_no);

            var claims_json = await AnonCreds.ProverGetClaimsForProofReqAsync(issuerWallet, proof_req_json);
            var claims = JObject.Parse(claims_json);
            var claims_for_attr_1 = claims["attrs"]["attr1_uuid"];
            var claim = claims_for_attr_1[0];
            var claim_uuid = claim.Value<string>("claim_uuid");

            //11. Prover create Proof
            var requested_claims_json = string.Format("{{" +
                                                      "\"self_attested_attributes\":{{}}," +
                                                      "\"requested_attrs\":{{\"attr1_uuid\":[\"{0}\", true]}}," +
                                                      "\"requested_predicates\":{{}}" +
                                                    "}}", claim_uuid);

            var schemas_json = string.Format("{{\"{0}\":{1}}}", claim_uuid, schema);
            var claim_defs_json = string.Format("{{\"{0}\":{1}}}", claim_uuid, claim_def_json);
            var revoc_regs_jsons = string.Format("{{\"{0}\":{1}}}", claim_uuid, revoc_reg_update_json);

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                AnonCreds.ProverCreateProofAsync(
                    issuerWallet,
                    proof_req_json,
                    requested_claims_json,
                    schemas_json,
                    master_secret_name,
                    claim_defs_json,
                    revoc_regs_jsons)
            );

            Assert.AreEqual(ErrorCode.AnoncredsClaimRevoked, ex.ErrorCode);

            StorageUtils.CleanupStorage();
        }

        [TestMethod]
        public async Task anoncreds_works_for_claim_revoked_after_proof_created()
        {
            StorageUtils.CleanupStorage();

            var issuer_did = "NcYxiDXkpYi6ov5FcYDi1e";

            //1. Create Issuer wallet, get wallet handle
            await Wallet.CreateWalletAsync("default", "issuerWallet", "default", null, null);
            var issuerWallet = await Wallet.OpenWalletAsync("issuerWallet", null, null);

            //2. Issuer create claim definition
            var schema_seq_no = 1;
            var schema = string.Format("{{\"seqNo\":{0},\"data\":{{\"name\":\"gvt\",\"version\":\"1.0\",\"keys\":[\"age\",\"sex\",\"height\",\"name\"]}}}}", schema_seq_no);

            var claim_def_json = await AnonCreds.IssuerCreateAndStoreClaimDefAsync(issuerWallet, issuer_did, schema, null, true);

            //3. Issuer create revocation registry
            await AnonCreds.IssuerCreateAndStoreRevocRegAsync(issuerWallet, issuer_did, schema_seq_no, 5);

            //4. Prover create Master Secret
            var master_secret_name = "prover_master_secret";
            await AnonCreds.ProverCreateMasterSecretAsync(issuerWallet, master_secret_name);

            //5. Prover store Claim Offer received from Issuer
            var claim_offer_json = string.Format("{{\"issuer_did\":\"{0}\",\"schema_seq_no\":{1}}}", issuer_did, schema_seq_no);
            await AnonCreds.ProverStoreClaimOfferAsync(issuerWallet, claim_offer_json);

            //6. Prover create Claim Request
            var prover_did = "BzfFCYk";
            var claim_req = await AnonCreds.ProverCreateAndStoreClaimReqAsync(
                issuerWallet,
                prover_did,
                claim_offer_json,
                claim_def_json,
                master_secret_name);

            //7. Issuer create Claim
            var claim_json = "{" +
                "\"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"]," +
               "\"name\":[\"Alex\",\"1139481716457488690172217916278103335\"]," +
               "\"height\":[\"175\",\"175\"]," +
               "\"age\":[\"28\",\"28\"]" +
            "}";

            var user_revoc_index = 1;
            var claim_result = await AnonCreds.IssuerCreateClaimAsync(issuerWallet, claim_req, claim_json, user_revoc_index);

            //8. Prover store received Claim
            await AnonCreds.ProverStoreClaimAsync(issuerWallet, claim_result.ClaimJson);

            //9. Prover gets Claims for Proof Request
            var proof_req_json = string.Format("{{" +
                                       "\"nonce\":\"123432421212\"," +
                                       "\"name\":\"proof_req_1\"," +
                                       "\"version\":\"0.1\"," +
                                       "\"requested_attrs\":{{\"attr1_uuid\":{{\"schema_seq_no\":{0},\"name\":\"name\"}}}}," +
                                       "\"requested_predicates\":{{}}" +
                                    "}}", schema_seq_no);

            var claims_json = await AnonCreds.ProverGetClaimsForProofReqAsync(issuerWallet, proof_req_json);
            var claims = JObject.Parse(claims_json);
            var claims_for_attr_1 = claims["attrs"]["attr1_uuid"];
            var claim = claims_for_attr_1[0];
            var claim_uuid = claim.Value<string>("claim_uuid");

            //10. Prover create Proof
            var requested_claims_json = string.Format("{{" +
                                                      "\"self_attested_attributes\":{{}}," +
                                                      "\"requested_attrs\":{{\"attr1_uuid\":[\"{0}\", true]}}," +
                                                      "\"requested_predicates\":{{}}" +
                                                    "}}", claim_uuid);

            var schemas_json = string.Format("{{\"{0}\":{1}}}", claim_uuid, schema);
            var claim_defs_json = string.Format("{{\"{0}\":{1}}}", claim_uuid, claim_def_json);
            var revoc_regs_jsons = string.Format("{{\"{0}\":{1}}}", claim_uuid, claim_result.RevocRegUpdateJson);

            var proof_json = await AnonCreds.ProverCreateProofAsync(
                issuerWallet,
                proof_req_json,
                requested_claims_json,
                schemas_json,
                master_secret_name,
                claim_defs_json,
                revoc_regs_jsons);

            //11. Issuer revoke prover claim
            var revoc_reg_update_json = await AnonCreds.IssuerRevokeClaimAsync(
                issuerWallet,
                issuer_did,
                schema_seq_no,
                user_revoc_index);

            //12. Verifier verify proof
            var updated_revoc_regs_jsons = string.Format("{{\"{0}\":{1}}}", claim_uuid, revoc_reg_update_json);

            var valid = await AnonCreds.VerifierVerifyProofAsync(
                proof_req_json,
                proof_json,
                schemas_json,
                claim_defs_json,
                updated_revoc_regs_jsons);

            Assert.IsFalse(valid);

            StorageUtils.CleanupStorage();
        }
    }
}
