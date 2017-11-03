using Hyperledger.Indy.LedgerApi;
using Hyperledger.Indy.SignusApi;
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
            var result = await Signus.CreateAndStoreMyDidAsync(wallet, "{}");
            var did = result.Did;
            var key = result.VerKey;

            var receivedKey = await Signus.KeyForDidAsync(pool, wallet, did);

            Assert.AreEqual(key, receivedKey);
        }

        [TestMethod]
        public async Task TestKeyForDidWorksForTheirDid()
        {
            var identityJson = string.Format("{{\"did\":\"{0}\", \"verkey\":\"{1}\"}}", DID_FOR_MY1_SEED, VERKEY_FOR_MY1_SEED);
            await Signus.StoreTheirDidAsync(wallet, identityJson);

            var receivedKey = await Signus.KeyForDidAsync(pool, wallet, DID_FOR_MY1_SEED);

            Assert.AreEqual(VERKEY_FOR_MY1_SEED, receivedKey);
        }

        [TestMethod]
        public async Task TestKeyForDidWorksForGetKeyFromLedger()
        {
            var result = await Signus.CreateAndStoreMyDidAsync(wallet, TRUSTEE_IDENTITY_JSON);
            var trusteeDid = result.Did;

            var identityJson = string.Format("{{\"did\":\"{0}\", \"verkey\":\"{1}\"}}", DID_FOR_MY1_SEED, VERKEY_FOR_MY1_SEED);
            await Signus.StoreTheirDidAsync(wallet, identityJson);

            var nymRequest = await Ledger.BuildNymRequestAsync(trusteeDid, DID_FOR_MY1_SEED, VERKEY_FOR_MY1_SEED, null, null);
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, trusteeDid, nymRequest);

            var receivedKey = await Signus.KeyForDidAsync(pool, wallet, DID_FOR_MY1_SEED);

            Assert.AreEqual(VERKEY_FOR_MY1_SEED, receivedKey);
        }

        [TestMethod]
        public async Task TestKeyForDidWorksForNoKey()
        {
            var ex = await Assert.ThrowsExceptionAsync<InvalidStateException>(() =>
               Signus.KeyForDidAsync(pool, wallet, DID_FOR_MY2_SEED)
           );
        }
    }
}
