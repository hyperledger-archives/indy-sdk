using Hyperledger.Indy.AnonCredsApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.AnonCredsTests
{
    [TestClass]
    public class ProverGetCredentialTest : AnonCredsIntegrationTestBase
    {
        [TestMethod]
        public async Task TestProverGetCredentialWorks()
        {
            var credentialJson = await AnonCreds.ProverGetCredentialAsync(wallet, credentialId1);

            var credential = JObject.Parse(credentialJson);

            var expected = JObject.FromObject(new
            {
                schema_id = gvtSchemaId,
                cred_def_id = issuer1gvtCredDefId,
                referent = credentialId1,
                rev_reg_id = (object)null,
                cred_rev_id = (object)null,
                attrs = new
                {
                    sex = "male",
                    name = "Alex",
                    height = "175",
                    age = "28",
                }
            });

            Assert.IsTrue(JToken.DeepEquals(expected, credential));
        }

        [TestMethod]
        public async Task TestProverGetCredentialWorksForNotFound()
        {
            var ex = await Assert.ThrowsExceptionAsync<WalletItemNotFoundException>(() =>
                AnonCreds.ProverGetCredentialAsync(wallet, "other_cred_id")
            );
        }
    }
}
