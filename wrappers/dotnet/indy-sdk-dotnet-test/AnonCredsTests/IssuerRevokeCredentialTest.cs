using Hyperledger.Indy.AnonCredsApi;
using Hyperledger.Indy.BlobStorageApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.AnonCredsTests
{
    [TestClass]
    public class IssuerRevokeCredentialTest : AnonCredsIntegrationTestBase
    {
        private readonly string tailsWriterConfig = JsonConvert.SerializeObject(
            new {
                base_dir = EnvironmentUtils.GetIndyHomePath("tails"),
                uri_pattern = string.Empty
            });

        [TestMethod]
        public async Task TestIssuerRevokeProofWorks()
        {
            // Create wallet, get wallet handle
            var walletConfig = JsonConvert.SerializeObject(new { id = "revocationWallet" });
            await Wallet.CreateWalletAsync(walletConfig, CREDENTIALS);
            var wallet = await Wallet.OpenWalletAsync(walletConfig, CREDENTIALS);

            // Issuer create Schema
            var createSchemaResult = await AnonCreds.IssuerCreateSchemaAsync(issuerDid, gvtSchemaName, schemaVersion, gvtSchemaAttributes);
            var schemaId = createSchemaResult.SchemaId;
            var schemaJson = createSchemaResult.SchemaJson;

            // Issuer create issuer1GvtCredential definition
            var revocationCredentialDefConfig = "{\"support_revocation\":true}";
            var createCredentialDefResult = await AnonCreds.IssuerCreateAndStoreCredentialDefAsync(wallet, issuerDid, schemaJson, tag, null, revocationCredentialDefConfig);
            var credDefId = createCredentialDefResult.CredDefId;
            var credDefJson = createCredentialDefResult.CredDefJson;

            // Issuer create revocation registry
            var tailsWriter = await BlobStorage.OpenWriterAsync("default", tailsWriterConfig);
            var revRegConfig = "{\"issuance_type\":null,\"max_cred_num\":5}";
            var createRevRegResult = await AnonCreds.IssuerCreateAndStoreRevocRegAsync(wallet, issuerDid, null, tag, credDefId, revRegConfig, tailsWriter);
            var revRegId = createRevRegResult.RevRegId;
            var revRegDef = createRevRegResult.RevRegDefJson;

            // Prover create Master Secret
            await AnonCreds.ProverCreateMasterSecretAsync(wallet, masterSecretId);

            // Issuer create Credential Offer
            var credOfferJson = await AnonCreds.IssuerCreateCredentialOfferAsync(wallet, credDefId);

            // Prover create Credential Request
            var createCredReqResult =
                    await AnonCreds.ProverCreateCredentialReqAsync(wallet, proverDid, credOfferJson, credDefJson, masterSecretId);
            var credentialReqJson = createCredReqResult.CredentialRequestJson;
            var credentialReqMetadataJson = createCredReqResult.CredentialRequestMetadataJson;

            // Issuer open TailsReader
            var blobReader = await BlobStorage.OpenReaderAsync("default", tailsWriterConfig);

            //9. Issuer create Credential
            var createCredentialResult = await AnonCreds.IssuerCreateCredentialAsync(wallet, credOfferJson, credentialReqJson, gvtCredentialValuesJson, revRegId, blobReader);
            var credJson = createCredentialResult.CredentialJson;
            var credRevocId = createCredentialResult.RevocId;
            var revRegDelta = createCredentialResult.RevocRegDeltaJson;

            // Prover create RevocationState
            var timestamp = 100;
            var revStateJson = await AnonCreds.CreateRevocationStateAsync(blobReader, revRegDef, revRegDelta, timestamp, credRevocId);

            // Prover store received Credential
            await AnonCreds.ProverStoreCredentialAsync(wallet, credentialId1, credentialReqMetadataJson, credJson, credDefJson, revRegDef);

            // Prover gets Credentials for Proof Request
            var credentialsJson = await AnonCreds.ProverGetCredentialsForProofReqAsync(wallet, proofRequest);
            var credentials = JObject.Parse(credentialsJson);
            var credentialsForAttr1 = credentials["attrs"]["attr1_referent"];

            var credentialUuid = credentialsForAttr1[0]["cred_info"]["referent"];

            // Prover create Proof
            var requestedCredentialsJson = string.Format("{{" +
                    "\"self_attested_attributes\":{{}}," +
                    "\"requested_attributes\":{{\"attr1_referent\":{{\"cred_id\":\"{0}\", \"revealed\":true, \"timestamp\":{1} }}}}," +
                    "\"requested_predicates\":{{\"predicate1_referent\":{{\"cred_id\":\"{2}\", \"timestamp\":{3}}}}}" +
                    "}}", credentialUuid, timestamp, credentialUuid, timestamp);

            var schemasJson = JObject.Parse(string.Format("{{\"{0}\":{1}}}", schemaId, schemaJson)).ToString();
            var credentialDefsJson = JObject.Parse(string.Format("{{\"{0}\":{1}}}", credDefId, credDefJson)).ToString();
            var revStatesJson = JObject.Parse(string.Format("{{\"{0}\":{{\"{1}\":{2}}}}}", revRegId, timestamp, revStateJson)).ToString();

            var proofJson = await AnonCreds.ProverCreateProofAsync(wallet, proofRequest, requestedCredentialsJson, masterSecretId, schemasJson, credentialDefsJson, revStatesJson);
            var proof = JObject.Parse(proofJson);

            // Issuer revoke Credential
            revRegDelta = await AnonCreds.IssuerRevokeCredentialAsync(wallet, blobReader, revRegId, credRevocId);

            // Verifier verify proof
            var revealedAttr1 = proof["requested_proof"]["revealed_attrs"]["attr1_referent"];
            Assert.AreEqual("Alex", revealedAttr1["raw"]);

            var revRegDefsJson = JObject.Parse(string.Format("{{\"{0}\":{1}}}", revRegId, revRegDef)).ToString();
            var revRegs = JObject.Parse(string.Format("{{\"{0}\":{{\"{1}\":{2}}}}}", revRegId, timestamp, revRegDelta)).ToString();

            var valid = await AnonCreds.VerifierVerifyProofAsync(proofRequest, proofJson, schemasJson, credentialDefsJson, revRegDefsJson, revRegs);
            Assert.IsFalse(valid);

            //// Close and Delete Wallet
            await wallet.CloseAsync();
            await Wallet.DeleteWalletAsync(walletConfig, CREDENTIALS);
        }

       
    }
}
