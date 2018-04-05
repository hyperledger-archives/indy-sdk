using Hyperledger.Indy.CryptoApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System;
using System.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.CryptoTests
{
    [TestClass]
    public class AuthDecryptTest : IndyIntegrationTestWithSingleWallet
    {
        [TestMethod]
        public async Task TestAuthDecryptWorks()
        {
            var senderVk = await Crypto.CreateKeyAsync(wallet, MY1_IDENTITY_KEY_JSON);
            var recipientVk = await Crypto.CreateKeyAsync(wallet, string.Format("{{\"seed\":\"{0}\"}}", MY2_SEED));

            var encrypted = await Crypto.AuthCryptAsync(wallet, senderVk, recipientVk, MESSAGE);

            var decryptedMessage = await Crypto.AuthDecryptAsync(wallet, recipientVk, encrypted);

            Assert.IsTrue(MESSAGE.SequenceEqual(decryptedMessage.MessageData));
            Assert.AreEqual(senderVk, decryptedMessage.TheirVk);
        }

        [TestMethod]
        public async Task TestAuthDecryptWorksForUnknownMyKey()
        {
            var ex = await Assert.ThrowsExceptionAsync<WalletValueNotFoundException>(() =>
                Crypto.AuthDecryptAsync(wallet, VERKEY_MY1, ENCRYPTED_MESSAGE)
           );
        }

        [TestMethod]
        public async Task TestAuthDecryptWorksForOtherCoder()
        {
            var myVk = await Crypto.CreateKeyAsync(wallet, MY1_IDENTITY_KEY_JSON);

            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                Crypto.AuthDecryptAsync(wallet, myVk, ENCRYPTED_MESSAGE)
           );
        }

        [TestMethod]
        public async Task TestAuthDecryptWorksForNonceNotCorrespondMessage()
        {
            var nonce = (byte[])(Array)new sbyte[] { 46, 33, -4, 67, 1, 44, 57, -46, -91, 87, 14, 41, -39, 48, 42, -126, -121, 84, -58, 59, -27, 51, -32, -23};
            var myVk = await Crypto.CreateKeyAsync(wallet, MY1_IDENTITY_KEY_JSON);

            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                Crypto.AuthDecryptAsync(wallet, myVk, ENCRYPTED_MESSAGE)
           );
        }
    }
}
