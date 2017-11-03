using Hyperledger.Indy.SignusApi;
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
            await Signus.SetDidMetadataAsync(wallet, DID1, METADATA);
        }

        [TestMethod]
        public async Task TestSetDidMetadataWorksForReplace()
        {
            await Signus.SetDidMetadataAsync(wallet, DID1, METADATA);
            var receivedMetadata = await Signus.GetDidMetadataAsync(wallet, DID1);
            Assert.AreEqual(METADATA, receivedMetadata);

            var newMetadata = "updated metadata";
            await Signus.SetDidMetadataAsync(wallet, DID1, newMetadata);

            var updatedMetadata = await Signus.GetDidMetadataAsync(wallet, DID1);
            Assert.AreEqual(newMetadata, updatedMetadata);
        }

        [TestMethod]
        public async Task TestSetDidMetadataWorksForEmptyString()
        {
            await Signus.SetDidMetadataAsync(wallet, DID1, string.Empty);
        }

        [TestMethod]
        public async Task TestSetDidMetadataWorksForInvalidDid()
        {
            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
               Signus.SetDidMetadataAsync(wallet, "invalid_base58string", METADATA)
           );
        }
    }
}
