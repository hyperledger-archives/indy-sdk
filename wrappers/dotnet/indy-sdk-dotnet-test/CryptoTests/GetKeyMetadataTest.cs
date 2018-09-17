using Hyperledger.Indy.CryptoApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.CryptoTests
{
    [TestClass]
    public class GetKeyMetadataTest : IndyIntegrationTestWithSingleWallet
    {
        [TestMethod]
        public async Task TestGetKeyMetadataWorks()
        {
            await Crypto.SetKeyMetadataAsync(wallet, VERKEY, METADATA);
            var receivedMetadata = await Crypto.GetKeyMetadataAsync(wallet, VERKEY);
            Assert.AreEqual(METADATA, receivedMetadata);
        }

        [TestMethod]
        public async Task TestGetKeyMetadataWorksForEmptyString()
        {
            await Crypto.SetKeyMetadataAsync(wallet, VERKEY, string.Empty);
            var receivedMetadata = await Crypto.GetKeyMetadataAsync(wallet, VERKEY);
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
