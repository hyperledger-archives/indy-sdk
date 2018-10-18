using Hyperledger.Indy.CryptoApi;
using Hyperledger.Indy.DidApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.CryptoTests
{
    [TestClass]
    public class AuthDecryptTest : IndyIntegrationTestWithSingleWallet
    {
        [TestMethod]
        public async Task TestAuthDecryptWorks()
        {
            var theirVk = await Crypto.CreateKeyAsync(wallet, MY1_IDENTITY_KEY_JSON);
            var myVk = await Crypto.CreateKeyAsync(wallet, string.Format("{{\"seed\":\"{0}\"}}", MY2_SEED));

            var encryptedMsg = await Crypto.AuthCryptAsync(wallet, theirVk, myVk, MESSAGE);

            var decryptedMessage = await Crypto.AuthDecryptAsync(wallet, myVk, encryptedMsg);

            Assert.IsTrue(MESSAGE.SequenceEqual(decryptedMessage.MessageData));
            Assert.AreEqual(theirVk, decryptedMessage.TheirVk);
        }

        [TestMethod]
        public async Task TestAuthDecryptWorksForInvalidMessage()
        {
            var result = await Did.CreateAndStoreMyDidAsync(wallet, "{}");
            var recipientDid = result.Did;
            var myVk = result.VerKey;

            var identityJson = string.Format(IDENTITY_JSON_TEMPLATE, recipientDid, myVk);
            await Did.StoreTheirDidAsync(wallet, identityJson);

            var msgString = "[" + string.Join(",", ENCRYPTED_MESSAGE) + "]";

            var msg = string.Format("{{\"auth\":true,\"nonсe\":\"Th7MpTaRZVRYnPiabds81Y12\",\"sender\":\"{0}\",\"msg\":{1}}}", VERKEY, msgString);


            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                Crypto.AuthDecryptAsync(wallet, myVk, Encoding.UTF8.GetBytes(msg))
           );
        }

        [TestMethod]
        public async Task TestAuthDecryptWorksForUnknownTheirVk()
        {
            var theirVk = await Crypto.CreateKeyAsync(wallet, MY1_IDENTITY_KEY_JSON);

            var encryptedMsg = await Crypto.AuthCryptAsync(wallet, theirVk, VERKEY, MESSAGE);

            var ex = await Assert.ThrowsExceptionAsync<WalletItemNotFoundException>(() =>
               Crypto.AuthDecryptAsync(wallet, VERKEY, encryptedMsg)
          );
        }
    }
}
