using Hyperledger.Indy.CryptoApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.CryptoTests
{
    [TestClass]
    public class SignTest : CryptoIntegrationTestBase
    {
        [TestMethod]
        public async Task TestSignWorks()
        {
            var signature = await Crypto.SignAsync(wallet, senderVerKey, MESSAGE);            
            Assert.IsTrue(SIGNATURE.SequenceEqual(signature));
        }

        [TestMethod]
        public async Task TestSignFailsIfKeyNotInWallet()
        {
            var ex = await Assert.ThrowsExceptionAsync<WalletValueNotFoundException>(() =>
               Crypto.SignAsync(wallet, KEY_NOT_IN_WALLET, MESSAGE)
           );
        }
    }
}
