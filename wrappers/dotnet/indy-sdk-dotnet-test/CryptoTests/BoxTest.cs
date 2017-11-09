using Hyperledger.Indy.CryptoApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.CryptoTests
{
    [TestClass]
    public class BoxTest : IndyIntegrationTestWithSingleWallet
    {
        [TestMethod]
        public async Task TestBoxWorks()
        {
            var myVk = await Crypto.CreateKeyAsync(wallet, MY1_IDENTITY_KEY_JSON);
            await Crypto.BoxAsync(wallet, myVk, VERKEY_MY2, MESSAGE);
        }

        [TestMethod]
        public async Task TestBoxWorksForUnknownCoder()
        {
            var ex = await Assert.ThrowsExceptionAsync<WalletValueNotFoundException>(() =>
               Crypto.BoxAsync(wallet, VERKEY_MY1, VERKEY_MY2, MESSAGE)
           );
        }
    }
}
