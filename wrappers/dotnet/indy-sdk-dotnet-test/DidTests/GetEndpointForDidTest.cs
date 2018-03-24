using Hyperledger.Indy.LedgerApi;
using Hyperledger.Indy.DidApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.SignusTests
{
    [TestClass]
    public class GetEndpointForDidTest : IndyIntegrationTestWithPoolAndSingleWallet
    {
        [TestMethod]
        public async Task TestGetEndpointForDidWorks()
        {
            await Did.SetEndpointForDidAsync(wallet, DID1, ENDPOINT, VERKEY);
            var receivedEndpoint = await Did.GetEndpointForDidAsync(wallet, pool, DID1);
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
               Did.GetEndpointForDidAsync(wallet, pool, DID1)
           );
        }
    }
}