using System;
using System.Diagnostics;
using System.Drawing;
using System.Threading.Tasks;
using Hyperledger.Indy.AnonCredsApi;
using Hyperledger.Indy.BlobStorageApi;
using Hyperledger.Indy.PoolApi;
using Hyperledger.Indy.Samples.Utils;
using Hyperledger.Indy.WalletApi;
using Newtonsoft.Json.Linq;
using Console = Colorful.Console;

namespace Hyperledger.Indy.Samples
{
    static class AnonCredsRevocationDemo
    {
        public static async Task Execute()
        {
            Console.Write("Executing anoncreds with revocation sample... ");

            var proverWalletConfig = "{\"id\":\"prover_wallet\"}";
            var issuerWalletConfig = "{\"id\":\"issuer_wallet\"}";

            var issuerWalletCredentials = "{\"key\":\"issuer_wallet_key\"}";
            var proverWalletCredentials = "{\"key\":\"prover_wallet_key\"}";

            var issuerDid = "NcYxiDXkpYi6ov5FcYDi1e";
            var proverDid = "VsKV7grR1BUE29mG2Fm2kX";

            try
            {
                //1. Create and Open Pool
                await PoolUtils.CreatePoolLedgerConfig();

                //2. Issuer Create and Open Wallet
                await WalletUtils.CreateWalletAsync(issuerWalletConfig, issuerWalletCredentials);

                //3. Prover Create and Open Wallet
                await WalletUtils.CreateWalletAsync(proverWalletConfig, proverWalletCredentials);

                // Open pool and wallets in using statements to ensure they are closed when finished.
                using (var issuerWallet = await Wallet.OpenWalletAsync(issuerWalletConfig, issuerWalletCredentials))
                using (var proverWallet = await Wallet.OpenWalletAsync(proverWalletConfig, proverWalletCredentials))
                {
                    //4. Issuer Creates Credential Schema
                    var schemaName = "gvt";
                    var schemaVersion = "1.0";
                    var schemaAttributes = "[\"name\", \"age\", \"sex\", \"height\"]";
                    var createSchemaResult = await AnonCreds.IssuerCreateSchemaAsync(issuerDid, schemaName, schemaVersion, schemaAttributes);
                    var schemaId = createSchemaResult.SchemaId;
                    var schemaJson = createSchemaResult.SchemaJson;

                    //5. Issuer create Credential Definition
                    var credDefTag = "Tag1";
                    var credDefConfigJson = "{\"support_revocation\":true}";
                    var createCredDefResult = await AnonCreds.IssuerCreateAndStoreCredentialDefAsync(issuerWallet, issuerDid, schemaJson, credDefTag, null, credDefConfigJson);
                    var credDefId = createCredDefResult.CredDefId;
                    var credDefJson = createCredDefResult.CredDefJson;

                    //6. Issuer create Revocation Registry
                    var revRegDefConfig = "{\"issuance_type\":\"ISSUANCE_ON_DEMAND\",\"max_cred_num\":5}";
                    var tailsWriterConfig = string.Format("{{\"base_dir\":\"{0}\", \"uri_pattern\":\"\"}}", EnvironmentUtils.GetIndyHomePath("tails")).Replace('\\', '/');
                    var tailsWriter = await BlobStorage.OpenWriterAsync("default", tailsWriterConfig);

                    var revRegDefTag = "Tag2";
                    var createRevRegResult = await AnonCreds.IssuerCreateAndStoreRevocRegAsync(issuerWallet, issuerDid, null, revRegDefTag, credDefId, revRegDefConfig, tailsWriter);
                    var revRegId = createRevRegResult.RevRegId;
                    var revRegDefJson = createRevRegResult.RevRegDefJson;

                    //7. Prover create Master Secret
                    var masterSecretId = await AnonCreds.ProverCreateMasterSecretAsync(proverWallet, null);

                    //8. Issuer Creates Credential Offer
                    var credOffer = await AnonCreds.IssuerCreateCredentialOfferAsync(issuerWallet, credDefId);

                    //9. Prover Creates Credential Request
                    var createCredReqResult = await AnonCreds.ProverCreateCredentialReqAsync(proverWallet, proverDid, credOffer, credDefJson, masterSecretId);
                    var credReqJson = createCredReqResult.CredentialRequestJson;
                    var credReqMetadataJson = createCredReqResult.CredentialRequestMetadataJson;

                    //10. Issuer open Tails Reader
                    var blobStorageReaderCfg = await BlobStorage.OpenReaderAsync("default", tailsWriterConfig);

                    //11. Issuer create Credential
                    var credValuesJson = "{\n" +
                            "        \"sex\": {\"raw\": \"male\", \"encoded\": \"594465709955896723921094925839488742869205008160769251991705001\"},\n" +
                            "        \"name\": {\"raw\": \"Alex\", \"encoded\": \"1139481716457488690172217916278103335\"},\n" +
                            "        \"height\": {\"raw\": \"175\", \"encoded\": \"175\"},\n" +
                            "        \"age\": {\"raw\": \"28\", \"encoded\": \"28\"}\n" +
                            "    }";

                    var createCredentialResult = await AnonCreds.IssuerCreateCredentialAsync(issuerWallet, credOffer, credReqJson, credValuesJson, revRegId, blobStorageReaderCfg);
                    var credential = createCredentialResult.CredentialJson;
                    var revRegDeltaJson = createCredentialResult.RevocRegDeltaJson;
                    var credRevId = createCredentialResult.RevocId;

                    //12. Prover Stores Credential
                    await AnonCreds.ProverStoreCredentialAsync(proverWallet, null, credReqMetadataJson, credential, credDefJson, revRegDefJson);

                    //13. Prover Gets Credentials for Proof Request
                    var proofRequestJson = "{\n" +
                            "                   \"nonce\":\"123432421212\",\n" +
                            "                   \"name\":\"proof_req_1\",\n" +
                            "                   \"version\":\"0.1\", " +
                            "                   \"requested_attributes\":{" +
                            "                          \"attr1_referent\":{\"name\":\"name\"}" +
                            "                    },\n" +
                            "                    \"requested_predicates\":{" +
                            "                          \"predicate1_referent\":{\"name\":\"age\",\"p_type\":\">=\",\"p_value\":18}" +
                            "                    }" +
                            "               }";

                    var credentialsForProofJson = await AnonCreds.ProverGetCredentialsForProofReqAsync(proverWallet, proofRequestJson);

                    var credentials = JObject.Parse(credentialsForProofJson);
                    var credentialsForAttr1 = (JArray)credentials["attrs"]["attr1_referent"];
                    var credentialsForPredicate1 = (JArray)credentials["predicates"]["predicate1_referent"];

                    var credIdForAttr1 = credentialsForAttr1[0]["cred_info"]["referent"].ToObject<string>();
                    var credIdForPred1 = credentialsForPredicate1[0]["cred_info"]["referent"].ToObject<string>();

                    //14. Prover create RevocationState
                    long timestamp = 100;
                    var revStateJson = await AnonCreds.CreateRevocationStateAsync(blobStorageReaderCfg, revRegDefJson, revRegDeltaJson, timestamp, credRevId);

                    //15. Prover Creates Proof
                    var requestedCredentialsJson = string.Format("{{" +
                                                                    "\"self_attested_attributes\":{{}}," +
                                                                    "\"requested_attributes\":{{\"attr1_referent\":{{\"cred_id\":\"{0}\", \"revealed\":true, \"timestamp\":{1} }}}}," +
                                                                    "\"requested_predicates\":{{\"predicate1_referent\":{{\"cred_id\":\"{2}\", \"timestamp\":{3}}}}}" +
                                                                    "}}", credIdForAttr1, timestamp, credIdForPred1, timestamp);

                    var schemas = string.Format("{{\"{0}\":{1}}}", schemaId, schemaJson);
                    var credentialDefs = string.Format("{{\"{0}\":{1}}}", credDefId, credDefJson);
                    var revStates = string.Format("{{\"{0}\": {{ \"{1}\":{2} }}}}", revRegId, timestamp, revStateJson);

                    var proofJson = await AnonCreds.ProverCreateProofAsync(proverWallet, proofRequestJson, requestedCredentialsJson, masterSecretId, schemas, credentialDefs, revStates);
                    var proof = JObject.Parse(proofJson);

                    //16. Verifier verify Proof
                    var revealedAttr1 = proof["requested_proof"]["revealed_attrs"]["attr1_referent"];
                    Debug.Assert("Alex" == revealedAttr1["raw"].ToObject<string>());

                    var revRegDefs = string.Format("{{\"{0}\":{1}}}", revRegId, revRegDefJson);
                    var revRegs = string.Format("{{\"{0}\": {{ \"{1}\":{2} }}}}", revRegId, timestamp, revRegDeltaJson);

                    var valid = await AnonCreds.VerifierVerifyProofAsync(proofRequestJson, proofJson, schemas, credentialDefs, revRegDefs, revRegs);
                    Debug.Assert(valid);

                    await issuerWallet.CloseAsync();
                    await proverWallet.CloseAsync();
                }

                Console.WriteLine("OK", Color.Green);
            }
            catch (Exception e)
            {
                Console.WriteLine($"Error: {e.Message}", Color.Red);
            }
            finally
            {
                //17. Delete wallets and Pool ledger config
                await WalletUtils.DeleteWalletAsync(issuerWalletConfig, issuerWalletCredentials);
                await WalletUtils.DeleteWalletAsync(proverWalletConfig, proverWalletCredentials);
                await PoolUtils.DeletePoolLedgerConfigAsync(PoolUtils.DEFAULT_POOL_NAME);
            }
        }
    }
}