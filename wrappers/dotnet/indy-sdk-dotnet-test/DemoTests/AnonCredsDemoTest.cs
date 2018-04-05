using Hyperledger.Indy.AnonCredsApi;
using Hyperledger.Indy.PoolApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.DemoTests
{
    [TestClass]
    public class AnonCredsDemoTest : IndyIntegrationTestBase
    {
        private Pool _pool;
        private Wallet _issuerWallet;
        private Wallet _proverWallet;
        private String _poolName;

        [TestInitialize]
        public async Task CreateWallet()
        {
            //1. Create and Open Pool
            _poolName = PoolUtils.CreatePoolLedgerConfig();

            _pool = await Pool.OpenPoolLedgerAsync(_poolName, "{}");

            //2. Issuer Create and Open Wallet
            await Wallet.CreateWalletAsync(_poolName, "issuerWallet", TYPE, null, null);
            _issuerWallet = await Wallet.OpenWalletAsync("issuerWallet", null, null);

            //3. Prover Create and Open Wallet
            await Wallet.CreateWalletAsync(_poolName, "proverWallet", TYPE, null, null);
            _proverWallet = await Wallet.OpenWalletAsync("proverWallet", null, null);
        }

        [TestCleanup]
        public async Task DeleteWallet()
        {
            if(_issuerWallet != null)
                await _issuerWallet.CloseAsync();

            await Wallet.DeleteWalletAsync("issuerWallet", null);

            if(_proverWallet != null)
                await _proverWallet.CloseAsync();

            await Wallet.DeleteWalletAsync("proverWallet", null);

            if(_pool != null)
                await _pool.CloseAsync();
        }


        [TestMethod]
        public async Task TestAnonCredsDemo()
        {
            //4. Issuer create ClaimDef
            var schemaJson = "{\n" +
                    "                    \"seqNo\":1,\n" +
                    "                    \"data\": {\n" +
                    "                        \"name\":\"gvt\",\n" +
                    "                        \"version\":\"1.0\",\n" +
                    "                        \"attr_names\":[\"age\",\"sex\",\"height\",\"name\"]\n" +
                    "                    }\n" +
                    "                }";
            var issuerDid = "NcYxiDXkpYi6ov5FcYDi1e";

            var claimDef = await AnonCreds.IssuerCreateAndStoreClaimDefAsync(_issuerWallet, issuerDid, schemaJson, null, false);
            Assert.IsNotNull(claimDef);

            //5. Prover create Master Secret
            var masterSecret = "masterSecretName";
            await AnonCreds.ProverCreateMasterSecretAsync(_proverWallet, masterSecret);

            //6. Prover store Claim Offer
            var claimOffer = string.Format("{{\"issuer_did\":\"{0}\", \"schema_seq_no\":{1}}}", issuerDid, 1);
            await AnonCreds.ProverStoreClaimOfferAsync(_proverWallet, claimOffer);

            //7. Prover get Claim Offers
            var claimOfferFilter = string.Format("{{\"issuer_did\":\"{0}\"}}", issuerDid);
            var claimOffersJson = await AnonCreds.ProverGetClaimOffersAsync(_proverWallet, claimOfferFilter);

            var claimOffersObject = JArray.Parse(claimOffersJson);
            Assert.AreEqual(claimOffersObject.Count, 1);

            var claimOfferObject = (JObject)claimOffersObject[0];
            var claimOfferJson = claimOfferObject.ToString();

            //8. Prover create ClaimReq
            var proverDid = "BzfFCYk";
            var claimReq = await AnonCreds.ProverCreateAndStoreClaimReqAsync(_proverWallet, proverDid, claimOfferJson, claimDef, masterSecret);
            Assert.IsNotNull(claimReq);

            //9. Issuer create Claim
            var claimAttributesJson = "{\n" +
                    "               \"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\n" +
                    "               \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\n" +
                    "               \"height\":[\"175\",\"175\"],\n" +
                    "               \"age\":[\"28\",\"28\"]\n" +
                    "        }";

            var createClaimResult = await AnonCreds.IssuerCreateClaimAsync(_issuerWallet, claimReq, claimAttributesJson, -1);
            Assert.IsNotNull(createClaimResult);
            var claimJson = createClaimResult.ClaimJson;

            //10. Prover store Claim
            await AnonCreds.ProverStoreClaimAsync(_proverWallet, claimJson, createClaimResult.RevocRegUpdateJson);

            //11. Prover gets Claims for Proof Request
            var proofRequestJson = "{\n" +
                    "                          \"nonce\":\"123432421212\",\n" +
                    "                          \"name\":\"proof_req_1\",\n" +
                    "                          \"version\":\"0.1\",\n" +
                    "                          \"requested_attrs\":{\"attr1_referent\":{\"name\":\"name\",\"restrictions\":[{\"schema_seq_no\":1}]},\n" +
                    "                                                \"attr2_referent\":{\"name\":\"sex\",\"restrictions\":[{\"schema_seq_no\":1}]},\n" +
                    "                                                \"attr3_referent\":{\"phone\":\"sex\"}},\n" +
                    "                          \"requested_predicates\":{\"predicate1_referent\":{\"attr_name\":\"age\",\"p_type\":\">=\",\"value\":18}}\n" +
                    "                  }";

            var claimsForProofJson = await AnonCreds.ProverGetClaimsForProofReqAsync(_proverWallet, proofRequestJson);
            Assert.IsNotNull(claimsForProofJson);

            var claimsForProof = JObject.Parse(claimsForProofJson);
            var claimsForAttribute1 = (JArray)claimsForProof["attrs"]["attr1_referent"];
            var claimsForAttribute2 = (JArray)claimsForProof["attrs"]["attr1_referent"];
            var claimsForPredicate = (JArray)claimsForProof["predicates"]["predicate1_referent"];

            Assert.AreEqual(claimsForAttribute1.Count, 1);
            Assert.AreEqual(claimsForAttribute2.Count, 1);
            Assert.AreEqual(claimsForPredicate.Count, 1);

            var claimUuid = claimsForAttribute1[0].Value<string>("referent");

            //12. Prover create Proof
            var selfAttestedValue = "8-800-200";
            var requestedClaimsJson = string.Format("{{\n" +
                    "                                          \"self_attested_attributes\":{{\"attr3_referent\":\"{0}\"}},\n" +
                    "                                          \"requested_attrs\":{{\"attr1_referent\":[\"{1}\", true],\n" +
                    "                                                               \"attr2_referent\":[\"{2}\", false]}},\n" +
                    "                                          \"requested_predicates\":{{\"predicate1_referent\":\"{3}\"}}\n" +
                    "                                        }}", selfAttestedValue, claimUuid, claimUuid, claimUuid);

            var schemasJson = string.Format("{{\"{0}\":{1}}}", claimUuid, schemaJson);
            var claimDefsJson = string.Format("{{\"{0}\":{1}}}", claimUuid, claimDef);
            var revocRegsJson = "{}";


            var proofJson = await AnonCreds.ProverCreateProofAsync(_proverWallet, proofRequestJson, requestedClaimsJson, schemasJson,
                    masterSecret, claimDefsJson, revocRegsJson);
            Assert.IsNotNull(proofJson);

            var proof = JObject.Parse(proofJson);

            //13. Verifier verify Proof
            Assert.AreEqual("Alex",
                    proof["requested_proof"]["revealed_attrs"]["attr1_referent"][1]);

            Assert.IsNotNull(proof["requested_proof"]["unrevealed_attrs"].Value<string>("attr2_referent"));

            Assert.AreEqual(selfAttestedValue, proof["requested_proof"]["self_attested_attrs"].Value<string>("attr3_referent"));

            Boolean valid = await AnonCreds.VerifierVerifyProofAsync(proofRequestJson, proofJson, schemasJson, claimDefsJson, revocRegsJson);
            Assert.IsTrue(valid);
        }

        [TestMethod]
        public async Task TestAnonCredsWorksForMultipleIssuerSingleProver()
        {
            var issuerDid = "NcYxiDXkpYi6ov5FcYDi1e";
            var issuerDid2 = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";

            var issuerGvtWallet = _issuerWallet;

            //1. Issuer2 Create and Open Wallet
            await Wallet.CreateWalletAsync(_poolName, "issuer2Wallet", TYPE, null, null);
            var issuerXyzWallet = await Wallet.OpenWalletAsync("issuer2Wallet", null, null);

            //2. Issuer create ClaimDef
            var gvtSchemaJson = "{\n" +
                    "                    \"seqNo\":1,\n" +
                    "                    \"data\": {\n" +
                    "                        \"name\":\"gvt\",\n" +
                    "                        \"version\":\"1.0\",\n" +
                    "                        \"attr_names\":[\"age\",\"sex\",\"height\",\"name\"]\n" +
                    "                    }\n" +
                    "                }";

            var gvtClaimDef = await AnonCreds.IssuerCreateAndStoreClaimDefAsync(issuerGvtWallet, issuerDid, gvtSchemaJson, null, false);

            //3. Issuer create ClaimDef
            var xyzSchemaJson = "{\n" +
                    "                    \"seqNo\":2,\n" +
                    "                    \"data\": {\n" +
                    "                        \"name\":\"xyz\",\n" +
                    "                        \"version\":\"1.0\",\n" +
                    "                        \"attr_names\":[\"status\",\"period\"]\n" +
                    "                    }\n" +
                    "                }";

            var xyzClaimDef = await AnonCreds.IssuerCreateAndStoreClaimDefAsync(issuerXyzWallet, issuerDid2, xyzSchemaJson, null, false);

            //4. Prover create Master Secret
            var masterSecret = "masterSecretName";
            await AnonCreds.ProverCreateMasterSecretAsync(_proverWallet, masterSecret);

            //5. Prover store Claim Offer received from Issuer1
            var claimOffer = string.Format("{{\"issuer_did\":\"{0}\", \"schema_seq_no\":{1}}}", issuerDid, 1);
            await AnonCreds.ProverStoreClaimOfferAsync(_proverWallet, claimOffer);

            //6. Prover store Claim Offer received from Issuer2
            var claimOffer2 = string.Format("{{\"issuer_did\":\"{0}\", \"schema_seq_no\":{1}}}", issuerDid2, 2);
            await AnonCreds.ProverStoreClaimOfferAsync(_proverWallet, claimOffer2);

            //7. Prover get Claim Offers
            var claimOffersJson = await AnonCreds.ProverGetClaimOffersAsync(_proverWallet, "{}");

            var claimOffersObject = JArray.Parse(claimOffersJson);
            Assert.AreEqual(2, claimOffersObject.Count);

            var claimOfferObj1 = claimOffersObject[0];
            var claimOfferObj2 = claimOffersObject[1];

            var gvtClaimOffer = claimOfferObj1.Value<string>("issuer_did").Equals(issuerDid) ? claimOfferObj1.ToString() : claimOfferObj2.ToString();
            var xyzClaimOffer = claimOfferObj1.Value<string>("issuer_did").Equals(issuerDid2) ? claimOfferObj1.ToString() : claimOfferObj2.ToString();


            //8. Prover create ClaimReq for GVT Claim Offer
            var proverDid = "BzfFCYk";
            var gvtClaimReq = await AnonCreds.ProverCreateAndStoreClaimReqAsync(_proverWallet, proverDid, gvtClaimOffer, gvtClaimDef, masterSecret);

            //9. Issuer create Claim
            var gvtClaimAttributesJson = "{\n" +
                    "               \"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\n" +
                    "               \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\n" +
                    "               \"height\":[\"175\",\"175\"],\n" +
                    "               \"age\":[\"28\",\"28\"]\n" +
                    "        }";

            var gvtCreateClaimResult = await AnonCreds.IssuerCreateClaimAsync(issuerGvtWallet, gvtClaimReq, gvtClaimAttributesJson, -1);
            var gvtClaimJson = gvtCreateClaimResult.ClaimJson;

            //10. Prover store Claim
            await AnonCreds.ProverStoreClaimAsync(_proverWallet, gvtClaimJson, gvtCreateClaimResult.RevocRegUpdateJson);

            //11. Prover create ClaimReq for GVT Claim Offer
            var xyzClaimReq = await AnonCreds.ProverCreateAndStoreClaimReqAsync(_proverWallet, proverDid, xyzClaimOffer, xyzClaimDef, masterSecret);

            //12. Issuer create Claim
            var xyzClaimAttributesJson = "{\n" +
                    "               \"status\":[\"partial\",\"51792877103171595686471452153480627530895\"],\n" +
                    "               \"period\":[\"8\",\"8\"]\n" +
                    "        }";

            var xyzCreateClaimResult = await AnonCreds.IssuerCreateClaimAsync(issuerXyzWallet, xyzClaimReq, xyzClaimAttributesJson, -1);
            var xyzClaimJson = xyzCreateClaimResult.ClaimJson;

            //13. Prover store Claim
            await AnonCreds.ProverStoreClaimAsync(_proverWallet, xyzClaimJson, xyzCreateClaimResult.RevocRegUpdateJson);

            //14. Prover gets Claims for Proof Request
            var proofRequestJson = "{\n" +
                    "                          \"nonce\":\"123432421212\",\n" +
                    "                          \"name\":\"proof_req_1\",\n" +
                    "                          \"version\":\"0.1\",\n" +
                    "                          \"requested_attrs\":{\"attr1_referent\":{\"name\":\"name\",\"restrictions\":[{\"schema_seq_no\":1}]},\n" +
                    "                                               \"attr2_referent\":{\"name\":\"status\",\"restrictions\":[{\"schema_seq_no\":2}]}},\n" +
                    "                          \"requested_predicates\":{\"predicate1_referent\":{\"attr_name\":\"age\",\"p_type\":\">=\",\"value\":18}," +
                    "                                                    \"predicate2_referent\":{\"attr_name\":\"period\",\"p_type\":\">=\",\"value\":5}}\n" +
                    "                  }";


            var claimsForProofJson = await AnonCreds.ProverGetClaimsForProofReqAsync(_proverWallet, proofRequestJson);
            Assert.IsNotNull(claimsForProofJson);

            var claimsForProof = JObject.Parse(claimsForProofJson);
            var claimsForAttribute1 = (JArray)claimsForProof["attrs"]["attr1_referent"];
            var claimsForAttribute2 = (JArray)claimsForProof["attrs"]["attr2_referent"];
            var claimsForPredicate1 = (JArray)claimsForProof["predicates"]["predicate1_referent"];
            var claimsForPredicate2 = (JArray)claimsForProof["predicates"]["predicate2_referent"];

            Assert.AreEqual(claimsForAttribute1.Count, 1);
            Assert.AreEqual(claimsForAttribute2.Count, 1);
            Assert.AreEqual(claimsForPredicate1.Count, 1);
            Assert.AreEqual(claimsForPredicate2.Count, 1);

            var claimUuidForAttr1 = claimsForAttribute1[0].Value<string>("referent");
            var claimUuidForAttr2 = claimsForAttribute2[0].Value<string>("referent");
            var claimUuidForPredicate1 = claimsForPredicate1[0].Value<string>("referent");
            var claimUuidForPredicate2 = claimsForPredicate2[0].Value<string>("referent");

            //15. Prover create Proof
            var requestedClaimsJson = string.Format("{{\n" +
                    "                                          \"self_attested_attributes\":{{}},\n" +
                    "                                          \"requested_attrs\":{{\"attr1_referent\":[\"{0}\", true],\n" +
                    "                                                               \"attr2_referent\":[\"{1}\", true]}},\n" +
                    "                                          \"requested_predicates\":{{\"predicate1_referent\":\"{2}\"," +
                    "                                                                    \"predicate2_referent\":\"{3}\"}}\n" +
                    "                                        }}", claimUuidForAttr1, claimUuidForAttr2, claimUuidForPredicate1, claimUuidForPredicate2);

            var schemasJson = string.Format("{{\"{0}\":{1}, \"{2}\":{3}}}", claimUuidForAttr1, gvtSchemaJson, claimUuidForAttr2, xyzSchemaJson);
            var claimDefsJson = string.Format("{{\"{0}\":{1}, \"{2}\":{3}}}", claimUuidForAttr1, gvtClaimDef, claimUuidForAttr2, xyzClaimDef);

            var revocRegsJson = "{}";

            var proofJson = await AnonCreds.ProverCreateProofAsync(_proverWallet, proofRequestJson, requestedClaimsJson, schemasJson,
                    masterSecret, claimDefsJson, revocRegsJson);
            Assert.IsNotNull(proofJson);

            var proof = JObject.Parse(proofJson);

            //16. Verifier verify Proof
            Assert.AreEqual("Alex",
                    proof["requested_proof"]["revealed_attrs"]["attr1_referent"].Value<string>(1));

            Assert.AreEqual("partial",
                    proof["requested_proof"]["revealed_attrs"]["attr2_referent"].Value<string>(1));

            Boolean valid = await AnonCreds.VerifierVerifyProofAsync(proofRequestJson, proofJson, schemasJson, claimDefsJson, revocRegsJson);
            Assert.IsTrue(valid);

            //18. Close and delete Issuer2 Wallet
            await issuerXyzWallet.CloseAsync();
            await Wallet.DeleteWalletAsync("issuer2Wallet", null);
        }

        [TestMethod]
        public async Task TestAnonCredsWorksForSingleIssuerSingleProverMultipleClaims()
        {
            var issuerDid = "NcYxiDXkpYi6ov5FcYDi1e";

            //1. Issuer create ClaimDef
            var gvtSchemaJson = "{\n" +
                    "                    \"seqNo\":1,\n" +
                    "                    \"data\": {\n" +
                    "                        \"name\":\"gvt\",\n" +
                    "                        \"version\":\"1.0\",\n" +
                    "                        \"attr_names\":[\"age\",\"sex\",\"height\",\"name\"]\n" +
                    "                    }\n" +
                    "                }";

            var gvtClaimDef = await AnonCreds.IssuerCreateAndStoreClaimDefAsync(_issuerWallet, issuerDid, gvtSchemaJson, null, false);

            //2. Issuer create ClaimDef
            var xyzSchemaJson = "{\n" +
                    "                    \"seqNo\":2,\n" +
                    "                    \"data\": {\n" +
                    "                        \"name\":\"xyz\",\n" +
                    "                        \"version\":\"1.0\",\n" +
                    "                        \"attr_names\":[\"status\",\"period\"]\n" +
                    "                    }\n" +
                    "                }";

            var xyzClaimDef = await AnonCreds.IssuerCreateAndStoreClaimDefAsync(_issuerWallet, issuerDid, xyzSchemaJson, null, false);

            //3. Prover create Master Secret
            var masterSecret = "masterSecretName";
            await AnonCreds.ProverCreateMasterSecretAsync(_proverWallet, masterSecret);

            //4. Prover store Claim Offer received from Issuer
            var claimOffer = string.Format("{{\"issuer_did\":\"{0}\", \"schema_seq_no\":{1}}}", issuerDid, 1);
            await AnonCreds.ProverStoreClaimOfferAsync(_proverWallet, claimOffer);

            //5. Prover store Claim Offer received from Issuer
            var claimOffer2 = string.Format("{{\"issuer_did\":\"{0}\", \"schema_seq_no\":{1}}}", issuerDid, 2);
            await AnonCreds.ProverStoreClaimOfferAsync(_proverWallet, claimOffer2);

            //6. Prover get Claim Offers
            var claimOffersJson = await AnonCreds.ProverGetClaimOffersAsync(_proverWallet, "{}");

            var claimOffersObject = JArray.Parse(claimOffersJson);
            Assert.AreEqual(2, claimOffersObject.Count);

            var claimOfferObj1 = claimOffersObject[0];
            var claimOfferObj2 = claimOffersObject[1];

            var gvtClaimOffer = claimOfferObj1.Value<int>("schema_seq_no") == 1 ? claimOfferObj1.ToString() : claimOfferObj2.ToString();
            var xyzClaimOffer = claimOfferObj1.Value<int>("schema_seq_no") == 2 ? claimOfferObj1.ToString() : claimOfferObj2.ToString();


            //7. Prover create ClaimReq for GVT Claim Offer
            var proverDid = "BzfFCYk";
            var gvtClaimReq = await AnonCreds.ProverCreateAndStoreClaimReqAsync(_proverWallet, proverDid, gvtClaimOffer, gvtClaimDef, masterSecret);

            //8. Issuer create Claim
            var gvtClaimAttributesJson = "{\n" +
                    "               \"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\n" +
                    "               \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\n" +
                    "               \"height\":[\"175\",\"175\"],\n" +
                    "               \"age\":[\"28\",\"28\"]\n" +
                    "        }";

            var gvtCreateClaimResult = await AnonCreds.IssuerCreateClaimAsync(_issuerWallet, gvtClaimReq, gvtClaimAttributesJson, -1);
            var gvtClaimJson = gvtCreateClaimResult.ClaimJson;

            //9. Prover store Claim
            await AnonCreds.ProverStoreClaimAsync(_proverWallet, gvtClaimJson, gvtCreateClaimResult.RevocRegUpdateJson);

            //10. Prover create ClaimReq for GVT Claim Offer
            var xyzClaimReq = await AnonCreds.ProverCreateAndStoreClaimReqAsync(_proverWallet, proverDid, xyzClaimOffer, xyzClaimDef, masterSecret);

            //11. Issuer create Claim
            var xyzClaimAttributesJson = "{\n" +
                    "               \"status\":[\"partial\",\"51792877103171595686471452153480627530895\"],\n" +
                    "               \"period\":[\"8\",\"8\"]\n" +
                    "        }";

            var xyzCreateClaimResult = await AnonCreds.IssuerCreateClaimAsync(_issuerWallet, xyzClaimReq, xyzClaimAttributesJson, -1);
            var xyzClaimJson = xyzCreateClaimResult.ClaimJson;

            //12. Prover store Claim
            await AnonCreds.ProverStoreClaimAsync(_proverWallet, xyzClaimJson, xyzCreateClaimResult.RevocRegUpdateJson);

            //13. Prover gets Claims for Proof Request
            var proofRequestJson = "{\n" +
                    "                          \"nonce\":\"123432421212\",\n" +
                    "                          \"name\":\"proof_req_1\",\n" +
                    "                          \"version\":\"0.1\",\n" +
                    "                          \"requested_attrs\":{\"attr1_referent\":{\"name\":\"name\",\"restrictions\":[{\"schema_seq_no\":1}]}},\n" +
                    "                          \"requested_predicates\":{\"predicate1_referent\":{\"attr_name\":\"age\",\"p_type\":\">=\",\"value\":18}," +
                    "                                                    \"predicate2_referent\":{\"attr_name\":\"period\",\"p_type\":\">=\",\"value\":5}}\n" +
                    "                  }";


            var claimsForProofJson = await AnonCreds.ProverGetClaimsForProofReqAsync(_proverWallet, proofRequestJson);
            Assert.IsNotNull(claimsForProofJson);

            var claimsForProof = JObject.Parse(claimsForProofJson);
            var claimsForAttribute1 = (JArray)claimsForProof["attrs"]["attr1_referent"];
            var claimsForPredicate1 = (JArray)claimsForProof["predicates"]["predicate1_referent"];
            var claimsForPredicate2 = (JArray)claimsForProof["predicates"]["predicate2_referent"];

            Assert.AreEqual(claimsForAttribute1.Count, 1);
            Assert.AreEqual(claimsForPredicate1.Count, 1);
            Assert.AreEqual(claimsForPredicate2.Count, 1);

            var claimUuidForAttr1 = claimsForAttribute1[0].Value<string>("referent");
            var claimUuidForPredicate1 = claimsForPredicate1[0].Value<string>("referent");
            var claimUuidForPredicate2 = claimsForPredicate2[0].Value<string>("referent");

            //14. Prover create Proof
            var requestedClaimsJson = string.Format("{{\n" +
                    "                                          \"self_attested_attributes\":{{}},\n" +
                    "                                          \"requested_attrs\":{{\"attr1_referent\":[\"{0}\", true]}},\n" +
                    "                                          \"requested_predicates\":{{\"predicate1_referent\":\"{1}\"," +
                    "                                                                    \"predicate2_referent\":\"{2}\"}}\n" +
                    "                                        }}", claimUuidForAttr1, claimUuidForPredicate1, claimUuidForPredicate2);

            var schemasJson = string.Format("{{\"{0}\":{1}, \"{2}\":{3}}}", claimUuidForAttr1, gvtSchemaJson, claimUuidForPredicate2, xyzSchemaJson);
            var claimDefsJson = string.Format("{{\"{0}\":{1}, \"{2}\":{3}}}", claimUuidForAttr1, gvtClaimDef, claimUuidForPredicate2, xyzClaimDef);

            var revocRegsJson = "{}";

            var proofJson = await AnonCreds.ProverCreateProofAsync(_proverWallet, proofRequestJson, requestedClaimsJson, schemasJson,
                    masterSecret, claimDefsJson, revocRegsJson);
            Assert.IsNotNull(proofJson);

            var proof = JObject.Parse(proofJson);

            //15. Verifier verify Proof
            Assert.AreEqual("Alex",
                    proof["requested_proof"]["revealed_attrs"]["attr1_referent"][1]);

            var valid = await AnonCreds.VerifierVerifyProofAsync(proofRequestJson, proofJson, schemasJson, claimDefsJson, revocRegsJson);
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
                    "                        \"attr_names\":[\"age\",\"sex\",\"height\",\"name\"]\n" +
                    "                    }\n" +
                    "                }";
            var issuerDid = "NcYxiDXkpYi6ov5FcYDi1e";

            var claimDef = await AnonCreds.IssuerCreateAndStoreClaimDefAsync(_issuerWallet, issuerDid, schemaJson, null, false);

            Assert.IsNotNull(claimDef);

            //2. Prover create Master Secret
            var masterSecret = "masterSecretName";
            await AnonCreds.ProverCreateMasterSecretAsync(_proverWallet, masterSecret);

            //3. Prover store Claim Offer
            var claimOffer = string.Format("{{\"issuer_did\":\"{0}\", \"schema_seq_no\":{1}}}", issuerDid, 1);
            await AnonCreds.ProverStoreClaimOfferAsync(_proverWallet, claimOffer);

            //4. Prover get Claim Offers
            var claimOfferFilter = string.Format("{{\"issuer_did\":\"{0}\"}}", issuerDid);
            var claimOffersJson = await AnonCreds.ProverGetClaimOffersAsync(_proverWallet, claimOfferFilter);

            var claimOffersObject = JArray.Parse(claimOffersJson);

            Assert.AreEqual(claimOffersObject.Count, 1);

            var claimOfferObject = claimOffersObject[0];
            var claimOfferJson = claimOfferObject.ToString();

            //5. Prover create ClaimReq
            var proverDid = "BzfFCYk";
            var claimReq = await AnonCreds.ProverCreateAndStoreClaimReqAsync(_proverWallet, proverDid, claimOfferJson, claimDef, masterSecret);

            Assert.IsNotNull(claimReq);

            //6. Issuer create Claim
            var claimAttributesJson = "{\n" +
                    "               \"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\n" +
                    "               \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\n" +
                    "               \"height\":[\"175\",\"175\"],\n" +
                    "               \"age\":[\"28\",\"28\"]\n" +
                    "        }";

            var createClaimResult = await AnonCreds.IssuerCreateClaimAsync(_issuerWallet, claimReq, claimAttributesJson, -1);

            Assert.IsNotNull(createClaimResult);
            var claimJson = createClaimResult.ClaimJson;

            //7. Prover store Claim
            await AnonCreds.ProverStoreClaimAsync(_proverWallet, claimJson, createClaimResult.RevocRegUpdateJson);

            //8. Prover gets Claims for Proof Request
            var proofRequestJson = "{\n" +
                    "                          \"nonce\":\"123432421212\",\n" +
                    "                          \"name\":\"proof_req_1\",\n" +
                    "                          \"version\":\"0.1\",\n" +
                    "                          \"requested_attrs\":{\"attr1_referent\":{\"name\":\"name\",\"restrictions\":[{\"schema_seq_no\":1}]}, \"attr2_referent\":{\"name\":\"phone\"}},\n" +
                    "                          \"requested_predicates\":{}\n" +
                    "                  }";

            var claimsForProofJson = await AnonCreds.ProverGetClaimsForProofReqAsync(_proverWallet, proofRequestJson);

            Assert.IsNotNull(claimsForProofJson);

            var claimsForProof = JObject.Parse(claimsForProofJson);
            var claimsForAttribute1 = (JArray)claimsForProof["attrs"]["attr1_referent"];


            Assert.AreEqual(claimsForAttribute1.Count, 1);

            var claimUuid = claimsForAttribute1[0].Value<string>("referent");

            //9. Prover create Proof
            var selfAttestedValue = "yes";
            var requestedClaimsJson = string.Format("{{\n" +
                    "                                          \"self_attested_attributes\":{{\"self1\":\"{0}\"}},\n" +
                    "                                          \"requested_attrs\":{{\"attr1_referent\":[\"{1}\", true]}},\n" +
                    "                                          \"requested_predicates\":{{}}\n" +
                    "                                        }}", selfAttestedValue, claimUuid);

            var schemasJson = string.Format("{{\"{0}\":{1}}}", claimUuid, schemaJson);
            var claimDefsJson = string.Format("{{\"{0}\":{1}}}", claimUuid, claimDef);
            var revocRegsJson = "{}";

            //TODO: Not sure why this call is failing...
            var proofJson = await AnonCreds.ProverCreateProofAsync(_proverWallet, proofRequestJson, requestedClaimsJson, schemasJson,
                    masterSecret, claimDefsJson, revocRegsJson);

            Assert.IsNotNull(proofJson);

            var proof = JObject.Parse(proofJson);

            //10. Verifier verify Proof
            Assert.AreEqual("Alex",
                    proof["requested_proof"]["revealed_attrs"]["attr1_referent"][1]);


            Assert.AreEqual(selfAttestedValue, proof["requested_proof"]["self_attested_attrs"].Value<string>("self1"));

            proofRequestJson = "{\n" +
                    "                            \"nonce\":\"123432421212\",\n" +
                    "                        \"name\":\"proof_req_1\",\n" +
                    "                        \"version\":\"0.1\",\n" +
                    "                    \"requested_attrs\":{\"attr1_referent\":{\"name\":\"name\",\"restrictions\":[{\"schema_seq_no\":1}]}},\n" +
                    "                    \"requested_predicates\":{\"predicate1_referent\":{\"attr_name\":\"age\",\"p_type\":\">=\",\"value\":18}\n" +
                    "           }";

            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
              AnonCreds.VerifierVerifyProofAsync(proofRequestJson, proofJson, schemasJson, claimDefsJson, revocRegsJson)
            );
        }
    }
}
