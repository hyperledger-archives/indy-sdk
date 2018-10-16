using Hyperledger.Indy.AnonCredsApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.AnonCredsTests
{
    [TestClass]
    public class ProverStoreCredentialTest : AnonCredsIntegrationTestBase
    {
        [TestMethod]
        public async Task TestProverStoreCredentialWorks()
        {
        }

        [TestMethod]
        public async Task TestProverStoreCredentialWorksForInvalidCredentialJson()
        {
            await AnonCreds.ProverCreateCredentialReqAsync(wallet, proverDid, issuer1GvtCredOffer, issuer1gvtCredDef, masterSecretId);

            var credentialJson = "{\"issuer1GvtCredential\":{\"sex\":[\"male\",\"1\"],\"age\":[\"28\",\"28\"],\"name\":[\"Alex\",\"1\"],\"height\":[\"175\",\"175\"]},\n" +
                    "            \"issuer_did\":1}";


            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                AnonCreds.ProverStoreCredentialAsync(wallet, credentialId1, issuer1GvtCredReqMetadata, credentialJson, issuer1gvtCredDef, null)
            );

        }
    }
}
