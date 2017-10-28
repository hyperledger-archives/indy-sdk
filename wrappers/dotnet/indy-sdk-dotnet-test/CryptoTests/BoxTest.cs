using Hyperledger.Indy.CryptoApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.CryptoTests
{
    [TestClass]
    public class BoxTest : CryptoIntegrationTestBase
    {
        [TestMethod]
        [Ignore]
        public async Task TestBoxWorks()
        {
            //Not sure how to implement this test.
        }

        [TestMethod]
        public async Task TestBoxFailsIfSenderKeyNotInWallet()
        {
            var ex = await Assert.ThrowsExceptionAsync<WalletValueNotFoundException>(() =>
               Crypto.BoxAsync(wallet, KEY_NOT_IN_WALLET, recipientVerKey, MESSAGE)
           );
        }
    }
}
