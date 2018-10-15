using Hyperledger.Indy.CryptoApi;
using Hyperledger.Indy.DidApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.CryptoTests
{
    [TestClass]
    public class AuthCryptTest : IndyIntegrationTestWithSingleWallet
    {
        [TestMethod]
        public async Task TestAuthCryptWorksForCreatedKey()
        {         
            var myVk = await Crypto.CreateKeyAsync(wallet, MY1_IDENTITY_KEY_JSON);
            byte[] encryptedMsg = await Crypto.AuthCryptAsync(wallet, myVk, VERKEY_MY2, MESSAGE);
            Assert.IsNotNull(encryptedMsg);
        }

        [TestMethod]
        public async Task TestAuthCryptWorksForCreatedDid()
        {
            var result = await Did.CreateAndStoreMyDidAsync(wallet, MY1_IDENTITY_JSON);
            var myVk = result.VerKey;
            
            byte[] encryptedMsg = await Crypto.AuthCryptAsync(wallet, myVk, VERKEY_MY2, MESSAGE);
            Assert.IsNotNull(encryptedMsg);
        }

        [TestMethod]
        [Ignore] //This test is identical to TestAuthCryptWorksForCreatedDid?
        public async Task TestAuthCryptWorksForCreatedDidAsCid()
        {
            var result = await Did.CreateAndStoreMyDidAsync(wallet, MY1_IDENTITY_JSON);
            var myVk = result.VerKey;

            byte[] encryptedMsg = await Crypto.AuthCryptAsync(wallet, myVk, VERKEY_MY2, MESSAGE);
            Assert.IsNotNull(encryptedMsg);
        }

        [TestMethod]
        public async Task TestAuthCryptWorksForUnknownSenderVerkey()
        {
            var ex = await Assert.ThrowsExceptionAsync<WalletItemNotFoundException>(() =>
                 Crypto.AuthCryptAsync(wallet, VERKEY, VERKEY_MY2, MESSAGE)
           );
        }

        [TestMethod]
        public async Task TestAuthCryptWorksForInvalidTheirVk()
        {
            var myVk = await Crypto.CreateKeyAsync(wallet, MY1_IDENTITY_KEY_JSON);

            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                 Crypto.AuthCryptAsync(wallet, myVk, INVALID_VERKEY, MESSAGE)
           );
        }
    }
}
