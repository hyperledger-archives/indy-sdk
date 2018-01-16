using Hyperledger.Indy.LedgerApi;
using Hyperledger.Indy.SignusApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.SignusTests
{
    [TestClass]
    public class EncryptTest : IndyIntegrationTestWithPoolAndSingleWallet
    {
        private string _trusteeDid;
        private string _trusteeVerkey;
        private string _did;
        private string _verkey;

        [TestInitialize]
        public async Task Before()
        {
            var result = await Signus.CreateAndStoreMyDidAsync(wallet, TRUSTEE_IDENTITY_JSON);
            _trusteeDid = result.Did;
            _trusteeVerkey = result.VerKey;

            var nym = await Signus.CreateAndStoreMyDidAsync(wallet, MY1_IDENTITY_JSON);
            _did = nym.Did;
            _verkey = nym.VerKey;

            var nymRequest = await Ledger.BuildNymRequestAsync(_trusteeDid, _did, _verkey, null, null);
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, _trusteeDid, nymRequest);
        }

        [TestMethod]
        public async Task TestEncryptWorksForPkCachedInWallet()
        {
            var identityJson = string.Format(IDENTITY_JSON_TEMPLATE, _did, _verkey);
            await Signus.StoreTheirDidAsync(wallet, identityJson);

            var encryptResult = await Signus.EncryptAsync(wallet, pool, _trusteeDid, _did, MESSAGE);
            Assert.IsNotNull(encryptResult);
        }

        [TestMethod]
        public async Task TestEncryptWorksForGetNymFromLedger()
        {
            var encryptResult = await Signus.EncryptAsync(wallet, pool, _trusteeDid, _did, MESSAGE);
            Assert.IsNotNull(encryptResult);
        }

        [TestMethod]
        public async Task TestEncryptWorksForUnknownMyDid()
        {
            var identityJson = string.Format(IDENTITY_JSON_TEMPLATE, _trusteeDid, _trusteeVerkey);
            await Signus.StoreTheirDidAsync(wallet, identityJson);

            var ex = await Assert.ThrowsExceptionAsync<WalletValueNotFoundException>(() =>
                Signus.EncryptAsync(wallet, pool, DID1, _trusteeDid, MESSAGE)
            );
        }

        [TestMethod]
        public async Task TestEncryptWorksForNotFoundNym()
        {
            var nym = await Signus.CreateAndStoreMyDidAsync(wallet, "{}");

            var ex = await Assert.ThrowsExceptionAsync<InvalidStateException>(() =>
               Signus.EncryptAsync(wallet, pool, _trusteeDid, DID1, MESSAGE)
            );
        }
    }
}
