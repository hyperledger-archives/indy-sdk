using Hyperledger.Indy.CryptoApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.CryptoTests
{
    [TestClass]
    public class SetKeyMetadataTest : IndyIntegrationTestWithSingleWallet
    {
        [TestMethod]
        public async Task TestSetKeyMetadataWorks()
        {
            await Crypto.SetKeyMetadataAsync(wallet, VERKEY, METADATA);
        }

        [TestMethod]
        public async Task TestSetKeyMetadataWorksForReplace()
        {
            await Crypto.SetKeyMetadataAsync(wallet, VERKEY, METADATA);
            var receivedMetadata = await Crypto.GetKeyMetadataAsync(wallet, VERKEY);
            Assert.AreEqual(METADATA, receivedMetadata);

            var newMetadata = "updated metadata";
            await Crypto.SetKeyMetadataAsync(wallet, VERKEY, newMetadata);
            var updatedMetadata = await Crypto.GetKeyMetadataAsync(wallet, VERKEY);

            Assert.AreEqual(newMetadata, updatedMetadata);
        }

        [TestMethod]
        public async Task TestSetKeyMetadataWorksForEmptyString()
        {
            await Crypto.SetKeyMetadataAsync(wallet, VERKEY, string.Empty);
        }

    }
}
