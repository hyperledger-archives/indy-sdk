using Hyperledger.Indy.DidApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.SignusTests
{
    [TestClass]
    public class GetDidMetadataTest : IndyIntegrationTestWithSingleWallet
    {
        [TestMethod]
        public async Task TestGetDidMetadataWorks()
        {
            await Did.SetDidMetadataAsync(wallet, DID1, METADATA);
            var receivedMetadata = await Did.GetDidMetadataAsync(wallet, DID1);

            Assert.AreEqual(METADATA, receivedMetadata);
        }

        [TestMethod]
        public async Task TestGetDidMetadataWorksForEmptyString()
        {
            await Did.SetDidMetadataAsync(wallet, DID1, string.Empty);
            var receivedMetadata = await Did.GetDidMetadataAsync(wallet, DID1);

            Assert.AreEqual(string.Empty, receivedMetadata);
        }

        [TestMethod]
        public async Task TestGetDidMetadataWorksForNoMetadata()
        {
            var ex = await Assert.ThrowsExceptionAsync<WalletValueNotFoundException>(() =>
               Did.GetDidMetadataAsync(wallet, DID1)
           );
        }
    }
}