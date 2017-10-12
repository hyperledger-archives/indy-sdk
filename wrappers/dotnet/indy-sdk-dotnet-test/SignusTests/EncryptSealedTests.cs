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
        private String did;
        private String verkey;

        [TestInitialize]
        public async Task Before()
        {
            var result = await Signus.CreateAndStoreMyDidAsync(wallet, TRUSTEE_IDENTITY_JSON);
            var trusteeDid = result.Did;

            var nym = await Signus.CreateAndStoreMyDidAsync(wallet, MY1_IDENTITY_JSON);
            did = nym.Did;
            verkey = nym.VerKey;

            var nymRequest = await Ledger.BuildNymRequestAsync(trusteeDid, did, verkey, null, null);
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, trusteeDid, nymRequest);
        }

        [TestMethod]
        public async Task TestEncryptSealedWorksForPkCachedInWallet()
        {
            var identityJson = string.Format(IDENTITY_JSON_TEMPLATE, did, verkey);
            await Signus.StoreTheirDidAsync(wallet, identityJson);

            var encryptResult = await Signus.EncryptSealedAsync(wallet, pool, did, MESSAGE);
            Assert.IsNotNull(encryptResult);
        }

        [TestMethod]
        public async Task TestEncryptSealedWorksForGetPkFromLedger()
        {
            var identityJson = string.Format("{{\"did\":\"{0}\"}}", did);
            await Signus.StoreTheirDidAsync(wallet, identityJson);

            var encryptResult = await Signus.EncryptSealedAsync(wallet, pool, did, MESSAGE);
            Assert.IsNotNull(encryptResult);
        }

        [TestMethod]
        public async Task TestEncryptSealedWorksForGetNymFromLedger()
        {
            var encryptResult = await Signus.EncryptSealedAsync(wallet, pool, did, MESSAGE);
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
