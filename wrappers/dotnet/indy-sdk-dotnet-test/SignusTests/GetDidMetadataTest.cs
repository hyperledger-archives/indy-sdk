using Hyperledger.Indy.SignusApi;
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
            await Signus.SetDidMetadataAsync(wallet, DID1, METADATA);
            var receivedMetadata = await Signus.GetDidMetadataAsync(wallet, DID1);

            Assert.AreEqual(METADATA, receivedMetadata);
        }

        [TestMethod]
        public async Task TestGetDidMetadataWorksForEmptyString()
        {
            await Signus.SetDidMetadataAsync(wallet, DID1, string.Empty);
            var receivedMetadata = await Signus.GetDidMetadataAsync(wallet, DID1);

            Assert.AreEqual(string.Empty, receivedMetadata);
        }

        [TestMethod]
        public async Task TestGetDidMetadataWorksForNoMetadata()
        {
            var ex = await Assert.ThrowsExceptionAsync<WalletValueNotFoundException>(() =>
               Signus.GetDidMetadataAsync(wallet, DID1)
           );
        }
    }
}