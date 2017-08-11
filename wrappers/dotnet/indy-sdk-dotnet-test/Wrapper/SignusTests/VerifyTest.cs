using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Test.Wrapper.SignusTests
{
    [TestClass]
    public class VerifyTest : IndyIntegrationTestBase
    {
        private Pool _pool;
        private Wallet _wallet;
        private string walletName = "signusWallet";
        private string _trusteeDid;
        private string _trusteeVerkey;
        private string _identityJson;
        private string _newDid;

        [TestInitialize]
        public void CreateWalletWithDid()
        {
            var poolName = PoolUtils.CreatePoolLedgerConfig();

            _pool = Pool.OpenPoolLedgerAsync(poolName, "{}").Result;

            Wallet.CreateWalletAsync(poolName, walletName, "default", null, null).Wait();
            _wallet = Wallet.OpenWalletAsync(walletName, null, null).Result;
            
            var json = "{\"seed\":\"000000000000000000000000Trustee1\",\"cid\":false}";

            var result = Signus.CreateAndStoreMyDidAsync(_wallet, json).Result;
            Assert.IsNotNull(result);

            _trusteeDid = result.Did;
            _trusteeVerkey = result.VerKey;
        }

        [TestCleanup]
        public void DeleteWallet()
        {
            _wallet.CloseAsync().Wait();
            Wallet.DeleteWalletAsync(walletName, null).Wait();
            _pool.CloseAsync().Wait();
        }

        private void CreateNewNymWithDidInLedger()
        {
            var json = "{\"seed\":\"00000000000000000000000000000My1\"}";

            var result = Signus.CreateAndStoreMyDidAsync(_wallet, json).Result;
            _newDid = result.Did;
            var newVerkey = result.VerKey;

            var nymRequest = Ledger.BuildNymRequestAsync(_trusteeDid, _newDid, newVerkey, null, null).Result;
            Ledger.SignAndSubmitRequestAsync(_pool, _wallet, _trusteeDid, nymRequest).Wait();
        }

        [TestMethod]
        public async Task TestVerifyWorksForVerkeyCachedInWallet()
        {
            _identityJson = string.Format("{{\"did\":\"{0}\",\"verkey\":\"{1}\"}}", _trusteeDid, _trusteeVerkey);
            await Signus.StoreTheirDidAsync(_wallet, _identityJson);

            var msg = "{\"reqId\":1496822211362017764}";
            var signature = "R4Rj68n4HZosQqEc3oMUbQh7MtG8tH7WmXE2Mok8trHJ67CrzyqahZn5ziJy4nebRtq6Qi6fVH9JkvVCM85XjFa";

            var valid = await Signus.VerifySignatureAsync(_wallet, _pool, _trusteeDid, msg, signature);
            Assert.IsTrue(valid);
        }

        [TestMethod]
        public async Task TestVerifyWorksForGetVerkeyFromLedger()
        {
            CreateNewNymWithDidInLedger();
            await Signus.StoreTheirDidAsync(_wallet, string.Format("{{\"did\":\"{0}\"}}", _newDid));

            var msg = "{\"reqId\":1496822211362017764}";
            var signature = "4Pwx83PGrDNPa1wonqLnQkzBEeFwMt8a8AKM3s86RMTW2ty6XV8Zk98Tg4UfYYXoEs3cCp4wUxGNvAfvurUDb24A";

            var valid = await Signus.VerifySignatureAsync(_wallet, _pool, _newDid, msg, signature);
            Assert.IsTrue(valid);
        }

        [TestMethod]
        public async Task TestVerifyWorksForGetNymFromLedger()
        {
            CreateNewNymWithDidInLedger();

            var msg = "{\"reqId\":1496822211362017764}";
            var signature = "4Pwx83PGrDNPa1wonqLnQkzBEeFwMt8a8AKM3s86RMTW2ty6XV8Zk98Tg4UfYYXoEs3cCp4wUxGNvAfvurUDb24A";

            var valid = await Signus.VerifySignatureAsync(_wallet, _pool, _newDid, msg, signature);
            Assert.IsTrue(valid);
        }
        
        [TestMethod]
        public async Task TestVerifyWorksForOtherSigner()
        {
            _identityJson = string.Format("{{\"did\":\"{0}\", \"verkey\":\"{1}\"}}", _trusteeDid, _trusteeVerkey);

            await Signus.StoreTheirDidAsync(_wallet, _identityJson);

            var createDidJson = "{\"seed\":\"000000000000000000000000Steward1\"}";

            var result = await Signus.CreateAndStoreMyDidAsync(_wallet, createDidJson);
            var stewardDid = result.Did;
            var stewardVerkey = result.VerKey;

            _identityJson = string.Format("{{\"did\":\"{0}\", \"verkey\":\"{1}\"}}", stewardDid, stewardVerkey);

            await Signus.StoreTheirDidAsync(_wallet, _identityJson);

            var msg = "{\"reqId\":1496822211362017764}";
            var signature = await Signus.SignAsync(_wallet, _trusteeDid, msg);

            var valid = await Signus.VerifySignatureAsync(_wallet, _pool, stewardDid, msg, signature);
            Assert.IsFalse(valid);
        }
    }
}
