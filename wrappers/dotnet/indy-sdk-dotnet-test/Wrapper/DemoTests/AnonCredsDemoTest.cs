using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;
using static Indy.Sdk.Dotnet.Wrapper.AgentObservers;
using System;

namespace Indy.Sdk.Dotnet.Test.Wrapper.LedgerTests
{
    [TestClass]
    public class AnonCredsDemoTest : IndyIntegrationTestBase
    {
        [TestMethod]
        public void TestAnonCredsDemo()
        {
            //1. Create and Open Pool
            var poolName = PoolUtils.CreatePoolLedgerConfig();

            var pool = Pool.OpenPoolLedgerAsync(poolName, "{}").Result;

            //2. Issuer Create and Open Wallet
            Wallet.CreateWalletAsync(poolName, "issuerWallet", "default", null, null).Wait();
            var issuerWallet = Wallet.OpenWalletAsync("issuerWallet", null, null).Result;

            //3. Prover Create and Open Wallet
            Wallet.CreateWalletAsync(poolName, "proverWallet", "default", null, null).Wait();
            var proverWallet = Wallet.OpenWalletAsync("proverWallet", null, null).Result;

            //4. Issuer create ClaimDef
            var schemaJson = "{\n" +
                    "                    \"seqNo\":1,\n" +
                    "                    \"data\": {\n" +
                    "                        \"name\":\"gvt\",\n" +
                    "                        \"version\":\"1.0\",\n" +
                    "                        \"keys\":[\"age\",\"sex\",\"height\",\"name\"]\n" +
                    "                    }\n" +
                    "                }";
            var issuerDid = "NcYxiDXkpYi6ov5FcYDi1e";

            var claimDef = AnonCreds.IssuerCreateAndStoreClaimDefAsync(issuerWallet, issuerDid, schemaJson, null, false).Result;
            Assert.IsNotNull(claimDef);

            //5. Prover create Master Secret
            var masterSecret = "masterSecretName";
            AnonCreds.ProverCreateMasterSecretAsync(proverWallet, masterSecret).Wait();

            //6. Prover store Claim Offer
            var claimOffer = string.Format("{{\"issuer_did\":\"{0}\", \"schema_seq_no\":{1}}}", issuerDid, 1);
            AnonCreds.ProverStoreClaimOfferAsync(proverWallet, claimOffer).Wait();

            //7. Prover get Claim Offers
            var claimOfferFilter = string.Format("{{\"issuer_did\":\"{0}\"}}", issuerDid);
            var claimOffersJson = AnonCreds.ProverGetClaimOffersAsync(proverWallet, claimOfferFilter).Result;

            var claimOffersObject = JArray.Parse(claimOffersJson);
            Assert.AreEqual(claimOffersObject.Count, 1);

            var claimOfferObject = (JObject)claimOffersObject[0];
            var claimOfferJson = claimOfferObject.ToString();

            //8. Prover create ClaimReq
            var proverDid = "BzfFCYk";
            var claimReq = AnonCreds.ProverCreateAndStoreClaimReqAsync(proverWallet, proverDid, claimOfferJson, claimDef, masterSecret).Result;
            Assert.IsNotNull(claimReq);

            //9. Issuer create Claim
            var claimAttributesJson = "{\n" +
                    "               \"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\n" +
                    "               \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\n" +
                    "               \"height\":[\"175\",\"175\"],\n" +
                    "               \"age\":[\"28\",\"28\"]\n" +
                    "        }";

            var createClaimResult = AnonCreds.IssuerCreateClaimAsync(issuerWallet, claimReq, claimAttributesJson, -1, -1).Result;
            Assert.IsNotNull(createClaimResult);
            var claimJson = createClaimResult.ClaimJson;

            //10. Prover store Claim
            AnonCreds.ProverStoreClaimAsync(proverWallet, claimJson).Wait();

            //11. Prover gets Claims for Proof Request
            var proofRequestJson = "{\n" +
                    "                          \"nonce\":\"123432421212\",\n" +
                    "                          \"name\":\"proof_req_1\",\n" +
                    "                          \"version\":\"0.1\",\n" +
                    "                          \"requested_attrs\":{\"attr1_uuid\":{\"schema_seq_no\":1,\"name\":\"name\"},\n" +
                    "                                                \"attr2_uuid\":{\"schema_seq_no\":1,\"name\":\"sex\"}},\n" +
                    "                          \"requested_predicates\":{\"predicate1_uuid\":{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18}}\n" +
                    "                  }";

            var claimsForProofJson = AnonCreds.ProverGetClaimsForProofReqAsync(proverWallet, proofRequestJson).Result;
            Assert.IsNotNull(claimsForProofJson);

            var claimsForProof = JObject.Parse(claimsForProofJson);
            var claimsForAttribute1 = (JArray)claimsForProof["attrs"]["attr1_uuid"];
            var claimsForAttribute2 = (JArray)claimsForProof["attrs"]["attr1_uuid"];
            var claimsForPredicate = (JArray)claimsForProof["predicates"]["predicate1_uuid"];

            Assert.AreEqual(claimsForAttribute1.Count, 1);
            Assert.AreEqual(claimsForAttribute2.Count, 1);
            Assert.AreEqual(claimsForPredicate.Count, 1);

            var claimUuid = claimsForAttribute1[0].Value<string>("claim_uuid");

            //12. Prover create Proof
            var selfAttestedValue = "yes";
            var requestedClaimsJson = string.Format("{{\n" +
                    "                                          \"self_attested_attributes\":{{\"self1\":\"{0}\"}},\n" +
                    "                                          \"requested_attrs\":{{\"attr1_uuid\":[\"{1}\", true],\n" +
                    "                                                               \"attr2_uuid\":[\"{2}\", false]}},\n" +
                    "                                          \"requested_predicates\":{{\"predicate1_uuid\":\"{3}\"}}\n" +
                    "                                        }}", selfAttestedValue, claimUuid, claimUuid, claimUuid);

            var schemasJson = string.Format("{{\"{0}\":{1}}}", claimUuid, schemaJson);
            var claimDefsJson = string.Format("{{\"{0}\":{1}}}", claimUuid, claimDef);
            var revocRegsJson = "{}";


            var proofJson = AnonCreds.ProverCreateProofAsync(proverWallet, proofRequestJson, requestedClaimsJson, schemasJson,
                    masterSecret, claimDefsJson, revocRegsJson).Result;
            Assert.IsNotNull(proofJson);

            var proof = JObject.Parse(proofJson);

            //13. Verifier verify Proof
            Assert.AreEqual("Alex",
                    proof["requested_proof"]["revealed_attrs"]["attr1_uuid"][1]);

            Assert.IsNotNull(proof["requested_proof"]["unrevealed_attrs"].Value<string>("attr2_uuid"));

            Assert.AreEqual(selfAttestedValue, proof["requested_proof"]["self_attested_attrs"].Value<string>("self1"));

            Boolean valid = AnonCreds.VerifierVerifyProofAsync(proofRequestJson, proofJson, schemasJson, claimDefsJson, revocRegsJson).Result;
            Assert.IsTrue(valid);

            // 14. Close and delete Issuer Wallet
            issuerWallet.CloseAsync().Wait();
            Wallet.DeleteWalletAsync("issuerWallet", null).Wait();

            // 15. Close and delete Prover Wallet
            proverWallet.CloseAsync().Wait();
            Wallet.DeleteWalletAsync("proverWallet", null).Wait();

            //16. Close Pool
            pool.CloseAsync().Wait();
        }
       
    }
}
