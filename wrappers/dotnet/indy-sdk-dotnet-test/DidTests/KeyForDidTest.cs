using Hyperledger.Indy.LedgerApi;
using Hyperledger.Indy.DidApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.SignusTests
{

    [TestClass]
    public class KeyForDidTest : IndyIntegrationTestWithPoolAndSingleWallet
    {
        [TestMethod]
        public async Task TestKeyForDidWorksForMyDid()
        {
            var result = await Did.CreateAndStoreMyDidAsync(wallet, "{}");
            var did = result.Did;
            var key = result.VerKey;

            var receivedKey = await Did.KeyForDidAsync(pool, wallet, did);

            Assert.AreEqual(key, receivedKey);
        }

        [TestMethod]
        public async Task TestKeyForDidWorksForTheirDid()
        {
            var identityJson = string.Format(IDENTITY_JSON_TEMPLATE, DID_MY1, VERKEY_MY1);
            await Did.StoreTheirDidAsync(wallet, identityJson);

            var receivedKey = await Did.KeyForDidAsync(pool, wallet, DID_MY1);

            Assert.AreEqual(VERKEY_MY1, receivedKey);
        }

        [TestMethod]
        public async Task TestKeyForDidWorksForGetKeyFromLedger()
        {
            var result = await Did.CreateAndStoreMyDidAsync(wallet, TRUSTEE_IDENTITY_JSON);
            var trusteeDid = result.Did;

            var identityJson = string.Format(IDENTITY_JSON_TEMPLATE, DID_MY1, VERKEY_MY1);
            await Did.StoreTheirDidAsync(wallet, identityJson);

            var nymRequest = await Ledger.BuildNymRequestAsync(trusteeDid, DID_MY1, VERKEY_MY1, null, null);
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, trusteeDid, nymRequest);

            var receivedKey = await Did.KeyForDidAsync(pool, wallet, DID_MY1);

            Assert.AreEqual(VERKEY_MY1, receivedKey);
        }

        [TestMethod]
        public async Task TestKeyForDidWorksForNoKey()
        {
            var ex = await Assert.ThrowsExceptionAsync<InvalidStateException>(() =>
               Did.KeyForDidAsync(pool, wallet, DID_MY2)
           );
        }
    }
}
