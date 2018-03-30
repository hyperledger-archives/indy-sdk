using Hyperledger.Indy.CryptoApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.CryptoTests
{
    [TestClass]
    public class AuthCryptTest : IndyIntegrationTestWithSingleWallet
    {
        [TestMethod]
        public async Task TestAuthCryptWorks()
        {
            var myVk = await Crypto.CreateKeyAsync(wallet, MY1_IDENTITY_KEY_JSON);
            await Crypto.AuthCryptAsync(wallet, myVk, VERKEY_MY2, MESSAGE);
        }

        [TestMethod]
        public async Task TestAuthCryptWorksForUnknownCoder()
        {
            var ex = await Assert.ThrowsExceptionAsync<WalletValueNotFoundException>(() =>
                 Crypto.AuthCryptAsync(wallet, VERKEY_MY1, VERKEY_MY2, MESSAGE)
           );
        }
    }
}
