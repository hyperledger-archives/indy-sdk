using Hyperledger.Indy.PoolApi;
using Hyperledger.Indy.SignusApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.SignusTests
{
    [TestClass]
    public class DecryptTest : IndyIntegrationTestBase
    {
        private Pool _pool;
        private Wallet _wallet;
        private string _trusteeDid;
        private string _trusteeVerkey;
        private string _myDid;
        private string _myVerkey;
        private string _walletName = "SignusWallet";
        private byte[] _msg = Encoding.UTF8.GetBytes("{\"reqId\":1496822211362017764}");
        private byte[] _encryptedMessage = (byte[]) (Array) new sbyte[] { -105, 30, 89, 75, 76, 28, -59, -45, 105, -46, 20, 124, -85, -13, 109, 29, -88, -82, -8, -6, -50, -84, -53, -48, -49, 56, 124, 114, 82, 126, 74, 99, -72, -78, -117, 96, 60, 119, 50, -40, 121, 21, 57, -68, 89 };
        private byte[] _nonce = (byte[])(Array) new sbyte[] { -14, 102, -41, -57, 1, 4, 75, -46, -91, 87, 14, 41, -39, 48, 42, -126, -121, 84, -58, 59, -27, 51, -32, -23 };
        private string _identityJsonTemplate = "{{\"did\":\"{0}\",\"verkey\":\"{1}\"}}";

        [TestInitialize]
        public async Task CreateWalletWithDid()
        {
            var poolName = PoolUtils.CreatePoolLedgerConfig();

            _pool = await Pool.OpenPoolLedgerAsync(poolName, "{}");

            await Wallet.CreateWalletAsync(poolName, _walletName, "default", null, null);
            _wallet = await Wallet.OpenWalletAsync(_walletName, null, null);

            var trusteeJson = "{\"seed\":\"000000000000000000000000Trustee1\"}";

            var trusteeNym = await Signus.CreateAndStoreMyDidAsync(_wallet, trusteeJson);
            _trusteeDid = trusteeNym.Did;
            _trusteeVerkey = trusteeNym.VerKey;

            var otherDid = "{\"seed\":\"00000000000000000000000000000My1\"}";

            var myNym = await Signus.CreateAndStoreMyDidAsync(_wallet, otherDid);
            _myDid = myNym.Did;
            _myVerkey = myNym.VerKey;
        }

        [TestCleanup]
        public async Task DeleteWallet()
        {
            if(_pool != null)
                await _pool.CloseAsync();

            if (_wallet != null)
                await _wallet.CloseAsync();

            await Wallet.DeleteWalletAsync(_walletName, null);            
        }

        [TestMethod]
        public async Task TestDecryptWorks()
        {
            var identityJson = string.Format(_identityJsonTemplate, _trusteeDid, _trusteeVerkey);
            await Signus.StoreTheirDidAsync(_wallet, identityJson);

            var decryptedMessage = await Signus.DecryptAsync(_wallet, _myDid, _trusteeDid, _encryptedMessage, _nonce);
            Assert.IsTrue(_msg.SequenceEqual(decryptedMessage));

        }

        [TestMethod]
        public async Task TestDecryptWorksForOtherCoder()
        {
            var identityJson = string.Format(_identityJsonTemplate, _trusteeDid, _trusteeVerkey);
            await Signus.StoreTheirDidAsync(_wallet, identityJson);

            identityJson = string.Format(_identityJsonTemplate, _myDid, _myVerkey);
            await Signus.StoreTheirDidAsync(_wallet, identityJson);

            var encryptResult = await Signus.EncryptAsync(_wallet, _pool, _myDid, _myDid, _msg);            

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Signus.DecryptAsync(_wallet, _myDid, _trusteeDid, encryptResult.EncryptedMsg, encryptResult.Nonce)
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestDecryptWorksForNonceNotCorrespondMessage()
        {
            var identityJson = string.Format(_identityJsonTemplate, _trusteeDid, _trusteeVerkey);
            await Signus.StoreTheirDidAsync(_wallet, identityJson);

            var nonce = (byte[])(Array)new sbyte[] { 46, 33, -4, 67, 1, 44, 57, -46, -91, 87, 14, 41, -39, 48, 42, -126, -121, 84, -58, 59, -27, 51, -32, -23 };
                        
            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
               Signus.DecryptAsync(_wallet, _myDid, _trusteeDid, _encryptedMessage, nonce)
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestDecryptWorksForUnknownMyDid()
        {
            var identityJson = string.Format(_identityJsonTemplate, _trusteeDid, _trusteeVerkey);
            await Signus.StoreTheirDidAsync(_wallet, identityJson);

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
               Signus.DecryptAsync(_wallet, "unknowDid", _trusteeDid, _encryptedMessage, _nonce)
           );

            Assert.AreEqual(ErrorCode.WalletNotFoundError, ex.ErrorCode);
        }
    }
}
