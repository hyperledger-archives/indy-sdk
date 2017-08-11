using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Test.Wrapper.DemoTests
{
    [TestClass]
    public class AnonCredsDemoTest : IndyIntegrationTestBase
    {
        private Pool _pool;
        private Wallet _issuerWallet;
        private Wallet _proverWallet;
        private String _poolName;

        [TestInitialize]
        public void CreateWallet()
        {
            //1. Create and Open Pool
            _poolName = PoolUtils.CreatePoolLedgerConfig();

            _pool = Pool.OpenPoolLedgerAsync(_poolName, "{}").Result;

            //2. Issuer Create and Open Wallet
            Wallet.CreateWalletAsync(_poolName, "issuerWallet", "default", null, null).Wait();
            _issuerWallet = Wallet.OpenWalletAsync("issuerWallet", null, null).Result;

            //3. Prover Create and Open Wallet
            Wallet.CreateWalletAsync(_poolName, "proverWallet", "default", null, null).Wait();
            _proverWallet = Wallet.OpenWalletAsync("proverWallet", null, null).Result;
        }

        [TestCleanup]
        public void DeleteWallet()
        {
            _issuerWallet.CloseAsync().Wait();
            Wallet.DeleteWalletAsync("issuerWallet", null).Wait();

            _proverWallet.CloseAsync().Wait();
            Wallet.DeleteWalletAsync("proverWallet", null).Wait();

            _pool.CloseAsync().Wait();
        }


        [TestMethod]
        public void TestAnonCredsDemo()
        {
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

            var claimDef = AnonCreds.IssuerCreateAndStoreClaimDefAsync(_issuerWallet, issuerDid, schemaJson, null, false).Result;
            Assert.IsNotNull(claimDef);

            //5. Prover create Master Secret
            var masterSecret = "masterSecretName";
            AnonCreds.ProverCreateMasterSecretAsync(_proverWallet, masterSecret).Wait();

            //6. Prover store Claim Offer
            var claimOffer = string.Format("{{\"issuer_did\":\"{0}\", \"schema_seq_no\":{1}}}", issuerDid, 1);
            AnonCreds.ProverStoreClaimOfferAsync(_proverWallet, claimOffer).Wait();

            //7. Prover get Claim Offers
            var claimOfferFilter = string.Format("{{\"issuer_did\":\"{0}\"}}", issuerDid);
            var claimOffersJson = AnonCreds.ProverGetClaimOffersAsync(_proverWallet, claimOfferFilter).Result;

            var claimOffersObject = JArray.Parse(claimOffersJson);
            Assert.AreEqual(claimOffersObject.Count, 1);

            var claimOfferObject = (JObject)claimOffersObject[0];
            var claimOfferJson = claimOfferObject.ToString();

            //8. Prover create ClaimReq
            var proverDid = "BzfFCYk";
            var claimReq = AnonCreds.ProverCreateAndStoreClaimReqAsync(_proverWallet, proverDid, claimOfferJson, claimDef, masterSecret).Result;
            Assert.IsNotNull(claimReq);

            //9. Issuer create Claim
            var claimAttributesJson = "{\n" +
                    "               \"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\n" +
                    "               \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\n" +
                    "               \"height\":[\"175\",\"175\"],\n" +
                    "               \"age\":[\"28\",\"28\"]\n" +
                    "        }";

            var createClaimResult = AnonCreds.IssuerCreateClaimAsync(_issuerWallet, claimReq, claimAttributesJson, -1, -1).Result;
            Assert.IsNotNull(createClaimResult);
            var claimJson = createClaimResult.ClaimJson;

            //10. Prover store Claim
            AnonCreds.ProverStoreClaimAsync(_proverWallet, claimJson).Wait();

            //11. Prover gets Claims for Proof Request
            var proofRequestJson = "{\n" +
                    "                          \"nonce\":\"123432421212\",\n" +
                    "                          \"name\":\"proof_req_1\",\n" +
                    "                          \"version\":\"0.1\",\n" +
                    "                          \"requested_attrs\":{\"attr1_uuid\":{\"schema_seq_no\":1,\"name\":\"name\"},\n" +
                    "                                                \"attr2_uuid\":{\"schema_seq_no\":1,\"name\":\"sex\"}},\n" +
                    "                          \"requested_predicates\":{\"predicate1_uuid\":{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18}}\n" +
                    "                  }";

            var claimsForProofJson = AnonCreds.ProverGetClaimsForProofReqAsync(_proverWallet, proofRequestJson).Result;
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


            var proofJson = AnonCreds.ProverCreateProofAsync(_proverWallet, proofRequestJson, requestedClaimsJson, schemasJson,
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
        }

        [TestMethod]
        public void TestAnonCredsWorksForMultipleIssuerSingleProver()
        {
            var issuerDid = "NcYxiDXkpYi6ov5FcYDi1e";
            var issuerDid2 = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";

            var issuerGvtWallet = _issuerWallet;

            //1. Issuer2 Create and Open Wallet
            Wallet.CreateWalletAsync(_poolName, "issuer2Wallet", "default", null, null).Wait();
            var issuerXyzWallet = Wallet.OpenWalletAsync("issuer2Wallet", null, null).Result;

            //2. Issuer create ClaimDef
            var gvtSchemaJson = "{\n" +
                    "                    \"seqNo\":1,\n" +
                    "                    \"data\": {\n" +
                    "                        \"name\":\"gvt\",\n" +
                    "                        \"version\":\"1.0\",\n" +
                    "                        \"keys\":[\"age\",\"sex\",\"height\",\"name\"]\n" +
                    "                    }\n" +
                    "                }";

            var gvtClaimDef = AnonCreds.IssuerCreateAndStoreClaimDefAsync(issuerGvtWallet, issuerDid, gvtSchemaJson, null, false).Result;

            //3. Issuer create ClaimDef
            var xyzSchemaJson = "{\n" +
                    "                    \"seqNo\":2,\n" +
                    "                    \"data\": {\n" +
                    "                        \"name\":\"xyz\",\n" +
                    "                        \"version\":\"1.0\",\n" +
                    "                        \"keys\":[\"status\",\"period\"]\n" +
                    "                    }\n" +
                    "                }";

            var xyzClaimDef = AnonCreds.IssuerCreateAndStoreClaimDefAsync(issuerXyzWallet, issuerDid2, xyzSchemaJson, null, false).Result;

            //4. Prover create Master Secret
            var masterSecret = "masterSecretName";
            AnonCreds.ProverCreateMasterSecretAsync(_proverWallet, masterSecret).Wait();

            //5. Prover store Claim Offer received from Issuer1
            var claimOffer = string.Format("{{\"issuer_did\":\"{0}\", \"schema_seq_no\":{1}}}", issuerDid, 1);
            AnonCreds.ProverStoreClaimOfferAsync(_proverWallet, claimOffer).Wait();

            //6. Prover store Claim Offer received from Issuer2
            var claimOffer2 = string.Format("{{\"issuer_did\":\"{0}\", \"schema_seq_no\":{1}}}", issuerDid2, 2);
            AnonCreds.ProverStoreClaimOfferAsync(_proverWallet, claimOffer2).Wait();

            //7. Prover get Claim Offers
            var claimOffersJson = AnonCreds.ProverGetClaimOffersAsync(_proverWallet, "{}").Result;

            var claimOffersObject = JArray.Parse(claimOffersJson);
            Assert.AreEqual(2, claimOffersObject.Count);

            var claimOfferObj1 = claimOffersObject[0];
            var claimOfferObj2 = claimOffersObject[1];

            var gvtClaimOffer = claimOfferObj1.Value<string>("issuer_did").Equals(issuerDid) ? claimOfferObj1.ToString() : claimOfferObj2.ToString();
            var xyzClaimOffer = claimOfferObj1.Value<string>("issuer_did").Equals(issuerDid2) ? claimOfferObj1.ToString() : claimOfferObj2.ToString();


            //8. Prover create ClaimReq for GVT Claim Offer
            var proverDid = "BzfFCYk";
            var gvtClaimReq = AnonCreds.ProverCreateAndStoreClaimReqAsync(_proverWallet, proverDid, gvtClaimOffer, gvtClaimDef, masterSecret).Result;

            //9. Issuer create Claim
            var gvtClaimAttributesJson = "{\n" +
                    "               \"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\n" +
                    "               \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\n" +
                    "               \"height\":[\"175\",\"175\"],\n" +
                    "               \"age\":[\"28\",\"28\"]\n" +
                    "        }";

            var gvtCreateClaimResult = AnonCreds.IssuerCreateClaimAsync(issuerGvtWallet, gvtClaimReq, gvtClaimAttributesJson, -1, -1).Result;
            var gvtClaimJson = gvtCreateClaimResult.ClaimJson;

            //10. Prover store Claim
            AnonCreds.ProverStoreClaimAsync(_proverWallet, gvtClaimJson).Wait();

            //11. Prover create ClaimReq for GVT Claim Offer
            var xyzClaimReq = AnonCreds.ProverCreateAndStoreClaimReqAsync(_proverWallet, proverDid, xyzClaimOffer, xyzClaimDef, masterSecret).Result;

            //12. Issuer create Claim
            var xyzClaimAttributesJson = "{\n" +
                    "               \"status\":[\"partial\",\"51792877103171595686471452153480627530895\"],\n" +
                    "               \"period\":[\"8\",\"8\"]\n" +
                    "        }";

            var xyzCreateClaimResult = AnonCreds.IssuerCreateClaimAsync(issuerXyzWallet, xyzClaimReq, xyzClaimAttributesJson, -1, -1).Result;
            var xyzClaimJson = xyzCreateClaimResult.ClaimJson;

            //13. Prover store Claim
            AnonCreds.ProverStoreClaimAsync(_proverWallet, xyzClaimJson).Wait();

            //14. Prover gets Claims for Proof Request
            var proofRequestJson = "{\n" +
                    "                          \"nonce\":\"123432421212\",\n" +
                    "                          \"name\":\"proof_req_1\",\n" +
                    "                          \"version\":\"0.1\",\n" +
                    "                          \"requested_attrs\":{\"attr1_uuid\":{\"schema_seq_no\":1,\"name\":\"name\"},\n" +
                    "                                               \"attr2_uuid\":{\"schema_seq_no\":2,\"name\":\"status\"}},\n" +
                    "                          \"requested_predicates\":{\"predicate1_uuid\":{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18}," +
                    "                                                    \"predicate2_uuid\":{\"attr_name\":\"period\",\"p_type\":\"GE\",\"value\":5}}\n" +
                    "                  }";


            var claimsForProofJson = AnonCreds.ProverGetClaimsForProofReqAsync(_proverWallet, proofRequestJson).Result;
            Assert.IsNotNull(claimsForProofJson);

            var claimsForProof = JObject.Parse(claimsForProofJson);
            var claimsForAttribute1 = (JArray)claimsForProof["attrs"]["attr1_uuid"];
            var claimsForAttribute2 = (JArray)claimsForProof["attrs"]["attr2_uuid"];
            var claimsForPredicate1 = (JArray)claimsForProof["predicates"]["predicate1_uuid"];
            var claimsForPredicate2 = (JArray)claimsForProof["predicates"]["predicate2_uuid"];

            Assert.AreEqual(claimsForAttribute1.Count, 1);
            Assert.AreEqual(claimsForAttribute2.Count, 1);
            Assert.AreEqual(claimsForPredicate1.Count, 1);
            Assert.AreEqual(claimsForPredicate2.Count, 1);

            var claimUuidForAttr1 = claimsForAttribute1[0].Value<string>("claim_uuid");
            var claimUuidForAttr2 = claimsForAttribute2[0].Value<string>("claim_uuid");
            var claimUuidForPredicate1 = claimsForPredicate1[0].Value<string>("claim_uuid");
            var claimUuidForPredicate2 = claimsForPredicate2[0].Value<string>("claim_uuid");

            //15. Prover create Proof
            var requestedClaimsJson = string.Format("{{\n" +
                    "                                          \"self_attested_attributes\":{{}},\n" +
                    "                                          \"requested_attrs\":{{\"attr1_uuid\":[\"{0}\", true],\n" +
                    "                                                               \"attr2_uuid\":[\"{1}\", true]}},\n" +
                    "                                          \"requested_predicates\":{{\"predicate1_uuid\":\"{2}\"," +
                    "                                                                    \"predicate2_uuid\":\"{3}\"}}\n" +
                    "                                        }}", claimUuidForAttr1, claimUuidForAttr2, claimUuidForPredicate1, claimUuidForPredicate2);

            var schemasJson = string.Format("{{\"{0}\":{1}, \"{2}\":{3}}}", claimUuidForAttr1, gvtSchemaJson, claimUuidForAttr2, xyzSchemaJson);
            var claimDefsJson = string.Format("{{\"{0}\":{1}, \"{2}\":{3}}}", claimUuidForAttr1, gvtClaimDef, claimUuidForAttr2, xyzClaimDef);

            var revocRegsJson = "{}";

            var proofJson = AnonCreds.ProverCreateProofAsync(_proverWallet, proofRequestJson, requestedClaimsJson, schemasJson,
                    masterSecret, claimDefsJson, revocRegsJson).Result;
            Assert.IsNotNull(proofJson);

            var proof = JObject.Parse(proofJson);

            //16. Verifier verify Proof
            Assert.AreEqual("Alex",
                    proof["requested_proof"]["revealed_attrs"]["attr1_uuid"].Value<string>(1));

            Assert.AreEqual("partial",
                    proof["requested_proof"]["revealed_attrs"]["attr2_uuid"].Value<string>(1));

            Boolean valid = AnonCreds.VerifierVerifyProofAsync(proofRequestJson, proofJson, schemasJson, claimDefsJson, revocRegsJson).Result;
            Assert.IsTrue(valid);

            //18. Close and delete Issuer2 Wallet
            issuerXyzWallet.CloseAsync().Wait();
            Wallet.DeleteWalletAsync("issuer2Wallet", null).Wait();
        }

        [TestMethod]
        public void TestAnonCredsWorksForSingleIssuerSingleProverMultipleClaims()
        {
            var issuerDid = "NcYxiDXkpYi6ov5FcYDi1e";

            //1. Issuer create ClaimDef
            var gvtSchemaJson = "{\n" +
                    "                    \"seqNo\":1,\n" +
                    "                    \"data\": {\n" +
                    "                        \"name\":\"gvt\",\n" +
                    "                        \"version\":\"1.0\",\n" +
                    "                        \"keys\":[\"age\",\"sex\",\"height\",\"name\"]\n" +
                    "                    }\n" +
                    "                }";

            var gvtClaimDef = AnonCreds.IssuerCreateAndStoreClaimDefAsync(_issuerWallet, issuerDid, gvtSchemaJson, null, false).Result;

            //2. Issuer create ClaimDef
            var xyzSchemaJson = "{\n" +
                    "                    \"seqNo\":2,\n" +
                    "                    \"data\": {\n" +
                    "                        \"name\":\"xyz\",\n" +
                    "                        \"version\":\"1.0\",\n" +
                    "                        \"keys\":[\"status\",\"period\"]\n" +
                    "                    }\n" +
                    "                }";

            var xyzClaimDef = AnonCreds.IssuerCreateAndStoreClaimDefAsync(_issuerWallet, issuerDid, xyzSchemaJson, null, false).Result;

            //3. Prover create Master Secret
            var masterSecret = "masterSecretName";
            AnonCreds.ProverCreateMasterSecretAsync(_proverWallet, masterSecret).Wait();

            //4. Prover store Claim Offer received from Issuer
            var claimOffer = string.Format("{{\"issuer_did\":\"{0}\", \"schema_seq_no\":{1}}}", issuerDid, 1);
            AnonCreds.ProverStoreClaimOfferAsync(_proverWallet, claimOffer).Wait();

            //5. Prover store Claim Offer received from Issuer
            var claimOffer2 = string.Format("{{\"issuer_did\":\"{0}\", \"schema_seq_no\":{1}}}", issuerDid, 2);
            AnonCreds.ProverStoreClaimOfferAsync(_proverWallet, claimOffer2).Wait();

            //6. Prover get Claim Offers
            var claimOffersJson = AnonCreds.ProverGetClaimOffersAsync(_proverWallet, "{}").Result;

            var claimOffersObject = JArray.Parse(claimOffersJson);
            Assert.AreEqual(2, claimOffersObject.Count);

            var claimOfferObj1 = claimOffersObject[0];
            var claimOfferObj2 = claimOffersObject[1];

            var gvtClaimOffer = claimOfferObj1.Value<int>("schema_seq_no") == 1 ? claimOfferObj1.ToString() : claimOfferObj2.ToString();
            var xyzClaimOffer = claimOfferObj1.Value<int>("schema_seq_no") == 2 ? claimOfferObj1.ToString() : claimOfferObj2.ToString();


            //7. Prover create ClaimReq for GVT Claim Offer
            var proverDid = "BzfFCYk";
            var gvtClaimReq = AnonCreds.ProverCreateAndStoreClaimReqAsync(_proverWallet, proverDid, gvtClaimOffer, gvtClaimDef, masterSecret).Result;

            //8. Issuer create Claim
            var gvtClaimAttributesJson = "{\n" +
                    "               \"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\n" +
                    "               \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\n" +
                    "               \"height\":[\"175\",\"175\"],\n" +
                    "               \"age\":[\"28\",\"28\"]\n" +
                    "        }";

            var gvtCreateClaimResult = AnonCreds.IssuerCreateClaimAsync(_issuerWallet, gvtClaimReq, gvtClaimAttributesJson, -1, -1).Result;
            var gvtClaimJson = gvtCreateClaimResult.ClaimJson;

            //9. Prover store Claim
            AnonCreds.ProverStoreClaimAsync(_proverWallet, gvtClaimJson).Wait();

            //10. Prover create ClaimReq for GVT Claim Offer
            var xyzClaimReq = AnonCreds.ProverCreateAndStoreClaimReqAsync(_proverWallet, proverDid, xyzClaimOffer, xyzClaimDef, masterSecret).Result;

            //11. Issuer create Claim
            var xyzClaimAttributesJson = "{\n" +
                    "               \"status\":[\"partial\",\"51792877103171595686471452153480627530895\"],\n" +
                    "               \"period\":[\"8\",\"8\"]\n" +
                    "        }";

            var xyzCreateClaimResult = AnonCreds.IssuerCreateClaimAsync(_issuerWallet, xyzClaimReq, xyzClaimAttributesJson, -1, -1).Result;
            var xyzClaimJson = xyzCreateClaimResult.ClaimJson;

            //12. Prover store Claim
            AnonCreds.ProverStoreClaimAsync(_proverWallet, xyzClaimJson).Wait();

            //13. Prover gets Claims for Proof Request
            var proofRequestJson = "{\n" +
                    "                          \"nonce\":\"123432421212\",\n" +
                    "                          \"name\":\"proof_req_1\",\n" +
                    "                          \"version\":\"0.1\",\n" +
                    "                          \"requested_attrs\":{\"attr1_uuid\":{\"schema_seq_no\":1,\"name\":\"name\"}},\n" +
                    "                          \"requested_predicates\":{\"predicate1_uuid\":{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18}," +
                    "                                                    \"predicate2_uuid\":{\"attr_name\":\"period\",\"p_type\":\"GE\",\"value\":5}}\n" +
                    "                  }";


            var claimsForProofJson = AnonCreds.ProverGetClaimsForProofReqAsync(_proverWallet, proofRequestJson).Result;
            Assert.IsNotNull(claimsForProofJson);

            var claimsForProof = JObject.Parse(claimsForProofJson);
            var claimsForAttribute1 = (JArray)claimsForProof["attrs"]["attr1_uuid"];
            var claimsForPredicate1 = (JArray)claimsForProof["predicates"]["predicate1_uuid"];
            var claimsForPredicate2 = (JArray)claimsForProof["predicates"]["predicate2_uuid"];

            Assert.AreEqual(claimsForAttribute1.Count, 1);
            Assert.AreEqual(claimsForPredicate1.Count, 1);
            Assert.AreEqual(claimsForPredicate2.Count, 1);

            var claimUuidForAttr1 = claimsForAttribute1[0].Value<string>("claim_uuid");
            var claimUuidForPredicate1 = claimsForPredicate1[0].Value<string>("claim_uuid");
            var claimUuidForPredicate2 = claimsForPredicate2[0].Value<string>("claim_uuid");

            //14. Prover create Proof
            var requestedClaimsJson = string.Format("{{\n" +
                    "                                          \"self_attested_attributes\":{{}},\n" +
                    "                                          \"requested_attrs\":{{\"attr1_uuid\":[\"{0}\", true]}},\n" +
                    "                                          \"requested_predicates\":{{\"predicate1_uuid\":\"{1}\"," +
                    "                                                                    \"predicate2_uuid\":\"{2}\"}}\n" +
                    "                                        }}", claimUuidForAttr1, claimUuidForPredicate1, claimUuidForPredicate2);

            var schemasJson = string.Format("{{\"{0}\":{1}, \"{2}\":{3}}}", claimUuidForAttr1, gvtSchemaJson, claimUuidForPredicate2, xyzSchemaJson);
            var claimDefsJson = string.Format("{{\"{0}\":{1}, \"{2}\":{3}}}", claimUuidForAttr1, gvtClaimDef, claimUuidForPredicate2, xyzClaimDef);

            var revocRegsJson = "{}";

            var proofJson = AnonCreds.ProverCreateProofAsync(_proverWallet, proofRequestJson, requestedClaimsJson, schemasJson,
                    masterSecret, claimDefsJson, revocRegsJson).Result;
            Assert.IsNotNull(proofJson);

            var proof = JObject.Parse(proofJson);

            //15. Verifier verify Proof
            Assert.AreEqual("Alex",
                    proof["requested_proof"]["revealed_attrs"]["attr1_uuid"][1]);

            Boolean valid = AnonCreds.VerifierVerifyProofAsync(proofRequestJson, proofJson, schemasJson, claimDefsJson, revocRegsJson).Result;
            Assert.IsTrue(valid);
        }

        [TestMethod]
        public async Task TestVerifyProofWorksForProofDoesNotCorrespondToProofRequest()
        {
            //1. Issuer create ClaimDef
            var schemaJson = "{\n" +
                    "                    \"seqNo\":1,\n" +
                    "                    \"data\": {\n" +
                    "                        \"name\":\"gvt\",\n" +
                    "                        \"version\":\"1.0\",\n" +
                    "                        \"keys\":[\"age\",\"sex\",\"height\",\"name\"]\n" +
                    "                    }\n" +
                    "                }";
            var issuerDid = "NcYxiDXkpYi6ov5FcYDi1e";

            var claimDef = AnonCreds.IssuerCreateAndStoreClaimDefAsync(_issuerWallet, issuerDid, schemaJson, null, false).Result;

            Assert.IsNotNull(claimDef);

            //2. Prover create Master Secret
            var masterSecret = "masterSecretName";
            AnonCreds.ProverCreateMasterSecretAsync(_proverWallet, masterSecret).Wait();

            //3. Prover store Claim Offer
            var claimOffer = string.Format("{{\"issuer_did\":\"{0}\", \"schema_seq_no\":{1}}}", issuerDid, 1);
            AnonCreds.ProverStoreClaimOfferAsync(_proverWallet, claimOffer).Wait();

            //4. Prover get Claim Offers
            var claimOfferFilter = string.Format("{{\"issuer_did\":\"{0}\"}}", issuerDid);
            var claimOffersJson = AnonCreds.ProverGetClaimOffersAsync(_proverWallet, claimOfferFilter).Result;

            var claimOffersObject = JArray.Parse(claimOffersJson);

            Assert.AreEqual(claimOffersObject.Count, 1);

            var claimOfferObject = claimOffersObject[0];
            var claimOfferJson = claimOfferObject.ToString();

            //5. Prover create ClaimReq
            var proverDid = "BzfFCYk";
            var claimReq = AnonCreds.ProverCreateAndStoreClaimReqAsync(_proverWallet, proverDid, claimOfferJson, claimDef, masterSecret).Result;

            Assert.IsNotNull(claimReq);

            //6. Issuer create Claim
            var claimAttributesJson = "{\n" +
                    "               \"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\n" +
                    "               \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\n" +
                    "               \"height\":[\"175\",\"175\"],\n" +
                    "               \"age\":[\"28\",\"28\"]\n" +
                    "        }";

            var createClaimResult = AnonCreds.IssuerCreateClaimAsync(_issuerWallet, claimReq, claimAttributesJson, -1, -1).Result;

            Assert.IsNotNull(createClaimResult);
            var claimJson = createClaimResult.ClaimJson;

            //7. Prover store Claim
            AnonCreds.ProverStoreClaimAsync(_proverWallet, claimJson).Wait();

            //8. Prover gets Claims for Proof Request
            var proofRequestJson = "{\n" +
                    "                          \"nonce\":\"123432421212\",\n" +
                    "                          \"name\":\"proof_req_1\",\n" +
                    "                          \"version\":\"0.1\",\n" +
                    "                          \"requested_attrs\":{\"attr1_uuid\":{\"schema_seq_no\":1,\"name\":\"name\"}},\n" +
                    "                          \"requested_predicates\":{}\n" +
                    "                  }";

            var claimsForProofJson = AnonCreds.ProverGetClaimsForProofReqAsync(_proverWallet, proofRequestJson).Result;

            Assert.IsNotNull(claimsForProofJson);

            var claimsForProof = JObject.Parse(claimsForProofJson);
            var claimsForAttribute1 = (JArray)claimsForProof["attrs"]["attr1_uuid"];


            Assert.AreEqual(claimsForAttribute1.Count, 1);

            var claimUuid = claimsForAttribute1[0].Value<string>("claim_uuid");

            //9. Prover create Proof
            var selfAttestedValue = "yes";
            var requestedClaimsJson = string.Format("{{\n" +
                    "                                          \"self_attested_attributes\":{{\"self1\":\"{0}\"}},\n" +
                    "                                          \"requested_attrs\":{{\"attr1_uuid\":[\"{1}\", true],\n" +
                    "                                          \"requested_predicates\":{{}}\n" +
                    "                                        }}", selfAttestedValue, claimUuid);

            var schemasJson = string.Format("{{\"{0}\":{1}}}", claimUuid, schemaJson);
            var claimDefsJson = string.Format("{{\"{0}\":{1}}}", claimUuid, claimDef);
            var revocRegsJson = "{}";

            //TODO: Not sure why this call is failing...
            var proofJson = AnonCreds.ProverCreateProofAsync(_proverWallet, proofRequestJson, requestedClaimsJson, schemasJson,
                    masterSecret, claimDefsJson, revocRegsJson).Result;

            Assert.IsNotNull(proofJson);

            var proof = JObject.Parse(proofJson);

            //10. Verifier verify Proof
            Assert.AreEqual("Alex",
                    proof["requested_proof"]["revealed_attrs"]["attr1_uuid"][1]);


            Assert.AreEqual(selfAttestedValue, proof["requested_proof"]["self_attested_attrs"].Value<string>("self1"));

            proofRequestJson = "{\n" +
                    "                            \"nonce\":\"123432421212\",\n" +
                    "                        \"name\":\"proof_req_1\",\n" +
                    "                        \"version\":\"0.1\",\n" +
                    "                    \"requested_attrs\":{\"attr1_uuid\":{\"schema_seq_no\":1,\"name\":\"name\"}},\n" +
                    "                    \"requested_predicates\":{\"predicate1_uuid\":{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18}\n" +
                    "           }";

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
              AnonCreds.VerifierVerifyProofAsync(proofRequestJson, proofJson, schemasJson, claimDefsJson, revocRegsJson)
          );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
    }
    }
}
