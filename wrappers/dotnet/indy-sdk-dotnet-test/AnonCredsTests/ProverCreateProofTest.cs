using Hyperledger.Indy.AnonCredsApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.AnonCredsTests
{
    [TestClass]
    public class ProverCreateProofTest : AnonCredsIntegrationTestBase
    {
        private readonly string requestedCredentialsJson = string.Format("{{" +
            "\"self_attested_attributes\":{{}}," +
            "\"requested_attributes\":{{\"attr1_referent\":{{\"cred_id\":\"{0}\", \"revealed\":true}}}}," +
            "\"requested_predicates\":{{\"predicate1_referent\":{{\"cred_id\":\"{1}\"}}}}" +
            "}}", credentialId1, credentialId1);

        [TestMethod]
        public async Task TestProverCreateProofWorks()
        {
            var schemasJson = new JObject(new JProperty(gvtSchemaId, JObject.Parse(gvtSchema))).ToString();
            var credentialDefsJson = new JObject(new JProperty(issuer1gvtCredDefId, JObject.Parse(issuer1gvtCredDef))).ToString();
            var revocStatesJson = "{}";

            var proofJson = await AnonCreds.ProverCreateProofAsync(
                wallet, 
                proofRequest, 
                requestedCredentialsJson,
                masterSecretId, 
                schemasJson, 
                credentialDefsJson, 
                revocStatesJson);

            Assert.IsNotNull(proofJson);
        }
        
        [TestMethod]
        public async Task TestProverCreateProofWorksForUsingNotSatisfyClaim()
        {
            var requestedCredentialsJson = string.Format("{{\"self_attested_attributes\":{{}},\n" +
                 "                                    \"requested_attributes\":{{\"attr1_referent\":{{\"cred_id\":\"{0}\", \"revealed\":true}}}},\n" +
                 "                                    \"requested_predicates\":{{}}\n" +
                 "                                   }}", credentialId2);

            var schemasJson = new JObject(new JProperty(xyzSchemaId, xyzSchema)).ToString();
            var credentialDefsJson = new JObject(new JProperty(issuer1xyzCredDef, issuer1xyzCredDef)).ToString();
            var revocStatesJson = "{}";

            await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                AnonCreds.ProverCreateProofAsync(wallet, proofRequest, requestedCredentialsJson, 
                    masterSecretId, schemasJson, credentialDefsJson, revocStatesJson)
            );
        }

        [TestMethod]
        public async Task TestProverCreateProofWorksForInvalidMasterSecret()
        {
            var schemasJson = new JObject(new JProperty(gvtSchemaId, JObject.Parse(gvtSchema))).ToString();
            var credentialDefsJson = new JObject(new JProperty(issuer1gvtCredDefId, JObject.Parse(issuer1gvtCredDef))).ToString();
            var revocStatesJson = "{}";
            
            var ex = await Assert.ThrowsExceptionAsync<WalletItemNotFoundException>(() =>
                AnonCreds.ProverCreateProofAsync(wallet, proofRequest, requestedCredentialsJson, "wrong_master_secret", schemasJson, credentialDefsJson, revocStatesJson)
            );
        }

        [TestMethod]
        public async Task TestProverCreateProofWorksForInvalidSchemas()
        {
            var schemasJson = "{}";
            var credentialDefsJson = new JObject(new JProperty(issuer1gvtCredDefId, issuer1gvtCredDef)).ToString();
            var revocStatesJson = "{}";

            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                AnonCreds.ProverCreateProofAsync(wallet, proofRequest, requestedCredentialsJson, masterSecretId, schemasJson, credentialDefsJson, revocStatesJson)

            );
        }

        [TestMethod]
        public async Task TestProverCreateProofWorksForInvalidRequestedClaimsJson()
        {
            var schemasJson = new JObject(new JProperty(gvtSchemaId, gvtSchema)).ToString();
            var credentialDefsJson = new JObject(new JProperty(issuer1gvtCredDefId, issuer1gvtCredDef)).ToString();
            var revocStatesJson = "{}";
            var requestedCredentialsJson = JsonConvert.SerializeObject(
                new { self_attested_attributes = new JObject(), requested_predicates = new JObject() }
            );


            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                AnonCreds.ProverCreateProofAsync(wallet, proofRequest, requestedCredentialsJson, masterSecretId, schemasJson, credentialDefsJson, revocStatesJson)

            );
        }

    }
}
