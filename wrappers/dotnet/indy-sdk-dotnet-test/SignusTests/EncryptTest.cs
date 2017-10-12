using Hyperledger.Indy.LedgerApi;
using Hyperledger.Indy.PoolApi;
using Hyperledger.Indy.SignusApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Text;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.SignusTests
{
    [TestClass]
    public class EncryptTest : IndyIntegrationTestBase
    {
        private Pool _pool;
        private Wallet _wallet;
        private string _trusteeDid;
        private string _trusteeVerkey;
        private string _did;
        private string _verkey;
        private string _walletName = "SignusWallet";
        private byte[] _msg = Encoding.UTF8.GetBytes("{\"reqId\":1496822211362017764}");

        [TestInitialize]
        public async Task Before()
        {
            var poolName = PoolUtils.CreatePoolLedgerConfig();

            _pool = await Pool.OpenPoolLedgerAsync(poolName, "{}");

            await Wallet.CreateWalletAsync(poolName, _walletName, "default", null, null);
            _wallet = await Wallet.OpenWalletAsync(_walletName, null, null);

            var trusteeJson = "{\"seed\":\"000000000000000000000000Trustee1\"}";
            var result = await Signus.CreateAndStoreMyDidAsync(_wallet, trusteeJson);
            _trusteeDid = result.Did;
            _trusteeVerkey = result.VerKey;

            var otherDid = "{\"seed\":\"00000000000000000000000000000My1\"}";
            var nym = await Signus.CreateAndStoreMyDidAsync(_wallet, otherDid);
            _did = nym.Did;
            _verkey = nym.VerKey;

            var nymRequest = await Ledger.BuildNymRequestAsync(_trusteeDid, _did, _verkey, null, null);
            await Ledger.SignAndSubmitRequestAsync(_pool, _wallet, _trusteeDid, nymRequest);
        }

        [TestCleanup]
        public async Task After()
        {
            if (_pool != null)
                await _pool.CloseAsync();

            if (_wallet != null)
                await _wallet.CloseAsync();

            await Wallet.DeleteWalletAsync(_walletName, null);
        }

        [TestMethod]
        public async Task TestEncryptWorksForPkCachedInWallet()
        {
            var identityJson = string.Format("{{\"did\":\"{0}\",\"verkey\":\"{1}\"}}", _did, _verkey);
            await Signus.StoreTheirDidAsync(_wallet, identityJson);

            var encryptResult = await Signus.EncryptAsync(_wallet, _pool, _trusteeDid, _did, _msg);
            Assert.IsNotNull(encryptResult);
        }

        [TestMethod]
        public async Task TestEncryptWorksForGetPkFromLedger()
        {
            var identityJson = string.Format("{{\"did\":\"{0}\"}}", _did);
            await Signus.StoreTheirDidAsync(_wallet, identityJson);

            var encryptResult = await Signus.EncryptAsync(_wallet, _pool, _trusteeDid, _did, _msg);
            Assert.IsNotNull(encryptResult);
        }

        [TestMethod]
        public async Task TestEncryptWorksForGetNymFromLedger()
        {
            var encryptResult = await Signus.EncryptAsync(_wallet, _pool, _trusteeDid, _did, _msg);
            Assert.IsNotNull(encryptResult);
        }

        [TestMethod]
        public async Task TestEncryptWorksForUnknownMyDid()
        {
            var identityJson = string.Format("{{\"did\":\"{0}\",\"verkey\":\"{1}\"}}", _trusteeDid, _trusteeVerkey);
            await Signus.StoreTheirDidAsync(_wallet, identityJson);

            var ex = await Assert.ThrowsExceptionAsync<WalletValueNotFoundException>(() =>
                Signus.EncryptAsync(_wallet, _pool, "unknownDid", _trusteeDid, _msg)
            );
        }

        [TestMethod]
        public async Task TestEncryptWorksForNotFoundNym()
        {
            var nym = await Signus.CreateAndStoreMyDidAsync(_wallet, "{}");

            var ex = await Assert.ThrowsExceptionAsync<InvalidStateException>(() =>
               Signus.EncryptAsync(_wallet, _pool, _trusteeDid, nym.Did, _msg)
            );
        }
    }
}
