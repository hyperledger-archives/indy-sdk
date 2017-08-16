using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Text;
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

            var msg = Encoding.UTF8.GetBytes("{\"reqId\":1496822211362017764}");
            var signatureBytes = new byte[] { 20, 191, 100, 213, 101, 12, 197, 198, 203, 49, 89, 220, 205, 192, 224, 221, 97, 77, 220, 190, 90, 60, 142, 23, 16, 240, 189, 129, 45, 148, 245, 8, 102, 95, 95, 249, 100, 89, 41, 227, 213, 25, 100, 1, 232, 188, 245, 235, 186, 21, 52, 176, 236, 11, 99, 70, 155, 159, 89, 215, 197, 239, 138, 5 };

            var valid = await Signus.VerifySignatureAsync(_wallet, _pool, _trusteeDid, msg, signatureBytes);
            Assert.IsTrue(valid);
        }

        [TestMethod]
        public async Task TestVerifyWorksForGetVerkeyFromLedger()
        {
            CreateNewNymWithDidInLedger();
            await Signus.StoreTheirDidAsync(_wallet, string.Format("{{\"did\":\"{0}\"}}", _newDid));

            var msgBytes = Encoding.UTF8.GetBytes("{\"reqId\":1496822211362017764}");
            var signatureBytes = new byte[] { 169, 215, 8, 225, 7, 107, 110, 9, 193, 162, 202, 214, 162, 66, 238, 211, 63, 209, 12, 196, 8, 211, 55, 27, 120, 94, 204, 147, 53, 104, 103, 61, 60, 249, 237, 127, 103, 46, 220, 223, 10, 95, 75, 53, 245, 210, 241, 151, 191, 41, 48, 30, 9, 16, 78, 252, 157, 206, 210, 145, 125, 133, 109, 11 };

            var valid = await Signus.VerifySignatureAsync(_wallet, _pool, _newDid, msgBytes, signatureBytes);
            Assert.IsTrue(valid);
        }

        [TestMethod]
        public async Task TestVerifyWorksForGetNymFromLedger()
        {
            CreateNewNymWithDidInLedger();

            var msgBytes = Encoding.UTF8.GetBytes("{\"reqId\":1496822211362017764}");
            var signatureBytes = new byte[] { 169, 215, 8, 225, 7, 107, 110, 9, 193, 162, 202, 214, 162, 66, 238, 211, 63, 209, 12, 196, 8, 211, 55, 27, 120, 94, 204, 147, 53, 104, 103, 61, 60, 249, 237, 127, 103, 46, 220, 223, 10, 95, 75, 53, 245, 210, 241, 151, 191, 41, 48, 30, 9, 16, 78, 252, 157, 206, 210, 145, 125, 133, 109, 11 };

            var valid = await Signus.VerifySignatureAsync(_wallet, _pool, _newDid, msgBytes, signatureBytes);
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

            var msgBytes = Encoding.UTF8.GetBytes("{\"reqId\":1496822211362017764}");
            var signatureBytes = await Signus.SignAsync(_wallet, _trusteeDid, msgBytes);

            var valid = await Signus.VerifySignatureAsync(_wallet, _pool, stewardDid, msgBytes, signatureBytes);
            Assert.IsFalse(valid);
        }
    }
}
