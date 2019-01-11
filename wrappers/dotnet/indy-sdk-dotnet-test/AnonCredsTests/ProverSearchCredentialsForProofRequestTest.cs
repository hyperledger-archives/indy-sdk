using Hyperledger.Indy.AnonCredsApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.AnonCredsTests
{
    [TestClass]
    public class ProverSearchCredentialsForProofRequestTest : AnonCredsIntegrationTestBase
    {
        [TestMethod]
        public async Task TestProverSearchCredentialsForProofRequestWorks()
        {
            var proofRequest = "{" +
                 "              \"nonce\":\"123432421212\"," +
                 "              \"name\":\"proof_req_1\"," +
                 "              \"version\":\"0.1\"," +
                 "              \"requested_attributes\":{" +
                 "                   \"attr1_referent\":{\"name\":\"name\"}" +
                 "               }," +
                 "              \"requested_predicates\":{}" +
                 "          }";

            var credentialsSearch = await AnonCreds.ProverSearchCredentialsForProofRequestAsync(wallet, proofRequest);
            var credentialsForAttribute1 = await credentialsSearch.NextAsync("attr1_referent", 100);
            var jsonArray = JArray.Parse(credentialsForAttribute1);

            Assert.AreEqual(2, jsonArray.Count);

            //TODO: Shouldn't there be an explicit close of the credential search here?
        }

        [TestMethod]
        public async Task TestProverSearchCredentialsForProofRequestWorksForNotFound()
        {
            var proofRequest = "{" +
                 "              \"nonce\":\"123432421212\"," +
                 "              \"name\":\"proof_req_1\"," +
                 "              \"version\":\"0.1\"," +
                 "              \"requested_attributes\":{" +
                 "                   \"attr1_referent\":{\"name\":\"not_found_attr\"}" +
                 "               }," +
                 "              \"requested_predicates\":{}" +
                 "          }";

            var credentialsSearch = await AnonCreds.ProverSearchCredentialsForProofRequestAsync(wallet, proofRequest);
            var credentialsForAttribute1 = await credentialsSearch.NextAsync("attr1_referent", 100);
            var jsonArray = JArray.Parse(credentialsForAttribute1);

            Assert.AreEqual(0, jsonArray.Count);

            //TODO: Shouldn't there be an explicit close of the credential search here?
        }

        [TestMethod]
        public async Task testProverSearchCredentialsForProofRequestWorksForRevealedAttributeAndExtraQuery()
        {
            var proofRequest = "{" +
                 "              \"nonce\":\"123432421212\"," +
                 "              \"name\":\"proof_req_1\"," +
                 "              \"version\":\"0.1\"," +
                 "              \"requested_attributes\":{" +
                 "                   \"attr1_referent\":{\"name\":\"name\"}" +
                 "               }," +
                 "              \"requested_predicates\":{}" +
                 "          }";

            var extraQuery = "{\"attr1_referent\": { \"attr::name::value\": \"Alex\"}}";

            var credentialsSearch = await AnonCreds.ProverSearchCredentialsForProofRequestAsync(wallet, proofRequest, extraQuery);
            var credentialsForAttribute1 = await credentialsSearch.NextAsync("attr1_referent", 100);
            var jsonArray = JArray.Parse(credentialsForAttribute1);

            Assert.AreEqual(1, jsonArray.Count);

            //TODO: Shouldn't there be an explicit close of the credential search here?
        }
    }
}
