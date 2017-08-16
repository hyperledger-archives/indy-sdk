using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Test.Wrapper.LedgerTests
{
    [TestClass]
    public class RequestTest : IndyIntegrationTestBase
    {
        private Pool _pool;
        private Wallet _wallet;
        private string _walletName = "ledgerWallet";

        [TestInitialize]
        public void OpenPool()
        {
            var poolName = PoolUtils.CreatePoolLedgerConfig();
            _pool = Pool.OpenPoolLedgerAsync(poolName, null).Result;

            Wallet.CreateWalletAsync(poolName, _walletName, "default", null, null).Wait();
            _wallet = Wallet.OpenWalletAsync(_walletName, null, null).Result;

        }

        [TestCleanup]
        public void ClosePool()
        {
            if (_pool != null)
                _pool.CloseAsync().Wait();

            if (_wallet != null)
                _wallet.CloseAsync().Wait();

            Wallet.DeleteWalletAsync(_walletName, null).Wait();;
        }

        [TestMethod]
        public void TestSubmitRequestWorks()
        {
            var request = "{\"reqId\":1491566332010860,\n" +
                 "          \"identifier\":\"Th7MpTaRZVRYnPiabds81Y\",\n" +
                 "          \"operation\":{\n" +
                 "             \"type\":\"105\",\n" +
                 "             \"dest\":\"Th7MpTaRZVRYnPiabds81Y\"\n" +
                 "          },\n" +
                 "          \"signature\":\"4o86XfkiJ4e2r3J6Ufoi17UU3W5Zi9sshV6FjBjkVw4sgEQFQov9dxqDEtLbAJAWffCWd5KfAk164QVo7mYwKkiV\"}";

            var response = Ledger.SubmitRequestAsync(_pool, request).Result;

            var responseObject = JObject.Parse(response);

            Assert.AreEqual("REPLY", (string)responseObject["op"]);
            Assert.AreEqual("105", (string)responseObject["result"]["type"]);
            Assert.AreEqual(1491566332010860L, (long)responseObject["result"]["reqId"]);
            Assert.AreEqual("{\"dest\":\"Th7MpTaRZVRYnPiabds81Y\",\"identifier\":\"V4SGRU86Z58d6TV7PBUe6f\",\"role\":\"2\",\"verkey\":\"~7TYfekw4GUagBnBVCqPjiC\"}", (string)responseObject["result"]["data"]);
            Assert.AreEqual("Th7MpTaRZVRYnPiabds81Y", (string)responseObject["result"]["identifier"]);
            Assert.AreEqual("Th7MpTaRZVRYnPiabds81Y", (string)responseObject["result"]["dest"]);
        }

        [TestMethod]
        public void TestSignAndSubmitRequestWorks()
        {
            var trusteeDidJson = "{\"seed\":\"000000000000000000000000Trustee1\"}";
            var trusteeDidResult = Signus.CreateAndStoreMyDidAsync(_wallet, trusteeDidJson).Result;
            var trusteeDid = trusteeDidResult.Did;

            var myDidResult = Signus.CreateAndStoreMyDidAsync(_wallet, "{}").Result;
            var myDid = myDidResult.Did;

            var nymRequest = Ledger.BuildNymRequestAsync(trusteeDid, myDid, null, null, null).Result;
            var nymResponse = Ledger.SignAndSubmitRequestAsync(_pool, _wallet, trusteeDid, nymRequest).Result;
            Assert.IsNotNull(nymResponse);
        }
        [TestMethod]
        public async Task TestSignAndSubmitRequestWorksForNotFoundSigner()
        {
            var trusteeDidJson = "{\"seed\":\"00000000000000000000UnknowSigner\"}";

            var trusteeDidResult = Signus.CreateAndStoreMyDidAsync(_wallet, trusteeDidJson).Result;
            var signerDid = trusteeDidResult.Did;

            var myDidResult = Signus.CreateAndStoreMyDidAsync(_wallet, "{}").Result;
            var myDid = myDidResult.Did;

            var nymRequest = Ledger.BuildNymRequestAsync(signerDid, myDid, null, null, null).Result;            

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
               Ledger.SignAndSubmitRequestAsync(_pool, _wallet, signerDid, nymRequest)
            );

            Assert.AreEqual(ErrorCode.LedgerInvalidTransaction, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestSignAndSubmitRequestWorksForIncompatibleWalletAndPool()
        {
            var walletName = "incompatibleWallet";

            Wallet.CreateWalletAsync("otherPoolName", walletName, "default", null, null).Wait();
            var wallet = Wallet.OpenWalletAsync(walletName, null, null).Result;

            var trusteeDidJson = "{\"seed\":\"000000000000000000000000Trustee1\"}";
            var trusteeDidResult = Signus.CreateAndStoreMyDidAsync(wallet, trusteeDidJson).Result;
            var trusteeDid = trusteeDidResult.Did;

            var myDidResult = Signus.CreateAndStoreMyDidAsync(wallet, "{}").Result;
            var myDid = myDidResult.Did;

            var nymRequest = Ledger.BuildNymRequestAsync(trusteeDid, myDid, null, null, null).Result;            
            
            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Ledger.SignAndSubmitRequestAsync(_pool, wallet, trusteeDid, nymRequest)
            );

            Assert.AreEqual(ErrorCode.WalletIncompatiblePoolError, ex.ErrorCode);
        }
    }
}
