using Hyperledger.Indy.CryptoApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.CryptoTests
{
    [TestClass]
    public class AnonDecryptTest : IndyIntegrationTestWithSingleWallet
    {
        [TestMethod]
        public async Task TestAnonDecryptWorks()
        {
            var verkey = await Crypto.CreateKeyAsync(wallet, MY1_IDENTITY_KEY_JSON);

            var encryptedMessage = await Crypto.AnonCryptAsync(verkey, MESSAGE);
            var decryptedMessage = await Crypto.AnonDecryptAsync(wallet, verkey, encryptedMessage);

            Assert.IsTrue(MESSAGE.SequenceEqual(decryptedMessage));
        }

        [TestMethod]
        public async Task TestAnonDecryptWorksForOtherKey()
        {
            var verkey = await Crypto.CreateKeyAsync(wallet, MY1_IDENTITY_KEY_JSON);
            var encryptedMessage = await Crypto.AnonCryptAsync(VERKEY_TRUSTEE, MESSAGE);

            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                Crypto.AnonDecryptAsync(wallet, verkey, encryptedMessage)
           );
        }

        [TestMethod]
        public async Task TestAnonDecryptWorksForUnknownKey()
        {
            var encryptedMessage = await Crypto.AnonCryptAsync(VERKEY_MY1, MESSAGE);

            var ex = await Assert.ThrowsExceptionAsync<WalletValueNotFoundException>(() =>
                Crypto.AnonDecryptAsync(wallet, VERKEY_MY1, encryptedMessage)
           );
        }
       
    }
}
