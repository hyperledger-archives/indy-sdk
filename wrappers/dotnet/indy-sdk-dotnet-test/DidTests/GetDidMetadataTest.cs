using Hyperledger.Indy.DidApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.DidTests
{
    [TestClass]
    public class GetDidMetadataTest : IndyIntegrationTestWithSingleWallet
    {
        [TestMethod]
        public async Task TestGetDidMetadataWorks()
        {
            await Did.SetDidMetadataAsync(wallet, DID, METADATA);
            var receivedMetadata = await Did.GetDidMetadataAsync(wallet, DID);

            Assert.AreEqual(METADATA, receivedMetadata);
        }

        [TestMethod]
        public async Task TestGetDidMetadataWorksForEmptyString()
        {
            await Did.SetDidMetadataAsync(wallet, DID, string.Empty);
            var receivedMetadata = await Did.GetDidMetadataAsync(wallet, DID);

            Assert.AreEqual(string.Empty, receivedMetadata);
        }

        [TestMethod]
        public async Task TestGetDidMetadataWorksForNoMetadata()
        {
            var ex = await Assert.ThrowsExceptionAsync<WalletItemNotFoundException>(() =>
               Did.GetDidMetadataAsync(wallet, DID)
           );
        }
    }
}