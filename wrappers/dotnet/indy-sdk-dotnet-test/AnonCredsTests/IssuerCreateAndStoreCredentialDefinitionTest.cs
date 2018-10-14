using Hyperledger.Indy.AnonCredsApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;


namespace Hyperledger.Indy.Test.AnonCredsTests
{
    [TestClass]
    public class IssuerCreateAndStoreCredentialDefinitionTest : AnonCredsIntegrationTestBase
    {

        [TestMethod]
        public async Task TestIssuerCreateAndStoreCredentialDefWorks()
        {
        }



        [TestMethod]
        public async Task TestIssuerCreateAndStoreCredentialDefWorksForInvalidSchemaJson()
        {
            var schema = "{\"seqNo\":1, \"name\":\"name\",\"version\":\"1.0\", \"attr_names\":[\"name\"]}";

            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                AnonCreds.IssuerCreateAndStoreCredentialDefAsync(wallet, issuerDid, schema, "invalidSchema", null, defaultCredentialDefinitionConfig)
            );
        }

        [TestMethod]
        public async Task TestIssuerCreateAndStoreCredentialDefWorksForEmptyKeys()
        {
            var schema = "{\n" +
                "                        \"id\":\"1\",\n" +
                "                        \"name\":\"gvt\",\n" +
                "                        \"version\":\"1.0\",\n" +
                "                        \"attr_names\":[]\n" +
                "                 }";

            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                AnonCreds.IssuerCreateAndStoreCredentialDefAsync(wallet, issuerDid, schema, "EmptyKeys", null, defaultCredentialDefinitionConfig)
            );
        }

        [TestMethod]
        public async Task TestIssuerCreateAndStoreCredentialDefWorksForDuplicate()
        {
            var ex = await Assert.ThrowsExceptionAsync<CredentialDefinitionAlreadyExistsException>(() =>
                AnonCreds.IssuerCreateAndStoreCredentialDefAsync(wallet, issuerDid, gvtSchema, tag, null, defaultCredentialDefinitionConfig)
            );
        }
    }
}
