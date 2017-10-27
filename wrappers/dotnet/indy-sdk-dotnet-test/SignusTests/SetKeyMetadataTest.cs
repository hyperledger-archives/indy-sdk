using Hyperledger.Indy.SignusApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.SignusTests
{
    [TestClass]
    public class SetKeyMetadataTest : IndyIntegrationTestWithSingleWallet
    {
        [TestMethod]
        public async Task TestSetKeyMetadataWorks()
        {
            await Signus.SetKeyMetadataAsync(wallet, VERKEY, METADATA);
        }

        [TestMethod]
        public async Task TestSetKeyMetadataWorksForReplace()
        {
            await Signus.SetKeyMetadataAsync(wallet, VERKEY, METADATA);
            var receivedMetadata = await Signus.GetKeyMetadataAsync(wallet, VERKEY);
            Assert.AreEqual(METADATA, receivedMetadata);

            var newMetadata = "updated metadata";
            await Signus.SetKeyMetadataAsync(wallet, VERKEY, newMetadata);
            var updatedMetadata = await Signus.GetKeyMetadataAsync(wallet, VERKEY);

            Assert.AreEqual(newMetadata, updatedMetadata);
        }

        [TestMethod]
        public async Task TestSetKeyMetadataWorksForEmptyString()
        {
            await Signus.SetKeyMetadataAsync(wallet, VERKEY, string.Empty);
        }

    }
}
