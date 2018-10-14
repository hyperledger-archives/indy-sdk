using Hyperledger.Indy.DidApi;
using Hyperledger.Indy.LedgerApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.DidTests
{
    [TestClass]
    public class GetEndpointForDidTest : IndyIntegrationTestWithPoolAndSingleWallet
    {
        [TestMethod]
        public async Task TestGetEndpointForDidWorks()
        {
            await Did.SetEndpointForDidAsync(wallet, DID, ENDPOINT, VERKEY);
            var receivedEndpoint = await Did.GetEndpointForDidAsync(wallet, pool, DID);
            Assert.AreEqual(ENDPOINT, receivedEndpoint.Address);
            Assert.AreEqual(VERKEY, receivedEndpoint.TransportKey);
        }

        [TestMethod]
        public async Task TestGetEndpointForDidWorksFromLedger()
        {
            var trusteeDidResult = await Did.CreateAndStoreMyDidAsync(wallet, TRUSTEE_IDENTITY_JSON);
            var trusteeDid = trusteeDidResult.Did;
            var trusteeVerKey = trusteeDidResult.VerKey;

            var endpoint = string.Format("{{\"endpoint\":{{\"ha\":\"{0}\",\"verkey\":\"{1}\"}}}}", ENDPOINT, trusteeVerKey);

            var attribRequest = await Ledger.BuildAttribRequestAsync(trusteeDid, trusteeDid, null, endpoint, null);
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, trusteeDid, attribRequest);

            var receivedEndpoint = await Did.GetEndpointForDidAsync(wallet, pool, trusteeDid);
            Assert.AreEqual(ENDPOINT, receivedEndpoint.Address);
            Assert.AreEqual(trusteeVerKey, receivedEndpoint.TransportKey);
        }

        [TestMethod]
        public async Task TestGetEndpointForDidWorksForUnknownDid()
        {
            var ex = await Assert.ThrowsExceptionAsync<InvalidStateException>(() =>
               Did.GetEndpointForDidAsync(wallet, pool, DID)
           );
        }
    }
}