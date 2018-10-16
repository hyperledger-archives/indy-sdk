using Hyperledger.Indy.CryptoApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.CryptoTests
{
    [TestClass]
    public class GetKeyMetadataTest : IndyIntegrationTestWithSingleWallet
    {
        private string key;

        [TestInitialize]
        public async Task CreateKey()
        {
            key = await Crypto.CreateKeyAsync(wallet, "{}");
        }

        [TestMethod]
        public async Task TestGetKeyMetadataWorks()
        {
            await Crypto.SetKeyMetadataAsync(wallet, key, METADATA);
            var receivedMetadata = await Crypto.GetKeyMetadataAsync(wallet, key);
            Assert.AreEqual(METADATA, receivedMetadata);
        }

        [TestMethod]
        public async Task TestGetKeyMetadataWorksForEmptyString()
        {
            await Crypto.SetKeyMetadataAsync(wallet, key, string.Empty);
            var receivedMetadata = await Crypto.GetKeyMetadataAsync(wallet, key);
            Assert.AreEqual(string.Empty, receivedMetadata);
        }


        [TestMethod]
        public async Task TestGetKeyMetadataWorksForNoKey()
        {
            await Crypto.SetKeyMetadataAsync(wallet, key, string.Empty);
            var receivedMetadata = await Crypto.GetKeyMetadataAsync(wallet, key);
            Assert.AreEqual(string.Empty, receivedMetadata);
        }

        [TestMethod]
        public async Task TestGetKeyMetadataWorksForNoMetadata()
        {
            var ex = await Assert.ThrowsExceptionAsync<WalletItemNotFoundException>(() =>
               Crypto.GetKeyMetadataAsync(wallet, VERKEY)
           );
        }
    }
}
