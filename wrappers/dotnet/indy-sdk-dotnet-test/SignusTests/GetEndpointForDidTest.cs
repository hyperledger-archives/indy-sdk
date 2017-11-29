using Hyperledger.Indy.SignusApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.SignusTests
{
    [TestClass]
    public class GetEndpointForDidTest : IndyIntegrationTestWithSingleWallet
    {
        [TestMethod]
        public async Task TestGetEndpointForDidWorks()
        {
            await Signus.SetEndpointForDidAsync(wallet, DID1, ENDPOINT, VERKEY);
            var receivedEndpoint = await Signus.GetEndpointForDidAsync(wallet, DID1);
            Assert.AreEqual(ENDPOINT, receivedEndpoint.Address);
            Assert.AreEqual(VERKEY, receivedEndpoint.TransportKey);
        }

        [TestMethod]
        public async Task TestGetEndpointForDidWorksForUnknownDid()
        {
            var ex = await Assert.ThrowsExceptionAsync<WalletValueNotFoundException>(() =>
               Signus.GetEndpointForDidAsync(wallet, DID1)
           );
        }
    }
}