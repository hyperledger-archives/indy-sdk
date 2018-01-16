using Hyperledger.Indy.SignusApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.SignusTests
{
    [TestClass]
    public class SetEndpointForDidTest : IndyIntegrationTestWithPoolAndSingleWallet
    {
        [TestMethod]
        public async Task TestSetEndpointForDidWorks()
        {
            await Signus.SetEndpointForDidAsync(wallet, DID1, ENDPOINT, VERKEY);
        }

        [TestMethod]
        public async Task TestSetEndpointForDidWorksForReplace()
        {
            await Signus.SetEndpointForDidAsync(wallet, DID1, ENDPOINT, VERKEY);
            var receivedEndpoint = await Signus.GetEndpointForDidAsync(wallet, pool, DID1);
            Assert.AreEqual(ENDPOINT, receivedEndpoint.Address);
            Assert.AreEqual(VERKEY, receivedEndpoint.TransportKey);

            var newEndpoint = "10.10.10.1:9710";
            await Signus.SetEndpointForDidAsync(wallet, DID1, newEndpoint, VERKEY_MY2);
            var updatedEndpoint = await Signus.GetEndpointForDidAsync(wallet, pool, DID1);

            Assert.AreEqual(newEndpoint, updatedEndpoint.Address);
            Assert.AreEqual(VERKEY_MY2, updatedEndpoint.TransportKey);
        }

        [TestMethod]
        public async Task TestSetEndpointForDidWorksForInvalidDid()
        {
            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
               Signus.SetEndpointForDidAsync(wallet, "invalid_base58string", ENDPOINT, VERKEY)
           );
        }

        [TestMethod]
        public async Task TestSetEndpointForDidWorksForInvalidTransportKey()
        {
            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
               Signus.SetEndpointForDidAsync(wallet, DID1, ENDPOINT, INVALID_VERKEY)
           );
        }
    }
}