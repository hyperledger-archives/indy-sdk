using Hyperledger.Indy.DidApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.SignusTests
{
    [TestClass]
    public class SetDidMetadataTest : IndyIntegrationTestWithSingleWallet
    {
        [TestMethod]
        public async Task TestSetDidMetadataWorks()
        {
            await Did.SetDidMetadataAsync(wallet, DID1, METADATA);
        }

        [TestMethod]
        public async Task TestSetDidMetadataWorksForReplace()
        {
            await Did.SetDidMetadataAsync(wallet, DID1, METADATA);
            var receivedMetadata = await Did.GetDidMetadataAsync(wallet, DID1);
            Assert.AreEqual(METADATA, receivedMetadata);

            var newMetadata = "updated metadata";
            await Did.SetDidMetadataAsync(wallet, DID1, newMetadata);

            var updatedMetadata = await Did.GetDidMetadataAsync(wallet, DID1);
            Assert.AreEqual(newMetadata, updatedMetadata);
        }

        [TestMethod]
        public async Task TestSetDidMetadataWorksForEmptyString()
        {
            await Did.SetDidMetadataAsync(wallet, DID1, string.Empty);
        }

        [TestMethod]
        public async Task TestSetDidMetadataWorksForInvalidDid()
        {
            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
               Did.SetDidMetadataAsync(wallet, "invalid_base58string", METADATA)
           );
        }
    }
}
