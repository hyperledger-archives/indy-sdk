using Hyperledger.Indy.CryptoApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.CryptoTests
{
    [TestClass]
    public class SignTest : IndyIntegrationTestWithSingleWallet
    {
        [TestMethod]
        public async Task TestCryptoSignWorks()
        {
            var keyJson = string.Format("{{\"seed\":\"{0}\"}}", TRUSTEE_SEED);
            var key = await Crypto.CreateKeyAsync(wallet, keyJson);

            var signature = await Crypto.SignAsync(wallet, key, MESSAGE);            
            Assert.IsTrue(SIGNATURE.SequenceEqual(signature));
        }

        [TestMethod]
        public async Task TestCryptoSignWorksForUnknowSigner()
        {
            var ex = await Assert.ThrowsExceptionAsync<WalletItemNotFoundException>(() =>
               Crypto.SignAsync(wallet, VERKEY_TRUSTEE, MESSAGE)
           );
        }
    }
}
