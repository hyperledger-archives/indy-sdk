using Hyperledger.Indy.AnonCredsApi;
using Hyperledger.Indy.BlobStorageApi;
using Hyperledger.Indy.PoolApi;
using Hyperledger.Indy.Samples.Utils;
using Hyperledger.Indy.WalletApi;
using Newtonsoft.Json.Linq;
using System;
using System.Diagnostics;
using System.Drawing;
using System.Threading.Tasks;
using Console = Colorful.Console;

namespace Hyperledger.Indy.Samples
{
    static class AnonCredsDemo
    {
        public static async Task Execute()
        {
            Console.Write("Executing anoncreds sample... ");

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

                // Open wallets in using statements to ensure they are closed when finished.
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
                    var credDefConfigJson = "{\"support_revocation\":false}";
                    var createCredDefResult = await AnonCreds.IssuerCreateAndStoreCredentialDefAsync(issuerWallet, issuerDid, schemaJson, credDefTag, null, credDefConfigJson);
                    var credDefId = createCredDefResult.CredDefId;
                    var credDefJson = createCredDefResult.CredDefJson;

                    //6. Prover create Master Secret
                    var masterSecretId = await AnonCreds.ProverCreateMasterSecretAsync(proverWallet, null);

                    //7. Issuer Creates Credential Offer
                    var credOffer = await AnonCreds.IssuerCreateCredentialOfferAsync(issuerWallet, credDefId);

                    //8. Prover Creates Credential Request
                    var createCredReqResult = await AnonCreds.ProverCreateCredentialReqAsync(proverWallet, proverDid, credOffer, credDefJson, masterSecretId);
                    var credReqJson = createCredReqResult.CredentialRequestJson;
                    var credReqMetadataJson = createCredReqResult.CredentialRequestMetadataJson;

                    //9. Issuer create Credential
                    var credValuesJson = "{\n" +
                            "        \"sex\": {\"raw\": \"male\", \"encoded\": \"594465709955896723921094925839488742869205008160769251991705001\"},\n" +
                            "        \"name\": {\"raw\": \"Alex\", \"encoded\": \"1139481716457488690172217916278103335\"},\n" +
                            "        \"height\": {\"raw\": \"175\", \"encoded\": \"175\"},\n" +
                            "        \"age\": {\"raw\": \"28\", \"encoded\": \"28\"}\n" +
                            "    }";

                    var createCredentialResult = await AnonCreds.IssuerCreateCredentialAsync(issuerWallet, credOffer, credReqJson, credValuesJson, null, null);
                    var credential = createCredentialResult.CredentialJson;

                    //10. Prover Stores Credential
                    await AnonCreds.ProverStoreCredentialAsync(proverWallet, null, credReqMetadataJson, credential, credDefJson, null);

                    //11. Prover Gets Credentials for Proof Request
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

                    Debug.Assert(credentialsForAttribute1.Count == 1);
                    Debug.Assert(credentialsForAttribute2.Count == 1);
                    Debug.Assert(credentialsForAttribute3.Count == 0);
                    Debug.Assert(credentialsForPredicate.Count == 1);

                    var credentialId = credentialsForAttribute1[0]["cred_info"]["referent"].ToObject<string>();

                    //12. Prover Creates Proof
                    var selfAttestedValue = "8-800-300";
                    var requestedCredentialsJson = string.Format(
                        "{{\n" +
                        "                                          \"self_attested_attributes\":{{\"attr3_referent\":\"{0}\"}},\n" +
                        "                                          \"requested_attributes\":{{\"attr1_referent\":{{\"cred_id\":\"{1}\", \"revealed\":true}},\n" +
                        "                                                                    \"attr2_referent\":{{\"cred_id\":\"{2}\", \"revealed\":false}}}},\n" +
                        "                                          \"requested_predicates\":{{\"predicate1_referent\":{{\"cred_id\":\"{3}\"}}}}\n" +
                        "                                        }}", selfAttestedValue, credentialId, credentialId, credentialId);

                    var schemas = string.Format("{{\"{0}\":{1}}}", schemaId, schemaJson);
                    var credentialDefs = string.Format("{{\"{0}\":{1}}}", credDefId, credDefJson);
                    var revocStates = "{}";

                    var proofJson = await AnonCreds.ProverCreateProofAsync(proverWallet, proofRequestJson, requestedCredentialsJson, masterSecretId, schemas, credentialDefs, revocStates);
                    var proof = JObject.Parse(proofJson);

                    //13. Verifier verify Proof
                    var revealedAttr1 = proof["requested_proof"]["revealed_attrs"]["attr1_referent"];
                    Debug.Assert("Alex" == revealedAttr1["raw"].ToObject<string>());

                    Debug.Assert(null != proof["requested_proof"]["unrevealed_attrs"]["attr2_referent"]["sub_proof_index"]);

                    Debug.Assert(selfAttestedValue == proof["requested_proof"]["self_attested_attrs"]["attr3_referent"].ToObject<string>());

                    var revocRegDefs = "{}";
                    var revocRegs = "{}";

                    var valid = await AnonCreds.VerifierVerifyProofAsync(proofRequestJson, proofJson, schemas, credentialDefs, revocRegDefs, revocRegs);
                    Debug.Assert(valid);

                    //14. Close wallets and pool
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
                //15. Delete wallets and Pool ledger config
                await WalletUtils.DeleteWalletAsync(issuerWalletConfig, issuerWalletCredentials);
                await WalletUtils.DeleteWalletAsync(proverWalletConfig, proverWalletCredentials);
                await PoolUtils.DeletePoolLedgerConfigAsync(PoolUtils.DEFAULT_POOL_NAME);
            }
        }
    }
}
