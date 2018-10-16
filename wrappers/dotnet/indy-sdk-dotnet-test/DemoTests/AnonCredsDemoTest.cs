using Hyperledger.Indy.AnonCredsApi;
using Hyperledger.Indy.BlobStorageApi;
using Hyperledger.Indy.PoolApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;
using System;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.DemoTests
{
    [TestClass]
    public class AnonCredsDemoTest : IndyIntegrationTestBase
    {
        private Pool pool;
        private string issuerWalletConfig = JsonConvert.SerializeObject(new { id = "issuerWallet" });
        private Wallet issuerWallet;
        private string proverWalletConfig = JsonConvert.SerializeObject(new { id = "proverWallet" });
        private Wallet proverWallet;
        private string masterSecretId = "masterSecretId";
        private string credentialId1 = "id1";
        private string credentialId2 = "id2";
        private string issuerDid = "NcYxiDXkpYi6ov5FcYDi1e";
        private string proverDid = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";
        private string gvtCredentialValues = GVT_CRED_VALUES;
        private string xyzCredentialValues = JsonConvert.SerializeObject(
            new {
                status = new
                {
                    raw = "partial",
                    encoded = "51792877103171595686471452153480627530895"
                },
                period = new
                {
                    raw = "8",
                    encoded = "8"
                }
            }
        );

        [TestInitialize]
        public async Task CreateWallet()
        {
            // Set protocol version
            await Pool.SetProtocolVersionAsync(PROTOCOL_VERSION);

            // Create and Open Pool
            var poolName = PoolUtils.CreatePoolLedgerConfig();
            pool = await Pool.OpenPoolLedgerAsync(poolName, "{}");

            // Issuer Create and Open Wallet
            await Wallet.CreateWalletAsync(issuerWalletConfig, WALLET_CREDENTIALS);
            issuerWallet = await Wallet.OpenWalletAsync(issuerWalletConfig, WALLET_CREDENTIALS);

            // Prover Create and Open Wallet
            await Wallet.CreateWalletAsync(proverWalletConfig, WALLET_CREDENTIALS);
            proverWallet = await Wallet.OpenWalletAsync(proverWalletConfig, WALLET_CREDENTIALS);
        }

        [TestCleanup]
        public async Task DeleteWallet()
        {
            await issuerWallet.CloseAsync();
            await Wallet.DeleteWalletAsync(issuerWalletConfig, WALLET_CREDENTIALS);

            await proverWallet.CloseAsync();
            await Wallet.DeleteWalletAsync(proverWalletConfig, WALLET_CREDENTIALS);

            await pool.CloseAsync();
        }


        [TestMethod]
        public async Task TestAnonCredsDemo()
        {
            var createSchemaResult = await AnonCreds.IssuerCreateSchemaAsync(issuerDid, GVT_SCHEMA_NAME, SCHEMA_VERSION, GVT_SCHEMA_ATTRIBUTES);
            var gvtSchemaId = createSchemaResult.SchemaId;
            var gvtSchema = createSchemaResult.SchemaJson;

            // Issuer create CredentialDef
            var createCredDefResult = await AnonCreds.IssuerCreateAndStoreCredentialDefAsync(issuerWallet, issuerDid, gvtSchema, TAG, null, DEFAULT_CRED_DEF_CONFIG);
            var credDefId = createCredDefResult.CredDefId;
            var credDef = createCredDefResult.CredDefJson;

            // Prover create Master Secret
            await AnonCreds.ProverCreateMasterSecretAsync(proverWallet, masterSecretId);

            // Issuer create Credential Offer
            var credOffer = await AnonCreds.IssuerCreateCredentialOfferAsync(issuerWallet, credDefId);

            // Prover create CredentialReq
            var createCredReqResult = await AnonCreds.ProverCreateCredentialReqAsync(proverWallet, proverDid, credOffer, credDef, masterSecretId);
            var credReq = createCredReqResult.CredentialRequestJson;
            var credReqMetadata = createCredReqResult.CredentialRequestMetadataJson;

            // Issuer create Credential
            var createCredentialResult = await AnonCreds.IssuerCreateCredentialAsync(issuerWallet, credOffer, credReq, gvtCredentialValues, null, null);
            var credential = createCredentialResult.CredentialJson;

            // Prover store Credential
            await AnonCreds.ProverStoreCredentialAsync(proverWallet, credentialId1, credReqMetadata, credential, credDef, null);

            // Prover gets Credentials for Proof Request
            var proofRequestJson = "{" +
                    "                    \"nonce\":\"123432421212\",\n" +
                    "                    \"name\":\"proof_req_1\",\n" +
                    "                    \"version\":\"0.1\", " +
                    "                    \"requested_attributes\": {" +
                    "                          \"attr1_referent\":{\"name\":\"name\"}," +
                    "                          \"attr2_referent\":{\"name\":\"sex\"}," +
                    "                          \"attr3_referent\":{\"name\":\"phone\"}" +
                    "                     }," +
                    "                    \"requested_predicates\":{" +
                    "                         \"predicate1_referent\":{\"name\":\"age\",\"p_type\":\">=\",\"p_value\":18}" +
                    "                    }" +
                    "                  }";

            var credentialsForProofJson = await AnonCreds.ProverGetCredentialsForProofReqAsync(proverWallet, proofRequestJson);

            var credentialsForProof = JObject.Parse(credentialsForProofJson);
            var credentialsForAttribute1 = (JArray)credentialsForProof["attrs"]["attr1_referent"];
            var credentialsForAttribute2 = (JArray)credentialsForProof["attrs"]["attr2_referent"];
            var credentialsForAttribute3 = (JArray)credentialsForProof["attrs"]["attr3_referent"];
            var credentialsForPredicate = (JArray)credentialsForProof["predicates"]["predicate1_referent"];

            Assert.AreEqual(1, credentialsForAttribute1.Count);
            Assert.AreEqual(1, credentialsForAttribute2.Count);
            Assert.AreEqual(0, credentialsForAttribute3.Count);
            Assert.AreEqual(1, credentialsForPredicate.Count);

            var credentialUuid = credentialsForAttribute1[0]["cred_info"]["referent"];

            // Prover create Proof
            var selfAttestedValue = "8-800-300";
            var requestedCredentialsJson = JsonConvert.SerializeObject(
                new
                {
                    self_attested_attributes = new
                    {
                        attr3_referent = selfAttestedValue
                    },
                    requested_attributes = new
                    {
                        attr1_referent = new { cred_id = credentialUuid, revealed = true },
                        attr2_referent = new { cred_id = credentialUuid, revealed = false }
                    },
                    requested_predicates = new
                    {
                        predicate1_referent = new { cred_id = credentialUuid }
                    }
                }
            );

            var schemas = new JObject(new JProperty(gvtSchemaId, JObject.Parse(gvtSchema))).ToString();
            var credentialDefs = new JObject(new JProperty(credDefId, JObject.Parse(credDef))).ToString();
            var revocStates = "{}";

            var proofJson = await AnonCreds.ProverCreateProofAsync(proverWallet, proofRequestJson, requestedCredentialsJson,
                    masterSecretId, schemas, credentialDefs, revocStates);
            var proof = JObject.Parse(proofJson);

            // Verifier verify Proof
            var revealedAttr1 = proof["requested_proof"]["revealed_attrs"]["attr1_referent"];

            Assert.AreEqual("Alex", revealedAttr1["raw"]);
            Assert.IsNotNull(proof["requested_proof"]["unrevealed_attrs"]["attr2_referent"]["sub_proof_index"]);
            Assert.AreEqual(selfAttestedValue, proof["requested_proof"]["self_attested_attrs"]["attr3_referent"]);

            var revocRegDefs = "{}";
            var revocRegs = "{}";

            var valid = await AnonCreds.VerifierVerifyProofAsync(proofRequestJson, proofJson, schemas, credentialDefs, revocRegDefs, revocRegs);
            Assert.IsTrue(valid);
        }

        [TestMethod]
        public async Task TestAnonCredsWorksForMultipleIssuerSingleProver()
        {
            var issuerGvtWallet = issuerWallet;

            // Issuer2 Create and Open Wallet
            var issuer2WalletConfig = JsonConvert.SerializeObject(new { id = "issuer2Wallet" });
            await Wallet.CreateWalletAsync(issuer2WalletConfig, WALLET_CREDENTIALS);
            var issuerXyzWallet = await Wallet.OpenWalletAsync(issuer2WalletConfig, WALLET_CREDENTIALS);

            // Issuer1 create GVT Schema
            var createSchemaResult = await AnonCreds.IssuerCreateSchemaAsync(issuerDid, GVT_SCHEMA_NAME, SCHEMA_VERSION, GVT_SCHEMA_ATTRIBUTES);
            var gvtSchemaId = createSchemaResult.SchemaId;
            var gvtSchema = createSchemaResult.SchemaJson;

            // Issuer1 create CredentialDef
            var createCredDefResult = await AnonCreds.IssuerCreateAndStoreCredentialDefAsync(issuerGvtWallet, issuerDid, gvtSchema, TAG, null, DEFAULT_CRED_DEF_CONFIG);
            var gvtCredDefId = createCredDefResult.CredDefId;
            var gvtCredDef = createCredDefResult.CredDefJson;

            // Issuer2 create XYZ Schema
            var issuerDid2 = "VsKV7grR1BUE29mG2Fm2kX";

            // Issuer2 create XYZ Schema
            createSchemaResult = await AnonCreds.IssuerCreateSchemaAsync(issuerDid2, XYZ_SCHEMA_NAME, SCHEMA_VERSION, XYZ_SCHEMA_ATTRIBUTES);
            var xyzSchemaId = createSchemaResult.SchemaId;
            var xyzSchema = createSchemaResult.SchemaJson;

            //5. Issuer create CredentialDef
            createCredDefResult = await AnonCreds.IssuerCreateAndStoreCredentialDefAsync(issuerXyzWallet, issuerDid2, xyzSchema, TAG, null, DEFAULT_CRED_DEF_CONFIG);
            var xyzCredDefId = createCredDefResult.CredDefId;
            var xyzCredDef = createCredDefResult.CredDefJson;

            // Prover create Master Secret
            await AnonCreds.ProverCreateMasterSecretAsync(proverWallet, masterSecretId);

            // Issuer1 create Credential Offer
            var gvtCredOffer = await AnonCreds.IssuerCreateCredentialOfferAsync(issuerGvtWallet, gvtCredDefId);

            // Issuer2 create Credential Offer
            var xyzCredOffer = await AnonCreds.IssuerCreateCredentialOfferAsync(issuerXyzWallet, xyzCredDefId);

            // Prover create Credential Request for GVT Credential Offer
            var createCredReqResult = await AnonCreds.ProverCreateCredentialReqAsync(proverWallet, proverDid, gvtCredOffer, gvtCredDef, masterSecretId);
            var gvtCredReq = createCredReqResult.CredentialRequestJson;
            var gvtCredReqMetadata = createCredReqResult.CredentialRequestMetadataJson;

            // Issuer create Credential
            var gvtCreateCredentialResult = await AnonCreds.IssuerCreateCredentialAsync(issuerGvtWallet, gvtCredOffer, gvtCredReq, gvtCredentialValues, null, null);
            var gvtCredential = gvtCreateCredentialResult.CredentialJson;

            // Prover store Credential
            await AnonCreds.ProverStoreCredentialAsync(proverWallet, credentialId1, gvtCredReqMetadata, gvtCredential, gvtCredDef, null);

            // Prover create CredentialReq for GVT Credential Offer
            createCredReqResult = await AnonCreds.ProverCreateCredentialReqAsync(proverWallet, proverDid, xyzCredOffer, xyzCredDef, masterSecretId);
            var xyzCredReq = createCredReqResult.CredentialRequestJson;
            var xyzCredReqMetadata = createCredReqResult.CredentialRequestMetadataJson;

            // Issuer create Credential
            var xyzCreateCredentialResult = await AnonCreds.IssuerCreateCredentialAsync(issuerXyzWallet, xyzCredOffer, xyzCredReq, xyzCredentialValues, null, null);
            var xyzCredential = xyzCreateCredentialResult.CredentialJson;

            // Prover store Credential
            await AnonCreds.ProverStoreCredentialAsync(proverWallet, credentialId2, xyzCredReqMetadata, xyzCredential, xyzCredDef, null);

            // Prover gets Credentials for Proof Request
            var proofRequestJson = JsonConvert.SerializeObject(
                new
                {
                    nonce = "123432421212",
                    name = "proof_req_1",
                    version = "0.1",
                    requested_attributes = new
                    {
                        attr1_referent = new { name = "name" },
                        attr2_referent = new { name = "status" }
                    },
                    requested_predicates = new
                    {
                        predicate1_referent = new { name = "age", p_type = ">=", p_value = 18 },
                        predicate2_referent = new { name = "period", p_type = ">=", p_value = 5 },
                    }
                }
            );

            var credentialsForProofJson = await AnonCreds.ProverGetCredentialsForProofReqAsync(proverWallet, proofRequestJson);
            Assert.IsNotNull(credentialsForProofJson);

            var credentialsForProof = JObject.Parse(credentialsForProofJson);
            var credentialsForAttribute1 = (JArray)credentialsForProof["attrs"]["attr1_referent"];
            var credentialsForAttribute2 = (JArray)credentialsForProof["attrs"]["attr2_referent"];
            var credentialsForPredicate1 = (JArray)credentialsForProof["predicates"]["predicate1_referent"];
            var credentialsForPredicate2 = (JArray)credentialsForProof["predicates"]["predicate2_referent"];

            Assert.AreEqual(1, credentialsForAttribute1.Count);
            Assert.AreEqual(1, credentialsForAttribute2.Count);
            Assert.AreEqual(1, credentialsForPredicate1.Count);
            Assert.AreEqual(1, credentialsForPredicate2.Count);

            var credentialUuidForAttr1 = credentialsForAttribute1[0]["cred_info"]["referent"];
            var credentialUuidForAttr2 = credentialsForAttribute2[0]["cred_info"]["referent"];
            var credentialUuidForPredicate1 = credentialsForPredicate1[0]["cred_info"]["referent"];
            var credentialUuidForPredicate2 = credentialsForPredicate2[0]["cred_info"]["referent"];

            // Prover create Proof
            var requestedCredentialsJson = JsonConvert.SerializeObject(
                new
                {
                    self_attested_attributes = new { },
                    requested_attributes = new
                    {
                        attr1_referent = new { cred_id = credentialUuidForAttr1, revealed = true },
                        attr2_referent = new { cred_id = credentialUuidForAttr2, revealed = true }
                    },
                    requested_predicates = new
                    {
                        predicate1_referent = new { cred_id = credentialUuidForPredicate1 },
                        predicate2_referent = new { cred_id = credentialUuidForPredicate2 }
                    }
                }
            );

            var schemas = new JObject(new JProperty(gvtSchemaId, JObject.Parse(gvtSchema)), new JProperty(xyzSchemaId, JObject.Parse(xyzSchema))).ToString();
            var credentialDefs = new JObject(new JProperty(gvtCredDefId, JObject.Parse(gvtCredDef)), new JProperty(xyzCredDefId, JObject.Parse(xyzCredDef))).ToString();
            var revocStates = "{}";

            var proofJson = await AnonCreds.ProverCreateProofAsync(proverWallet, proofRequestJson, requestedCredentialsJson,
                    masterSecretId, schemas, credentialDefs, revocStates);
            var proof = JObject.Parse(proofJson);

            // Verifier verify Proof
            var revealedAttr1 = proof["requested_proof"]["revealed_attrs"]["attr1_referent"];
            Assert.AreEqual("Alex", revealedAttr1["raw"]);

            var revealedAttr2 = proof["requested_proof"]["revealed_attrs"]["attr2_referent"];
            Assert.AreEqual("partial", revealedAttr2["raw"]);

            var revocRegDefs = "{}";
            var revocRegs = "{}";

            var valid = await AnonCreds.VerifierVerifyProofAsync(proofRequestJson, proofJson, schemas, credentialDefs, revocRegDefs, revocRegs);
            Assert.IsTrue(valid);

            // Close and delete Issuer2 Wallet
            await issuerXyzWallet.CloseAsync();
            await Wallet.DeleteWalletAsync(issuer2WalletConfig, WALLET_CREDENTIALS);
        }

        [TestMethod]
        public async Task TestAnonCredsWorksForSingleIssuerSingleProverMultipleCredentials()
        {
            // Issuer create GVT Schema
            var createSchemaResult = await AnonCreds.IssuerCreateSchemaAsync(issuerDid, GVT_SCHEMA_NAME, SCHEMA_VERSION, GVT_SCHEMA_ATTRIBUTES);
            var gvtSchemaId = createSchemaResult.SchemaId;
            var gvtSchema = createSchemaResult.SchemaJson;

            // Issuer create CredentialDef
            var createCredDefResult = await AnonCreds.IssuerCreateAndStoreCredentialDefAsync(issuerWallet, issuerDid, gvtSchema, TAG, null, DEFAULT_CRED_DEF_CONFIG);
            var gvtCredDefId = createCredDefResult.CredDefId;
            var gvtCredDef = createCredDefResult.CredDefJson;

            // Issuer create XYZ Schema
            createSchemaResult = await AnonCreds.IssuerCreateSchemaAsync(issuerDid, XYZ_SCHEMA_NAME, SCHEMA_VERSION, XYZ_SCHEMA_ATTRIBUTES);
            var xyzSchemaId = createSchemaResult.SchemaId;
            var xyzSchema = createSchemaResult.SchemaJson;

            // Issuer create CredentialDef
            createCredDefResult = await AnonCreds.IssuerCreateAndStoreCredentialDefAsync(issuerWallet, issuerDid, xyzSchema, TAG, null, DEFAULT_CRED_DEF_CONFIG);
            var xyzCredDefId = createCredDefResult.CredDefId;
            var xyzCredDef = createCredDefResult.CredDefJson;

            // Prover create Master Secret
            await AnonCreds.ProverCreateMasterSecretAsync(proverWallet, masterSecretId);

            // Issuer create GVT Credential Offer
            var gvtCredOffer = await AnonCreds.IssuerCreateCredentialOfferAsync(issuerWallet, gvtCredDefId);

            // Issuer create XYZ Credential Offer
            var xyzCredOffer = await AnonCreds.IssuerCreateCredentialOfferAsync(issuerWallet, xyzCredDefId);

            // Prover create CredentialReq for GVT Credential Offer
            var createCredReqResult = await AnonCreds.ProverCreateCredentialReqAsync(proverWallet, proverDid, gvtCredOffer, gvtCredDef, masterSecretId);
            var gvtCredReq = createCredReqResult.CredentialRequestJson;
            var gvtCredReqMetadata = createCredReqResult.CredentialRequestMetadataJson;

            // Issuer create GVT Credential
            var gvtCreateCredentialResult =
                    await AnonCreds.IssuerCreateCredentialAsync(issuerWallet, gvtCredOffer, gvtCredReq, gvtCredentialValues, null, null);
            var gvtCredential = gvtCreateCredentialResult.CredentialJson;

            // Prover store GVT Credential
            await AnonCreds.ProverStoreCredentialAsync(proverWallet, credentialId1, gvtCredReqMetadata, gvtCredential, gvtCredDef, null);

            // Prover create CredentialReq for XYZ Credential Offer
            createCredReqResult = await AnonCreds.ProverCreateCredentialReqAsync(proverWallet, proverDid, xyzCredOffer, xyzCredDef, masterSecretId);
            var xyzCredReq = createCredReqResult.CredentialRequestJson;
            var xyzCredReqMetadata = createCredReqResult.CredentialRequestMetadataJson;

            // Issuer create XYZ Credential
            var xyzCreateCredentialResult =
                    await AnonCreds.IssuerCreateCredentialAsync(issuerWallet, xyzCredOffer, xyzCredReq, xyzCredentialValues, null, null);
            var xyzCredential = xyzCreateCredentialResult.CredentialJson;

            // Prover store XYZ Credential
            await AnonCreds.ProverStoreCredentialAsync(proverWallet, credentialId2, xyzCredReqMetadata, xyzCredential, xyzCredDef, null);

            // Prover gets Credentials for Proof Request
            var proofRequestJson = JsonConvert.SerializeObject(
                new
                {
                    nonce = "123432421212",
                    name = "proof_req_1",
                    version = "0.1",
                    requested_attributes = new
                    {
                        attr1_referent = new { name = "name" },
                        attr2_referent = new { name = "status" }
                    },
                    requested_predicates = new
                    {
                        predicate1_referent = new { name = "age", p_type = ">=", p_value = 18 },
                        predicate2_referent = new { name = "period", p_type = ">=", p_value = 5 },
                    }
                }
            );

            var credentialsForProofJson = await AnonCreds.ProverGetCredentialsForProofReqAsync(proverWallet, proofRequestJson);
            Assert.IsNotNull(credentialsForProofJson);

            var credentialsForProof = JObject.Parse(credentialsForProofJson);
            var credentialsForAttribute1 = (JArray)credentialsForProof["attrs"]["attr1_referent"];
            var credentialsForAttribute2 = (JArray)credentialsForProof["attrs"]["attr2_referent"];
            var credentialsForPredicate1 = (JArray)credentialsForProof["predicates"]["predicate1_referent"];
            var credentialsForPredicate2 = (JArray)credentialsForProof["predicates"]["predicate2_referent"];

            Assert.AreEqual(1, credentialsForAttribute1.Count);
            Assert.AreEqual(1, credentialsForAttribute2.Count);
            Assert.AreEqual(1, credentialsForPredicate1.Count);
            Assert.AreEqual(1, credentialsForPredicate2.Count);

            var credentialUuidForAttr1 = credentialsForAttribute1[0]["cred_info"]["referent"];
            var credentialUuidForAttr2 = credentialsForAttribute2[0]["cred_info"]["referent"];
            var credentialUuidForPredicate1 = credentialsForPredicate1[0]["cred_info"]["referent"];
            var credentialUuidForPredicate2 = credentialsForPredicate2[0]["cred_info"]["referent"];

            // Prover create Proof
            var requestedCredentialsJson = JsonConvert.SerializeObject(
                new
                {
                    self_attested_attributes = new { },
                    requested_attributes = new
                    {
                        attr1_referent = new { cred_id = credentialUuidForAttr1, revealed = true },
                        attr2_referent = new { cred_id = credentialUuidForAttr2, revealed = true }
                    },
                    requested_predicates = new
                    {
                        predicate1_referent = new { cred_id = credentialUuidForPredicate1 },
                        predicate2_referent = new { cred_id = credentialUuidForPredicate2 }
                    }
                }
            );

            var schemas = new JObject(new JProperty(gvtSchemaId, JObject.Parse(gvtSchema)), new JProperty(xyzSchemaId, JObject.Parse(xyzSchema))).ToString();
            var credentialDefs = new JObject(new JProperty(gvtCredDefId, JObject.Parse(gvtCredDef)), new JProperty(xyzCredDefId, JObject.Parse(xyzCredDef))).ToString();
            var revocStates = "{}";

            var proofJson = await AnonCreds.ProverCreateProofAsync(proverWallet, proofRequestJson, requestedCredentialsJson,
                    masterSecretId, schemas, credentialDefs, revocStates);
            var proof = JObject.Parse(proofJson);

            // Verifier verify Proof
            var revealedAttr1 = proof["requested_proof"]["revealed_attrs"]["attr1_referent"];
            Assert.AreEqual("Alex", revealedAttr1["raw"]);

            var revealedAttr2 = proof["requested_proof"]["revealed_attrs"]["attr2_referent"];
            Assert.AreEqual("partial", revealedAttr2["raw"]);

            var revocRegDefs = "{}";
            var revocRegs = "{}";

            Boolean valid = await AnonCreds.VerifierVerifyProofAsync(proofRequestJson, proofJson, schemas, credentialDefs, revocRegDefs, revocRegs);
            Assert.IsTrue(valid);
        }

        [TestMethod]
        public async Task TestAnonCredsWorksForRevocationProof()
        {
            // Issuer create Schema
            var createSchemaResult = await AnonCreds.IssuerCreateSchemaAsync(issuerDid, GVT_SCHEMA_NAME, SCHEMA_VERSION, GVT_SCHEMA_ATTRIBUTES);
            var gvtSchemaId = createSchemaResult.SchemaId;
            var schemaJson = createSchemaResult.SchemaJson;

            // Issuer create credential definition
            var revocationCredentialDefConfig = JsonConvert.SerializeObject(new { support_revocation = true });
            var createCredentialDefResult = await AnonCreds.IssuerCreateAndStoreCredentialDefAsync(issuerWallet, issuerDid, schemaJson, TAG, null, revocationCredentialDefConfig);
            var credDefId = createCredentialDefResult.CredDefId;
            var credDef = createCredentialDefResult.CredDefJson;

            // Issuer create revocation registry
            var revRegConfig = JsonConvert.SerializeObject(new { issuance_type = (object)null, max_cred_num = 5 });
            var tailsWriterConfig = JsonConvert.SerializeObject(
                new
                {
                    base_dir = EnvironmentUtils.GetIndyHomePath("tails"),
                    uri_pattern = string.Empty
                }
            );
            var tailsWriter = await BlobStorage.OpenWriterAsync("default", tailsWriterConfig);

            var createRevRegResult = await AnonCreds.IssuerCreateAndStoreRevocRegAsync(issuerWallet, issuerDid, null, TAG, credDefId, revRegConfig, tailsWriter);
            var revRegId = createRevRegResult.RevRegId;
            var revRegDef = createRevRegResult.RevRegDefJson;

            // Prover create Master Secret
            await AnonCreds.ProverCreateMasterSecretAsync(proverWallet, masterSecretId);

            // Issuer create Credential Offer
            var credOffer = await AnonCreds.IssuerCreateCredentialOfferAsync(issuerWallet, credDefId);

            // Prover create Credential Request
            var createCredReqResult = await AnonCreds.ProverCreateCredentialReqAsync(proverWallet, proverDid, credOffer, credDef, masterSecretId);
            var credReq = createCredReqResult.CredentialRequestJson;
            var credReqMetadata = createCredReqResult.CredentialRequestMetadataJson;

            // Issuer open TailsReader
            var blobStorageReader = await BlobStorage.OpenReaderAsync("default", tailsWriterConfig);

            // Issuer create Credential
            var createCredentialResult = await AnonCreds.IssuerCreateCredentialAsync(issuerWallet, credOffer, credReq, gvtCredentialValues, revRegId, blobStorageReader);
            var credential = createCredentialResult.CredentialJson;
            var revRegDelta = createCredentialResult.RevocRegDeltaJson;
            var credRevId = createCredentialResult.RevocId;

            // Prover store received Credential
            await AnonCreds.ProverStoreCredentialAsync(proverWallet, credentialId1, credReqMetadata, credential, credDef, revRegDef);

            // Prover gets Credentials for Proof Request
            var proofRequestJson = JsonConvert.SerializeObject(
                new
                {
                    nonce = "123432421212",
                    name = "proof_req_1",
                    version = "0.1",
                    requested_attributes = new
                    {
                        attr1_referent = new { name = "name" },
                    },
                    requested_predicates = new
                    {
                        predicate1_referent = new { name = "age", p_type = ">=", p_value = 18 },
                    }
                }
            );

            var credentialsJson = await AnonCreds.ProverGetCredentialsForProofReqAsync(proverWallet, proofRequestJson);
            var credentials = JObject.Parse(credentialsJson);
            var credentialsForAttr1 = (JArray)credentials["attrs"]["attr1_referent"];

            var credentialUuid = credentialsForAttr1[0]["cred_info"]["referent"];

            // Prover create RevocationState
            int timestamp = 100;
            var revStateJson = await AnonCreds.CreateRevocationStateAsync(blobStorageReader, revRegDef, revRegDelta, timestamp, credRevId);


            // Prover create Proof
            var requestedCredentialsJson = JsonConvert.SerializeObject(
                new
                {
                    self_attested_attributes = new { },
                    requested_attributes = new
                    {
                        attr1_referent = new { cred_id = credentialUuid, revealed = true, timestamp = timestamp },
                    },
                    requested_predicates = new
                    {
                        predicate1_referent = new { cred_id = credentialUuid, timestamp = timestamp },
                    }
                }
            );

            var schemas = new JObject(new JProperty(gvtSchemaId, JObject.Parse(schemaJson))).ToString();
            var credentialDefs = new JObject(new JProperty(credDefId, JObject.Parse(credDef))).ToString();
            var revStates = new JObject(new JProperty(revRegId, new JObject(new JProperty(timestamp.ToString(), JObject.Parse(revStateJson))))).ToString();

            var proofJson = await AnonCreds.ProverCreateProofAsync(proverWallet, proofRequestJson, requestedCredentialsJson, masterSecretId, schemas,
                    credentialDefs, revStates);
            var proof = JObject.Parse(proofJson);

            // Verifier verify proof
            var revealedAttr1 = proof["requested_proof"]["revealed_attrs"]["attr1_referent"];
            Assert.AreEqual("Alex", revealedAttr1["raw"]);

            var revRegDefs = new JObject(new JProperty(revRegId, JObject.Parse(revRegDef))).ToString();
            var revRegs = new JObject(new JProperty(revRegId, new JObject(new JProperty(timestamp.ToString(), JObject.Parse(revRegDelta))))).ToString();

            var valid = await AnonCreds.VerifierVerifyProofAsync(proofRequestJson, proofJson, schemas, credentialDefs, revRegDefs, revRegs);
            Assert.IsTrue(valid);
        }

        [TestMethod]
        public async Task TestVerifyProofWorksForProofDoesNotCorrespondToProofRequest()
        {

            // Issuer create Schema
            var createSchemaResult = await AnonCreds.IssuerCreateSchemaAsync(issuerDid, GVT_SCHEMA_NAME, SCHEMA_VERSION, GVT_SCHEMA_ATTRIBUTES);
            var gvtSchemaId = createSchemaResult.SchemaId;
            var gvtSchema = createSchemaResult.SchemaJson;

            // Issuer create CredentialDef
            var createCredDefResult = await AnonCreds.IssuerCreateAndStoreCredentialDefAsync(issuerWallet, issuerDid, gvtSchema, TAG, null, DEFAULT_CRED_DEF_CONFIG);
            var credDefId = createCredDefResult.CredDefId;
            var credDef = createCredDefResult.CredDefJson;

            // Prover create Master Secret
            await AnonCreds.ProverCreateMasterSecretAsync(proverWallet, masterSecretId);

            // Issuer create Credential Offer
            var credOffer = await AnonCreds.IssuerCreateCredentialOfferAsync(issuerWallet, credDefId);

            // Prover create CredentialReq
            var createCredReqResult = await AnonCreds.ProverCreateCredentialReqAsync(proverWallet, proverDid, credOffer, credDef, masterSecretId);
            var credReq = createCredReqResult.CredentialRequestJson;
            var credReqMetadata = createCredReqResult.CredentialRequestMetadataJson;

            // Issuer create Credential
            var createCredentialResult = await AnonCreds.IssuerCreateCredentialAsync(issuerWallet, credOffer, credReq, gvtCredentialValues, null, null);
            var credential = createCredentialResult.CredentialJson;

            // Prover store Credential
            await AnonCreds.ProverStoreCredentialAsync(proverWallet, credentialId1, credReqMetadata, credential, credDef, null);

            // Prover gets Credentials for Proof Request
            var proofRequestJson = JsonConvert.SerializeObject(
                new
                {
                    nonce = "123432421212",
                    name = "proof_req_1",
                    version = "0.1",
                    requested_attributes = new
                    {
                        attr1_referent = new
                        {
                            name = "name",
                            restrictions = new[] { new { schema_id = gvtSchemaId } }
                        },
                        attr2_referent = new { name = "phone" }
                    },
                    requested_predicates = new {}
                }
            );

            var credentialsForProofJson = await AnonCreds.ProverGetCredentialsForProofReqAsync(proverWallet, proofRequestJson);
            Assert.IsNotNull(credentialsForProofJson);

            var credentialsForProof = JObject.Parse(credentialsForProofJson);
            var credentialsForAttribute1 = (JArray)credentialsForProof["attrs"]["attr1_referent"];

            Assert.AreEqual(1, credentialsForAttribute1.Count);

            var credentialUuid = credentialsForAttribute1[0]["cred_info"]["referent"];

            // Prover create Proof
            var selfAttestedValue = "8-800-300";
            var requestedCredentialsJson = JsonConvert.SerializeObject(
                new
                {
                    self_attested_attributes = new { attr3_referent = selfAttestedValue },
                    requested_attributes = new
                    {
                        attr1_referent = new { cred_id = credentialUuid, revealed = true },
                    },
                    requested_predicates = new { }
                }
            );

            var schemas = new JObject(new JProperty(gvtSchemaId, JObject.Parse(gvtSchema))).ToString();
            var credentialDefs = new JObject(new JProperty(credDefId, JObject.Parse(credDef))).ToString();
            var revocInfos = "{}";

            var proofJson = await AnonCreds.ProverCreateProofAsync(proverWallet, proofRequestJson, requestedCredentialsJson,
                    masterSecretId, schemas, credentialDefs, revocInfos);
            var proof = JObject.Parse(proofJson);

            // Verifier verify Proof
            var revealedAttr1 = proof["requested_proof"]["revealed_attrs"]["attr1_referent"];
            Assert.AreEqual("Alex", revealedAttr1["raw"]);

            Assert.AreEqual(selfAttestedValue, proof["requested_proof"]["self_attested_attrs"]["attr3_referent"]);

            var revocRegDefs = "{}";
            var revocRegs = "{}";

            proofRequestJson = JsonConvert.SerializeObject(
                new
                {
                    nonce = "123432421212",
                    name = "proof_req_1",
                    version = "0.1",
                    requested_attributes = new
                    {
                        attr1_referent = new
                        {
                            name = "name",
                            restrictions = new[] { new { schema_id = gvtSchemaId } }
                        },
                        attr2_referent = new { name = "phone" }
                    },
                    requested_predicates = new
                    {
                        predicate1_referent = new
                        {
                            name = "age",
                            p_type = ">=",
                            p_value = 18
                        }
                    }
                }
            );

            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                AnonCreds.VerifierVerifyProofAsync(proofRequestJson, proofJson, schemas, credentialDefs, revocRegDefs, revocRegs)
            );
        }
    }
}
