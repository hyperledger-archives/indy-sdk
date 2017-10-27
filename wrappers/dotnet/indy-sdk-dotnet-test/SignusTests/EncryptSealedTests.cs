using Hyperledger.Indy.LedgerApi;
using Hyperledger.Indy.SignusApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.SignusTests
{
    [TestClass]
    public class EncryptSealedTests : IndyIntegrationTestWithPoolAndSingleWallet
    {
        private string _did;
        private string _verkey;

        [TestInitialize]
        public async Task Before()
        {
            var result = await Signus.CreateAndStoreMyDidAsync(wallet, TRUSTEE_IDENTITY_JSON);
            var trusteeDid = result.Did;

            var nym = await Signus.CreateAndStoreMyDidAsync(wallet, MY1_IDENTITY_JSON);
            _did = nym.Did;
            _verkey = nym.VerKey;

            var nymRequest = await Ledger.BuildNymRequestAsync(trusteeDid, _did, _verkey, null, null);
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, trusteeDid, nymRequest);
        }

        [TestMethod]
        public async Task TestEncryptSealedWorksForPkCachedInWallet()
        {
            var identityJson = string.Format(IDENTITY_JSON_TEMPLATE, _did, _verkey);
            await Signus.StoreTheirDidAsync(wallet, identityJson);

            var encryptResult = await Signus.EncryptSealedAsync(wallet, pool, _did, MESSAGE);
            Assert.IsNotNull(encryptResult);
        }

        [TestMethod]
        public async Task TestEncryptSealedWorksForGetPkFromLedger()
        {
            var identityJson = string.Format("{{\"did\":\"{0}\"}}", _did);
            await Signus.StoreTheirDidAsync(wallet, identityJson);

            var encryptResult = await Signus.EncryptSealedAsync(wallet, pool, _did, MESSAGE);
            Assert.IsNotNull(encryptResult);
        }

        [TestMethod]
        public async Task TestEncryptSealedWorksForGetNymFromLedger()
        {
            var encryptResult = await Signus.EncryptSealedAsync(wallet, pool, _did, MESSAGE);
            Assert.IsNotNull(encryptResult);
        }

        [TestMethod]
        public async Task TestEncryptSealedWorksForNotFoundNym()
        {
            var ex = await Assert.ThrowsExceptionAsync<InvalidStateException>(() =>
                Signus.EncryptSealedAsync(wallet, pool, DID1, MESSAGE)
            );
        }
    }
}
