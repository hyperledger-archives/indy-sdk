using Hyperledger.Indy.AnonCredsApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.AnonCredsTests
{
    [TestClass]
    public class IssuerCreateCredentialTest : AnonCredsIntegrationTestBase
    {
        [TestMethod]
        public async Task TestIssuerCreateAndStoreCredentialWorks()
        {
        }

        [TestMethod]
        public async Task TestIssuerCreateAndStoreCredentialWorksForCredentialValuesDoNotCorrespondToCredentialRequest()
        {
            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                AnonCreds.IssuerCreateCredentialAsync(wallet, issuer1GvtCredOffer, issuer1GvtCredReq, xyzCredentialValuesJson, null, null)
            );
        }

        [TestMethod]
        public async Task TestIssuerCreateAndStoreCredentialReqWorksForInvalidCredentialValues()
        {
            var credentialValues = "{\"sex\":\"male\",\n" +
                    "        \"age\":\"28\"" +
                    "       }";

            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                AnonCreds.IssuerCreateCredentialAsync(wallet, issuer1GvtCredOffer, issuer1GvtCredReq, credentialValues, null, null)
            );
        }
    }
}
