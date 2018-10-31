using Hyperledger.Indy.CryptoApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.CryptoTests
{
    [TestClass]
    public class SetKeyMetadataTest : IndyIntegrationTestWithSingleWallet
    {
        private string key;

        [TestInitialize]
        public async Task CreateKey()
        {
            key = await Crypto.CreateKeyAsync(wallet, "{}");
        }

        [TestMethod]
        public async Task TestSetKeyMetadataWorks()
        {
            await Crypto.SetKeyMetadataAsync(wallet, key, METADATA);
        }

        [TestMethod]
        public async Task TestSetKeyMetadataWorksForReplace()
        {
            await Crypto.SetKeyMetadataAsync(wallet, key, METADATA);
            var receivedMetadata = await Crypto.GetKeyMetadataAsync(wallet, key);
            Assert.AreEqual(METADATA, receivedMetadata);

            var newMetadata = "updated metadata";
            await Crypto.SetKeyMetadataAsync(wallet, key, newMetadata);
            var updatedMetadata = await Crypto.GetKeyMetadataAsync(wallet, key);

            Assert.AreEqual(newMetadata, updatedMetadata);
        }

        [TestMethod]
        public async Task TestSetKeyMetadataWorksForEmptyString()
        {
            await Crypto.SetKeyMetadataAsync(wallet, key, string.Empty);
        }

        [TestMethod]
        public async Task TestSetKeyMetadataWorksForNoKey()
        {
            await Crypto.SetKeyMetadataAsync(wallet, VERKEY, METADATA);
        }

    }
}
