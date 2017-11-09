using Hyperledger.Indy.SignusApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.SignusTests
{
    [TestClass]
    public class KeyForLocalDidTest : IndyIntegrationTestWithSingleWallet
    {
        [TestMethod]
        public async Task TestKeyForLocalDidWorksForMyDid()
        {
            var result = await Signus.CreateAndStoreMyDidAsync(wallet, "{}");
            var did = result.Did;
            var key = result.VerKey;

            var receivedKey = await Signus.KeyForLocalDidAsync(wallet, did);

            Assert.AreEqual(key, receivedKey);
        }

        [TestMethod]
        public async Task TestKeyForLocalDidWorksForTheirDid()
        {
            var identityJson = string.Format(IDENTITY_JSON_TEMPLATE, DID_MY1, VERKEY_MY1);
            await Signus.StoreTheirDidAsync(wallet, identityJson);

            var receivedKey = await Signus.KeyForLocalDidAsync(wallet, DID_MY1);

            Assert.AreEqual(VERKEY_MY1, receivedKey);
        }

        [TestMethod]
        public async Task TestKeyForDidWorksForNoKey()
        {
            var ex = await Assert.ThrowsExceptionAsync<WalletValueNotFoundException>(() =>
               Signus.KeyForLocalDidAsync(wallet, DID_MY2)
           );
        }
    }
}
