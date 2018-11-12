using Hyperledger.Indy.CryptoApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.CryptoTests
{
    [TestClass]
    public class AnonDecryptTest : IndyIntegrationTestWithSingleWallet
    {
        [TestMethod]
        public async Task TestAnonDecryptWorks()
        {
            var paramJson = string.Format("{{\"seed\":\"{0}\"}}", MY2_SEED);
            var theirVk = await Crypto.CreateKeyAsync(wallet, paramJson);

            var encryptedMessage = await Crypto.AnonCryptAsync(theirVk, MESSAGE);
            var decryptedMessage = await Crypto.AnonDecryptAsync(wallet, theirVk, encryptedMessage);

            Assert.IsTrue(MESSAGE.SequenceEqual(decryptedMessage));
        }

        [TestMethod]
        public async Task TestAnonDecryptWorksForInvalidMessage()
        {
            var myVk = await Crypto.CreateKeyAsync(wallet, "{}");
            var msg = "unencrypted message";

            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                Crypto.AnonDecryptAsync(wallet, myVk, Encoding.UTF8.GetBytes(msg))
           );
        }

        [TestMethod]
        public async Task TestParseMsgWorksForUnknownRecipientVk()
        {
            var encryptedMessage = await Crypto.AnonCryptAsync(VERKEY, MESSAGE);

            var ex = await Assert.ThrowsExceptionAsync<WalletItemNotFoundException>(() =>
                Crypto.AnonDecryptAsync(wallet, VERKEY, encryptedMessage)
           );
        }
       
    }
}
