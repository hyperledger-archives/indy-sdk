using Hyperledger.Indy.SignusApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System;
using System.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.SignusTests
{
    [TestClass]
    public class DecryptTest : IndyIntegrationTestWithPoolAndSingleWallet
    {
        private string _trusteeDid;
        private string _trusteeVerkey;
        private string _myDid;
        private string _myVerkey;
        private byte[] _encryptedMessage = (byte[]) (Array) new sbyte[] { -105, 30, 89, 75, 76, 28, -59, -45, 105, -46, 20, 124, -85, -13, 109, 29, -88, -82, -8, -6, -50, -84, -53, -48, -49, 56, 124, 114, 82, 126, 74, 99, -72, -78, -117, 96, 60, 119, 50, -40, 121, 21, 57, -68, 89 };
        private byte[] _nonce = (byte[])(Array) new sbyte[] { -14, 102, -41, -57, 1, 4, 75, -46, -91, 87, 14, 41, -39, 48, 42, -126, -121, 84, -58, 59, -27, 51, -32, -23 };

        [TestInitialize]
        public async Task CreateWalletWithDid()
        {           
            var trusteeNym = await Signus.CreateAndStoreMyDidAsync(wallet, TRUSTEE_IDENTITY_JSON);
            _trusteeDid = trusteeNym.Did;
            _trusteeVerkey = trusteeNym.VerKey;

            var myNym = await Signus.CreateAndStoreMyDidAsync(wallet, MY1_IDENTITY_JSON);
            _myDid = myNym.Did;
            _myVerkey = myNym.VerKey;

            var identityJson = string.Format(IDENTITY_JSON_TEMPLATE, _trusteeDid, _trusteeVerkey);
            await Signus.StoreTheirDidAsync(wallet, identityJson);
        }
               
        [TestMethod]
        public async Task TestDecryptWorks()
        {
            var decryptedMessage = await Signus.DecryptAsync(wallet, _myDid, _trusteeDid, _encryptedMessage, _nonce);
            Assert.IsTrue(MESSAGE.SequenceEqual(decryptedMessage));
        }

        [TestMethod]
        public async Task TestDecryptWorksForOtherCoder()
        {
            var identityJson = string.Format(IDENTITY_JSON_TEMPLATE, _myDid, _myVerkey);
            await Signus.StoreTheirDidAsync(wallet, identityJson);

            var encryptResult = await Signus.EncryptAsync(wallet, pool, _myDid, _myDid, MESSAGE);            

            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                Signus.DecryptAsync(wallet, _myDid, _trusteeDid, encryptResult.EncryptedMsg, encryptResult.Nonce)
            );;
        }

        [TestMethod]
        public async Task TestDecryptWorksForNonceNotCorrespondMessage()
        {
            var nonce = (byte[])(Array)new sbyte[] { 46, 33, -4, 67, 1, 44, 57, -46, -91, 87, 14, 41, -39, 48, 42, -126, -121, 84, -58, 59, -27, 51, -32, -23 };
                        
            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
               Signus.DecryptAsync(wallet, _myDid, _trusteeDid, _encryptedMessage, nonce)
            );
        }

        [TestMethod]
        public async Task TestDecryptWorksForUnknownMyDid()
        {
            var ex = await Assert.ThrowsExceptionAsync<WalletValueNotFoundException>(() =>
               Signus.DecryptAsync(wallet, "unknowDid", _trusteeDid, _encryptedMessage, _nonce)
           );
        }
    }
}
