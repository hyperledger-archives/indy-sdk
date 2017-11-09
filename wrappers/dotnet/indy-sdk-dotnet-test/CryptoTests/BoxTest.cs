using Hyperledger.Indy.CryptoApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.CryptoTests
{
    [TestClass]
    public class BoxTest : CryptoIntegrationTestBase
    {
        [TestMethod] //Not sure if this is a good test, but since the encrypted content is not static...
        public async Task TestBoxWorks()
        {
            var boxResult = await Crypto.BoxAsync(wallet, senderVerKey, recipientVerKey, MESSAGE);

            var decryptedMessage = await Crypto.BoxOpenAsync(wallet, recipientVerKey, senderVerKey, boxResult.EncryptedMessage, boxResult.Nonce);
            Assert.IsTrue(MESSAGE.SequenceEqual(decryptedMessage));
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
