using Hyperledger.Indy.AnonCredsApi;
using Hyperledger.Indy.BlobStorageApi;
using Hyperledger.Indy.DidApi;
using Hyperledger.Indy.LedgerApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;
using System;
using System.Threading;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.InteractionTests
{
    [TestClass]
    public class AnoncredsRevocationInteractionTest : IndyIntegrationTestWithPoolAndSingleWallet
    {
        private const string COMMON_MASTER_SECRET = "common_master_secret_name";
        private Wallet proverWallet;
        private string proverWalletConfig = JsonConvert.SerializeObject(new { id = "proverWallet" });

        [TestInitialize]
        public async Task CreateProverWallet()
        {
            await Wallet.CreateWalletAsync(proverWalletConfig, WALLET_CREDENTIALS);
            proverWallet = await Wallet.OpenWalletAsync(proverWalletConfig, WALLET_CREDENTIALS);
        }

        [TestCleanup]
        public async Task DeleteProverWallet()
        {
            await proverWallet.CloseAsync();
            await Wallet.DeleteWalletAsync(proverWalletConfig, WALLET_CREDENTIALS);
        }

        [TestMethod]
        public async Task TestAnoncredsRevocationInteractionIssuanceByDemand()
        {
            // Issuer create DID
            var trusteeDidInfo = await Did.CreateAndStoreMyDidAsync(this.wallet, JsonConvert.SerializeObject(new { seed = TRUSTEE_SEED }));
            var issuerDidInfo = await Did.CreateAndStoreMyDidAsync(this.wallet, "{}");
            var nymRequest = await Ledger.BuildNymRequestAsync(trusteeDidInfo.Did, issuerDidInfo.Did, issuerDidInfo.VerKey, null, "TRUSTEE");
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, trusteeDidInfo.Did, nymRequest);

            var issuerDid = issuerDidInfo.Did;

            // Prover create DID
            var proverDidInfo = await Did.CreateAndStoreMyDidAsync(proverWallet, "{}");

            var proverDid = proverDidInfo.Did;
            var proverVerkey = proverDidInfo.VerKey;

            // Issuer publish Prover DID
            nymRequest = await Ledger.BuildNymRequestAsync(issuerDid, proverDid, proverVerkey, null, null);
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, issuerDid, nymRequest);

            // ISSUER post to Ledger Schema, CredentialDefinition, RevocationRegistry

            // Issuer creates Schema
            var schemaInfo = await AnonCreds.IssuerCreateSchemaAsync(issuerDidInfo.Did, GVT_SCHEMA_NAME, SCHEMA_VERSION, GVT_SCHEMA_ATTRIBUTES);
            var schemaJson = schemaInfo.SchemaJson;

            // Issuer posts Schema to Ledger
            var schemaRequest = await Ledger.BuildSchemaRequestAsync(issuerDid, schemaJson);
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, issuerDid, schemaRequest);

            // Issuer get Schema from Ledger
            var getSchemaRequest = await Ledger.BuildGetSchemaRequestAsync(issuerDid, schemaInfo.SchemaId);
            var getSchemaResponse = await PoolUtils.EnsurePreviousRequestAppliedAsync(pool, getSchemaRequest, response => {
                var getSchemaResponseObject = JObject.Parse(response);
                return getSchemaResponseObject["result"]["seqNo"] != null;
            });

            // !!IMPORTANT!!
            // It is important to get Schema from Ledger and parse it to get the correct schema JSON and correspondent id in Ledger
            // After that we can create CredentialDefinition for received Schema(not for result of indy_issuer_create_schema)

            ParseResponseResult schemaInfo1 = await Ledger.ParseGetSchemaResponseAsync(getSchemaResponse);
            schemaJson = schemaInfo1.ObjectJson;

            // Issuer creates CredentialDefinition
            var credDefInfo = await AnonCreds.IssuerCreateAndStoreCredentialDefAsync(wallet, issuerDid, schemaJson, TAG, null, JsonConvert.SerializeObject(new { support_revocation = true }));

            var credDefId = credDefInfo.CredDefId;
            var credDefJson = credDefInfo.CredDefJson;


            // Issuer post CredentialDefinition to Ledger
            var credDefRequest = await Ledger.BuildCredDefRequestAsync(issuerDid, credDefJson);
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, issuerDid, credDefRequest);

            // Issuer creates RevocationRegistry
            /* FIXME: getIndyHomePath hard coded forward slash "/". It will not work for Windows. */
            var tailsWriterConfig = JsonConvert.SerializeObject(
                new
                {
                    base_dir = EnvironmentUtils.GetIndyHomePath("tails"),
                    uri_pattern = string.Empty
                }
            );

            var tailsWriterHandle = await BlobStorage.OpenWriterAsync("default", tailsWriterConfig);

            var revRegInfo =
                    await AnonCreds.IssuerCreateAndStoreRevocRegAsync(wallet, issuerDid, null, TAG,
                            credDefId,
                            JsonConvert.SerializeObject(new { max_cred_num = 5, issuance_type = "ISSUANCE_ON_DEMAND" }),
                            tailsWriterHandle);

            var revRegId = revRegInfo.RevRegId;
            var revRegDefJson = revRegInfo.RevRegDefJson;
            var revRegEntryJson = revRegInfo.RevRegEntryJson;

            // Issuer posts RevocationRegistryDefinition to Ledger
            var revRegDefRequest = await Ledger.BuildRevocRegDefRequestAsync(issuerDid, revRegDefJson);
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, issuerDid, revRegDefRequest);

            // Issuer posts RevocationRegistryEntry to Ledger
            var revRegEntryRequest = await Ledger.BuildRevocRegEntryRequestAsync(issuerDid, revRegId,
                    REVOC_REG_TYPE, revRegEntryJson);
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, issuerDid, revRegEntryRequest);

            // Issuance Credential for Prover

            // Prover creates Master Secret
            await AnonCreds.ProverCreateMasterSecretAsync(proverWallet, COMMON_MASTER_SECRET);

            // Issuer creates Credential Offer
            var credOfferJson = await AnonCreds.IssuerCreateCredentialOfferAsync(wallet, credDefId);

            // Prover gets CredentialDefinition from Ledger
            var credOffer = JObject.Parse(credOfferJson);
            var getCredDefRequest = await Ledger.BuildGetCredDefRequestAsync(proverDid, (string)credOffer["cred_def_id"]);

            var getCredDefResponse = await Ledger.SubmitRequestAsync(pool, getCredDefRequest);
            ParseResponseResult credDefIdInfo = await Ledger.ParseGetCredDefResponseAsync(getCredDefResponse);

            credDefId = credDefIdInfo.Id;
            credDefJson = credDefIdInfo.ObjectJson;

            // Prover creates Credential Request
            var credReqInfo =
                    await AnonCreds.ProverCreateCredentialReqAsync(proverWallet, proverDid, credOfferJson,
                            credDefJson, COMMON_MASTER_SECRET);

            var credReqJson = credReqInfo.CredentialRequestJson;
            var credReqMetadataJson = credReqInfo.CredentialRequestMetadataJson;

            // Issuer creates TailsReader
            var blobStorageReader = await BlobStorage.OpenReaderAsync(TYPE, tailsWriterConfig);

            // Issuer creates Credential
            var credRegInfo =
                    await AnonCreds.IssuerCreateCredentialAsync(wallet, credOfferJson, credReqJson,
                            GVT_CRED_VALUES, revRegId,
                            blobStorageReader);

            var credJson = credRegInfo.CredentialJson;
            var credRevId = credRegInfo.RevocId;
            var revocRegDeltaJson = credRegInfo.RevocRegDeltaJson;

            // Issuer posts RevocationRegistryDelta to Ledger
            revRegEntryRequest = await Ledger.BuildRevocRegEntryRequestAsync(issuerDid, revRegId, REVOC_REG_TYPE, revocRegDeltaJson);
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, issuerDid, revRegEntryRequest);

            // Prover gets RevocationRegistryDefinition
            var credential = JObject.Parse(credJson);
            var getRevRegDefRequest = await Ledger.BuildGetRevocRegDefRequestAsync(proverDid, (string)credential["rev_reg_id"]);
            var getRevRegDefResponse = await Ledger.SubmitRequestAsync(pool, getRevRegDefRequest);

            ParseResponseResult revRegInfo1 = await Ledger.ParseGetRevocRegDefResponseAsync(getRevRegDefResponse);
            var revocRegDefJson = revRegInfo1.ObjectJson;

            // Prover store received Credential
            await AnonCreds.ProverStoreCredentialAsync(proverWallet, "credential1_id",
                    credReqMetadataJson, credJson, credDefJson, revocRegDefJson);

            // Verifying Prover Credential
            Thread.Sleep(3000);

            long to = (long)(DateTime.UtcNow - new DateTime(1970, 1, 1, 0, 0, 0, DateTimeKind.Utc)).TotalMilliseconds / 1000;
            var proofRequest = JsonConvert.SerializeObject(
                new
                {
                    nonce = "123432421212",
                    name = "proof_req_1",
                    version = "0.1",
                    requested_attributes = new
                    {
                        attr1_referent = new
                        {
                            name = "name"
                        }
                    },
                    requested_predicates = new
                    {
                        predicate1_referent = new
                        {
                            name = "age",
                            p_type = ">=",
                            p_value = 18
                        }
                    },
                    non_revoked = new
                    {
                        to = to
                    }
                }
            );

            // Prover gets Claims for Proof Request
            var credsJson = await AnonCreds.ProverGetCredentialsForProofReqAsync(proverWallet, proofRequest);

            var credentials = JObject.Parse(credsJson);
            var credsForReferent = (JArray)credentials["attrs"]["attr1_referent"];
            var cred_info = credsForReferent[0]["cred_info"];

            // Prover gets RevocationRegistryDelta from Ledger

            var getRevRegDeltaRequest = await Ledger.BuildGetRevocRegDeltaRequestAsync(proverDid, (string)cred_info["rev_reg_id"], -1, (int)to);
            var getRevRegDeltaResponse = await Ledger.SubmitRequestAsync(pool, getRevRegDeltaRequest);

            var revRegInfo2 = await Ledger.ParseGetRevocRegDeltaResponseAsync(getRevRegDeltaResponse);

            revRegId = revRegInfo2.Id;
            revocRegDeltaJson = revRegInfo2.ObjectJson;
            var timestamp = revRegInfo2.Timestamp;

            // Prover creates RevocationState
            var revStateJson = await AnonCreds.CreateRevocationStateAsync(blobStorageReader,
                    revocRegDefJson, revocRegDeltaJson, (long)timestamp, credRevId);

            // Prover gets Schema from Ledger
            getSchemaRequest = await Ledger.BuildGetSchemaRequestAsync(proverDid, (string)cred_info["schema_id"]);
            getSchemaResponse = await Ledger.SubmitRequestAsync(pool, getSchemaRequest);

            ParseResponseResult schemaInfo2 = await Ledger.ParseGetSchemaResponseAsync(getSchemaResponse);
            var schemaId = schemaInfo2.Id;
            schemaJson = schemaInfo2.ObjectJson;

            // Prover creates Proof
            var requestedCredentialsJson = JsonConvert.SerializeObject(
                new
                {
                    self_attested_attributes = new { },
                    requested_attributes = new
                    {
                        attr1_referent = new
                        {
                            cred_id = cred_info["referent"],
                            timestamp = timestamp,
                            revealed = true
                        }
                    },
                    requested_predicates = new
                    {
                        predicate1_referent = new
                        {
                            cred_id = cred_info["referent"],
                            timestamp = timestamp
                        }
                    }
                }
            );

            var schemasJson = new JObject(new JProperty(schemaId, JObject.Parse(schemaJson))).ToString();
            var credDefsJson = new JObject(new JProperty(credDefId, JObject.Parse(credDefJson))).ToString(); 
            var revStatesJson = new JObject(new JProperty(revRegId, new JObject(new JProperty(timestamp.ToString(), JObject.Parse(revStateJson))))).ToString(); 

            var proofJson = await AnonCreds.ProverCreateProofAsync(proverWallet, proofRequest,
                    requestedCredentialsJson, COMMON_MASTER_SECRET,
                    schemasJson, credDefsJson, revStatesJson);

            var proof = JObject.Parse(proofJson);
            var identifier = proof["identifiers"][0];

            // Verifier gets Schema from Ledger
            var getSchemaReq = await Ledger.BuildGetSchemaRequestAsync(DID_MY1, (string)identifier["schema_id"]);
            var getSchemaResp = await Ledger.SubmitRequestAsync(pool, getSchemaReq);
            var schemaInfo3 = await Ledger.ParseGetSchemaResponseAsync(getSchemaResp);
            schemaId = schemaInfo3.Id;
            schemaJson = schemaInfo3.ObjectJson;

            // Verifier gets CredDef from Ledger
            var getCredDefReq = await Ledger.BuildGetCredDefRequestAsync(DID_MY1, (string)identifier["cred_def_id"]);
            var getCredDefResp = await Ledger.SubmitRequestAsync(pool, getCredDefReq);
            var credDefInfo3 = await Ledger.ParseGetCredDefResponseAsync(getCredDefResp);
            credDefId = credDefInfo3.Id;
            credDefJson = credDefInfo3.ObjectJson;

            // Verifier gets RevocationRegistryDefinition from Ledger
            var getRevRegDefReq = await Ledger.BuildGetRevocRegDefRequestAsync(DID_MY1, (string)identifier["rev_reg_id"]);
            var getRevRegDefResp = await Ledger.SubmitRequestAsync(pool, getRevRegDefReq);
            ParseResponseResult revRegDefInfo3 = await Ledger.ParseGetRevocRegDefResponseAsync(getRevRegDefResp);
            var revRegDefId = revRegDefInfo3.Id;
            revRegDefJson = revRegDefInfo3.ObjectJson;

            // Verifier gets RevocationRegistry from Ledger
            var getRevRegReq = await Ledger.BuildGetRevocRegRequestAsync(DID_MY1, (string)identifier["rev_reg_id"], (int)identifier["timestamp"]);
            var getRevRegResp = await Ledger.SubmitRequestAsync(pool, getRevRegReq);
            var revRegInfo3 = await Ledger.ParseGetRevocRegResponseAsync(getRevRegResp);
            revRegId = revRegInfo3.Id;
            var revRegJson = revRegInfo3.ObjectJson;
            timestamp = revRegInfo3.Timestamp;

            // Verifier verifies proof
            Assert.AreNotEqual("Alex", proof["requested_proof"]["revealed_attrs"]["attr1_referent"].ToString());

            schemasJson = new JObject(new JProperty(schemaId, JObject.Parse(schemaJson))).ToString();
            credDefsJson = new JObject(new JProperty(credDefId, JObject.Parse(credDefJson))).ToString();
            var revRegDefsJson = new JObject(new JProperty(revRegId, JObject.Parse(revRegDefJson))).ToString();
            var revRegsJson = new JObject(new JProperty(revRegId, new JObject(new JProperty(timestamp.ToString(), JObject.Parse(revRegJson))))).ToString();

            var valid = await AnonCreds.VerifierVerifyProofAsync(proofRequest,
                    proofJson,
                    schemasJson,
                    credDefsJson,
                    revRegDefsJson,
                    revRegsJson);

            Assert.IsTrue(valid);

            // Issuer revokes credential
            var revRegDeltaJson = await AnonCreds.IssuerRevokeCredentialAsync(wallet,
                    blobStorageReader,
                    revRegId, credRevId);

            // Issuer post RevocationRegistryDelta to Ledger
            revRegEntryRequest = await Ledger.BuildRevocRegEntryRequestAsync(issuerDid, revRegId, REVOC_REG_TYPE, revRegDeltaJson);

            await Ledger.SignAndSubmitRequestAsync(pool, wallet, issuerDid, revRegEntryRequest);

            // Verifying Prover Credential after Revocation
            Thread.Sleep(3000);

            long from = to;
            to = (long)(DateTime.UtcNow - new DateTime(1970, 1, 1, 0, 0, 0, DateTimeKind.Utc)).TotalMilliseconds / 1000; 

            // Prover gets RevocationRegistryDelta from Ledger
            getRevRegDeltaRequest = await Ledger.BuildGetRevocRegDeltaRequestAsync(proverDid, revRegId, (int)from, (int)to);
            getRevRegDeltaResponse = await Ledger.SubmitRequestAsync(pool, getRevRegDeltaRequest);
            var revRegInfo4 = await Ledger.ParseGetRevocRegDeltaResponseAsync(getRevRegDeltaResponse);

            revRegId = revRegInfo4.Id;
            revocRegDeltaJson = revRegInfo4.ObjectJson;
            timestamp = revRegInfo4.Timestamp;

            // Prover creates RevocationState
            revStateJson = await AnonCreds.CreateRevocationStateAsync(blobStorageReader,
                    revocRegDefJson, revocRegDeltaJson, (long)timestamp, credRevId);

            requestedCredentialsJson = JsonConvert.SerializeObject(
                new
                {
                    self_attested_attributes = new { },
                    requested_attributes = new
                    {
                        attr1_referent = new
                        {
                            cred_id = cred_info["referent"],
                            timestamp = timestamp,
                            revealed = true
                        }
                    },
                    requested_predicates = new
                    {
                        predicate1_referent = new
                        {
                            cred_id = cred_info["referent"],
                            timestamp = timestamp
                        }
                    }
                }
            );

            revStatesJson = new JObject(new JProperty(revRegId, new JObject(new JProperty(timestamp.ToString(), JObject.Parse(revStateJson))))).ToString();

            proofJson = await AnonCreds.ProverCreateProofAsync(proverWallet,
                    proofRequest,
                    requestedCredentialsJson,
                    COMMON_MASTER_SECRET,
                    schemasJson,
                    credDefsJson,
                    revStatesJson);

            proof = JObject.Parse(proofJson);
            identifier = proof["identifiers"][0];

            // Verifier gets RevocationRegistry from Ledger
            getRevRegReq = await Ledger.BuildGetRevocRegRequestAsync(DID_MY1, (string)identifier["rev_reg_id"], (int)identifier["timestamp"]);
            getRevRegResp = await Ledger.SubmitRequestAsync(pool, getRevRegReq);

            var revRegInfo5 = await Ledger.ParseGetRevocRegResponseAsync(getRevRegResp);
            revRegId = revRegInfo5.Id;
            revRegJson = revRegInfo5.ObjectJson;
            timestamp = revRegInfo5.Timestamp;

            revRegsJson = new JObject(new JProperty(revRegId, new JObject(new JProperty(timestamp.ToString(), JObject.Parse(revRegJson))))).ToString();

            valid = await AnonCreds.VerifierVerifyProofAsync(proofRequest,
                    proofJson,
                    schemasJson,
                    credDefsJson,
                    revRegDefsJson,
                    revRegsJson);

            Assert.IsFalse(valid);
        }

        [TestMethod]
        public async Task testAnoncredsRevocationInteractionIssuanceByDefault()
        {
            // Issuer create DID
            var trusteeDidInfo = await Did.CreateAndStoreMyDidAsync(this.wallet, JsonConvert.SerializeObject(new { seed = TRUSTEE_SEED }));

            var issuerDidInfo = await Did.CreateAndStoreMyDidAsync(this.wallet, "{}");
            var nymRequest = await Ledger.BuildNymRequestAsync(trusteeDidInfo.Did, issuerDidInfo.Did,
                    issuerDidInfo.VerKey, null, "TRUSTEE");
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, trusteeDidInfo.Did, nymRequest);

            var issuerDid = issuerDidInfo.Did;

            // Prover create DID
            var proverDidInfo = await Did.CreateAndStoreMyDidAsync(proverWallet, "{}");

            var proverDid = proverDidInfo.Did;
            var proverVerkey = proverDidInfo.VerKey;

            // Issuer publish Prover DID
            nymRequest = await Ledger.BuildNymRequestAsync(issuerDid, proverDid, proverVerkey, null, null);
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, issuerDid, nymRequest);

            // ISSUER post to Ledger Schema, CredentialDefinition, RevocationRegistry

            // Issuer creates Schema
            var schemaInfo =
                    await AnonCreds.IssuerCreateSchemaAsync(issuerDidInfo.Did, GVT_SCHEMA_NAME,
                            SCHEMA_VERSION, GVT_SCHEMA_ATTRIBUTES);

            var schemaJson = schemaInfo.SchemaJson;

            // Issuer posts Schema to Ledger
            var schemaRequest = await Ledger.BuildSchemaRequestAsync(issuerDid, schemaJson);
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, issuerDid, schemaRequest);

            // Issuer get Schema from Ledger
            var getSchemaRequest = await Ledger.BuildGetSchemaRequestAsync(issuerDid, schemaInfo.SchemaId);
            var getSchemaResponse = await PoolUtils.EnsurePreviousRequestAppliedAsync(pool, getSchemaRequest, response => {
                var getSchemaResponseObject = JObject.Parse(response);
                return getSchemaResponseObject["result"]["seqNo"] != null;
            });

            // !!IMPORTANT!!
            // It is important to get Schema from Ledger and parse it to get the correct schema JSON and correspondent id in Ledger
            // After that we can create CredentialDefinition for received Schema(not for result of indy_issuer_create_schema)

            ParseResponseResult schemaInfo1 = await Ledger.ParseGetSchemaResponseAsync(getSchemaResponse);
            schemaJson = schemaInfo1.ObjectJson;

            // Issuer creates CredentialDefinition

            var credDefInfo =
                    await AnonCreds.IssuerCreateAndStoreCredentialDefAsync(wallet, issuerDid, schemaJson,
                            TAG, null, JsonConvert.SerializeObject(new { support_revocation = true }));

            var credDefId = credDefInfo.CredDefId;
            var credDefJson = credDefInfo.CredDefJson;

            // Issuer post CredentialDefinition to Ledger
            var credDefRequest = await Ledger.BuildCredDefRequestAsync(issuerDid, credDefJson);
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, issuerDid, credDefRequest);

            // Issuer creates RevocationRegistry
            /* FIXME: getIndyHomePath hard coded forward slash "/". It will not work for Windows. */
            var tailsWriterConfig = JsonConvert.SerializeObject(
                new
                {
                    base_dir = EnvironmentUtils.GetIndyHomePath("tails"),
                    uri_pattern = string.Empty
                }
            );

            var tailsWriterHandle = await BlobStorage.OpenWriterAsync("default", tailsWriterConfig);

            var revRegInfo =
                    await AnonCreds.IssuerCreateAndStoreRevocRegAsync(wallet, issuerDid, null, TAG,
                            credDefId,
                            JsonConvert.SerializeObject(new { max_cred_num = 5, issuance_type = "ISSUANCE_BY_DEFAULT" }),
                            tailsWriterHandle);

            var revRegId = revRegInfo.RevRegId;
            var revRegDefJson = revRegInfo.RevRegDefJson;
            var revRegEntryJson = revRegInfo.RevRegEntryJson;

            // Issuer posts RevocationRegistryDefinition to Ledger
            var revRegDefRequest = await Ledger.BuildRevocRegDefRequestAsync(issuerDid, revRegDefJson);
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, issuerDid, revRegDefRequest);

            // Issuer posts RevocationRegistryEntry to Ledger
            var revRegEntryRequest = await Ledger.BuildRevocRegEntryRequestAsync(issuerDid, revRegId, REVOC_REG_TYPE, revRegEntryJson);
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, issuerDid, revRegEntryRequest);

            // Issuance Credential for Prover

            // Prover creates Master Secret
            await AnonCreds.ProverCreateMasterSecretAsync(proverWallet, COMMON_MASTER_SECRET);

            // Issuer creates Credential Offer
            var credOfferJson = await AnonCreds.IssuerCreateCredentialOfferAsync(wallet, credDefId);

            // Prover gets CredentialDefinition from Ledger
            var getCredDefRequest = await Ledger.BuildGetCredDefRequestAsync(proverDid, credDefInfo.CredDefId);

            var getCredDefResponse = await Ledger.SubmitRequestAsync(pool, getCredDefRequest);
            ParseResponseResult credDefIdInfo = await Ledger.ParseGetCredDefResponseAsync(getCredDefResponse);

            credDefId = credDefIdInfo.Id;
            credDefJson = credDefIdInfo.ObjectJson;

            // Prover creates Credential Request
            var credReqInfo =
                    await AnonCreds.ProverCreateCredentialReqAsync(proverWallet, proverDid, credOfferJson,
                            credDefJson, COMMON_MASTER_SECRET);

            var credReqJson = credReqInfo.CredentialRequestJson;
            var credReqMetadataJson = credReqInfo.CredentialRequestMetadataJson;

            // Issuer creates TailsReader
            BlobStorageReader blobStorageReader = await BlobStorage.OpenReaderAsync(TYPE, tailsWriterConfig);

            // Issuer creates Credential
            // Issuer must not post rev_reg_delta to ledger for ISSUANCE_BY_DEFAULT strategy

            var credRegInfo =
                    await AnonCreds.IssuerCreateCredentialAsync(wallet, credOfferJson, credReqJson,
                            GVT_CRED_VALUES, revRegId,
                            blobStorageReader);

            var credJson = credRegInfo.CredentialJson;
            var credRevId = credRegInfo.RevocId;

            // Prover gets RevocationRegistryDefinition
            var getRevRegDefRequest = await Ledger.BuildGetRevocRegDefRequestAsync(proverDid, revRegId);
            var getRevRegDefResponse = await Ledger.SubmitRequestAsync(pool, getRevRegDefRequest);

            ParseResponseResult revRegInfo1 = await Ledger.ParseGetRevocRegDefResponseAsync(getRevRegDefResponse);

            revRegId = revRegInfo1.Id;
            var revocRegDefJson = revRegInfo1.ObjectJson;

            // Prover store received Credential

            await AnonCreds.ProverStoreCredentialAsync(proverWallet, "credential1_id",
                    credReqMetadataJson, credJson, credDefJson,
                    revocRegDefJson);

            // Verifying Prover Credential
            Thread.Sleep(3000);

            long to = (long)(DateTime.UtcNow - new DateTime(1970, 1, 1, 0, 0, 0, DateTimeKind.Utc)).TotalMilliseconds / 1000;

            var proofRequest = JsonConvert.SerializeObject(
                new
                {
                    nonce = "123432421212",
                    name = "proof_req_1",
                    version = "0.1",
                    requested_attributes = new
                    {
                        attr1_referent = new
                        {
                            name = "name"
                        }
                    },
                    requested_predicates = new
                    {
                        predicate1_referent = new
                        {
                            name = "age",
                            p_type = ">=",
                            p_value = 18
                        }
                    },
                    non_revoked = new
                    {
                        to = to
                    }
                }
            );

            // Prover gets Claims for Proof Request

            var credentialsJson = await AnonCreds.ProverGetCredentialsForProofReqAsync(proverWallet, proofRequest);

            var credentials = JObject.Parse(credentialsJson);
            var credsForReferent = (JArray)credentials["attrs"]["attr1_referent"];
            var credential = credsForReferent[0]["cred_info"];

            // Prover gets RevocationRegistryDelta from Ledger

            /* FIXME */
            var getRevRegDeltaRequest = await Ledger.BuildGetRevocRegDeltaRequestAsync(proverDid, revRegId, -1, (int)to);
            var getRevRegDeltaResponse = await Ledger.SubmitRequestAsync(pool, getRevRegDeltaRequest);

            var revRegInfo2 = await Ledger.ParseGetRevocRegDeltaResponseAsync(getRevRegDeltaResponse);

            revRegId = revRegInfo2.Id;
            var revocRegDeltaJson = revRegInfo2.ObjectJson;

            // Prover creates RevocationState
            var timestamp = to;

            var revStateJson = await AnonCreds.CreateRevocationStateAsync(blobStorageReader,
                    revocRegDefJson, revocRegDeltaJson, (int)timestamp, credRevId);

            // Prover gets Schema from Ledger
            getSchemaRequest = await Ledger.BuildGetSchemaRequestAsync(proverDid, schemaInfo1.Id);
            getSchemaResponse = await Ledger.SubmitRequestAsync(pool, getSchemaRequest);

            ParseResponseResult schemaInfo2 = await Ledger.ParseGetSchemaResponseAsync(getSchemaResponse);
            var schemaId = schemaInfo2.Id;
            schemaJson = schemaInfo2.ObjectJson;

            // Prover creates Proof
            var requestedCredentialsJson = JsonConvert.SerializeObject(
                new
                {
                    self_attested_attributes = new { },
                    requested_attributes = new
                    {
                        attr1_referent = new
                        {
                            cred_id = credential["referent"],
                            timestamp = timestamp,
                            revealed = true
                        }
                    },
                    requested_predicates = new
                    {
                        predicate1_referent = new
                        {
                            cred_id = credential["referent"],
                            timestamp = timestamp
                        }
                    }
                }
            );


            var schemasJson = new JObject(new JProperty(schemaId, JObject.Parse(schemaJson))).ToString();
            var credDefsJson = new JObject(new JProperty(credDefId, JObject.Parse(credDefJson))).ToString();
            var revStatesJson = new JObject(new JProperty(revRegId, new JObject(new JProperty(timestamp.ToString(), JObject.Parse(revStateJson))))).ToString();

            var proofJson = await AnonCreds.ProverCreateProofAsync(proverWallet, proofRequest,
                    requestedCredentialsJson, COMMON_MASTER_SECRET,
                    schemasJson, credDefsJson, revStatesJson);


            var proof = JObject.Parse(proofJson);

            // Verifier gets RevocationRegistry from Ledger

            var getRevRegReq = await Ledger.BuildGetRevocRegRequestAsync(DID_MY1, revRegId, (int)timestamp);
            var getRevRegResp = await Ledger.SubmitRequestAsync(pool, getRevRegReq);
            var revRegInfo3 = await Ledger.ParseGetRevocRegResponseAsync(getRevRegResp);

            revRegId = revRegInfo3.Id;
            var revRegJson = revRegInfo3.ObjectJson;

            // Verifier verifies proof
            Assert.AreNotEqual("Alex", proof["requested_proof"]["revealed_attrs"]["attr1_referent"].ToString());

            var revRegDefsJson = new JObject(new JProperty(revRegId, JObject.Parse(revRegDefJson))).ToString();
            var revRegsJson = new JObject(new JProperty(revRegId, new JObject(new JProperty(timestamp.ToString(), JObject.Parse(revRegJson))))).ToString();

            var valid = await AnonCreds.VerifierVerifyProofAsync(proofRequest,
                    proofJson,
                    schemasJson,
                    credDefsJson,
                    revRegDefsJson,
                    revRegsJson);
            Assert.IsTrue(valid);

            // Issuer revokes credential
            var revRegDeltaJson = await AnonCreds.IssuerRevokeCredentialAsync(wallet,
                    blobStorageReader,
                    revRegId, credRevId);

            // Issuer post RevocationRegistryDelta to Ledger
            revRegEntryRequest = await Ledger.BuildRevocRegEntryRequestAsync(issuerDid, revRegId, REVOC_REG_TYPE, revRegDeltaJson);
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, issuerDid, revRegEntryRequest);

            // Verifying Prover Credential after Revocation
            Thread.Sleep(3000);

            long from = to;
            to = (long)(DateTime.UtcNow - new DateTime(1970, 1, 1, 0, 0, 0, DateTimeKind.Utc)).TotalMilliseconds / 1000;

            // Prover gets RevocationRegistryDelta from Ledger
            getRevRegDeltaRequest = await Ledger.BuildGetRevocRegDeltaRequestAsync(proverDid, revRegId, (int)from, (int)to);
            getRevRegDeltaResponse = await Ledger.SubmitRequestAsync(pool, getRevRegDeltaRequest);
            var revRegInfo4 = await Ledger.ParseGetRevocRegDeltaResponseAsync(getRevRegDeltaResponse);

            revRegId = revRegInfo4.Id;
            revocRegDeltaJson = revRegInfo4.ObjectJson;
            timestamp = (long)revRegInfo4.Timestamp;

            // Prover creates RevocationState
            revStateJson = await AnonCreds.CreateRevocationStateAsync(blobStorageReader,
                    revocRegDefJson, revocRegDeltaJson, (int)timestamp, credRevId);

            requestedCredentialsJson = JsonConvert.SerializeObject(
                new
                {
                    self_attested_attributes = new { },
                    requested_attributes = new
                    {
                        attr1_referent = new
                        {
                            cred_id = credential["referent"],
                            timestamp = timestamp,
                            revealed = true
                        }
                    },
                    requested_predicates = new
                    {
                        predicate1_referent = new
                        {
                            cred_id = credential["referent"],
                            timestamp = timestamp
                        }
                    }
                }
            );

            revStatesJson = new JObject(new JProperty(revRegId, new JObject(new JProperty(timestamp.ToString(), JObject.Parse(revStateJson))))).ToString();

            proofJson = await AnonCreds.ProverCreateProofAsync(proverWallet,
                    proofRequest,
                    requestedCredentialsJson,
                    COMMON_MASTER_SECRET,
                    schemasJson,
                    credDefsJson,
                    revStatesJson);

            // Verifier gets RevocationRegistry from Ledger
            getRevRegReq = await Ledger.BuildGetRevocRegRequestAsync(DID_MY1, revRegId, (int)timestamp);
            getRevRegResp = await Ledger.SubmitRequestAsync(pool, getRevRegReq);

            var revRegInfo5 = await Ledger.ParseGetRevocRegResponseAsync(getRevRegResp);
            revRegId = revRegInfo5.Id;
            revRegJson = revRegInfo5.ObjectJson;
            timestamp = (long)revRegInfo5.Timestamp;

            revRegsJson = new JObject(new JProperty(revRegId, new JObject(new JProperty(timestamp.ToString(), JObject.Parse(revRegJson))))).ToString();
            
            valid = await AnonCreds.VerifierVerifyProofAsync(proofRequest,
                    proofJson,
                    schemasJson,
                    credDefsJson,
                    revRegDefsJson,
                    revRegsJson);
            Assert.IsFalse(valid);
        }

    }
}
