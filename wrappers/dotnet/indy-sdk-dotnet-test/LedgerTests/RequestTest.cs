using Hyperledger.Indy.LedgerApi;
using Hyperledger.Indy.PoolApi;
using Hyperledger.Indy.SignusApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.LedgerTests
{
    [TestClass]
    public class RequestTest : IndyIntegrationTestBase
    {
        private Pool _pool;
        private Wallet _wallet;
        private string _walletName = "ledgerWallet";

        [TestInitialize]
        public async Task OpenPool()
        {
            var poolName = PoolUtils.CreatePoolLedgerConfig();
            _pool = await Pool.OpenPoolLedgerAsync(poolName, null);

            await Wallet.CreateWalletAsync(poolName, _walletName, "default", null, null);
            _wallet = await Wallet.OpenWalletAsync(_walletName, null, null);

        }

        [TestCleanup]
        public async Task ClosePool()
        {
            if (_pool != null)
                await _pool.CloseAsync();

            if (_wallet != null)
                await _wallet.CloseAsync();

            await Wallet.DeleteWalletAsync(_walletName, null);;
        }

        [TestMethod]
        public async Task TestSubmitRequestWorks()
        {
            var request = "{\"reqId\":1491566332010860,\n" +
                 "          \"identifier\":\"Th7MpTaRZVRYnPiabds81Y\",\n" +
                 "          \"operation\":{\n" +
                 "             \"type\":\"105\",\n" +
                 "             \"dest\":\"Th7MpTaRZVRYnPiabds81Y\"\n" +
                 "          },\n" +
                 "          \"signature\":\"4o86XfkiJ4e2r3J6Ufoi17UU3W5Zi9sshV6FjBjkVw4sgEQFQov9dxqDEtLbAJAWffCWd5KfAk164QVo7mYwKkiV\"}";

            var response = await Ledger.SubmitRequestAsync(_pool, request);

            var responseObject = JObject.Parse(response);

            Assert.AreEqual("REPLY", (string)responseObject["op"]);
            Assert.AreEqual("105", (string)responseObject["result"]["type"]);
            Assert.AreEqual(1491566332010860L, (long)responseObject["result"]["reqId"]);
            Assert.AreEqual("{\"dest\":\"Th7MpTaRZVRYnPiabds81Y\",\"identifier\":\"V4SGRU86Z58d6TV7PBUe6f\",\"role\":\"2\",\"verkey\":\"~7TYfekw4GUagBnBVCqPjiC\"}", (string)responseObject["result"]["data"]);
            Assert.AreEqual("Th7MpTaRZVRYnPiabds81Y", (string)responseObject["result"]["identifier"]);
            Assert.AreEqual("Th7MpTaRZVRYnPiabds81Y", (string)responseObject["result"]["dest"]);
        }

        [TestMethod]
        public async Task TestSignAndSubmitRequestWorks()
        {
            var trusteeDidJson = "{\"seed\":\"000000000000000000000000Trustee1\"}";
            var trusteeDidResult = await Signus.CreateAndStoreMyDidAsync(_wallet, trusteeDidJson);
            var trusteeDid = trusteeDidResult.Did;

            var myDidResult = await Signus.CreateAndStoreMyDidAsync(_wallet, "{}");
            var myDid = myDidResult.Did;

            var nymRequest = await Ledger.BuildNymRequestAsync(trusteeDid, myDid, null, null, null);
            var nymResponse = await Ledger.SignAndSubmitRequestAsync(_pool, _wallet, trusteeDid, nymRequest);
            Assert.IsNotNull(nymResponse);
        }
        [TestMethod]
        public async Task TestSignAndSubmitRequestWorksForNotFoundSigner()
        {
            var trusteeDidJson = "{\"seed\":\"00000000000000000000UnknowSigner\"}";

            var trusteeDidResult = await Signus.CreateAndStoreMyDidAsync(_wallet, trusteeDidJson);
            var signerDid = trusteeDidResult.Did;

            var myDidResult = await Signus.CreateAndStoreMyDidAsync(_wallet, "{}");
            var myDid = myDidResult.Did;

            var nymRequest = await Ledger.BuildNymRequestAsync(signerDid, myDid, null, null, null);            

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
               Ledger.SignAndSubmitRequestAsync(_pool, _wallet, signerDid, nymRequest)
            );

            Assert.AreEqual(ErrorCode.LedgerInvalidTransaction, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestSignAndSubmitRequestWorksForIncompatibleWalletAndPool()
        {
            var walletName = "incompatibleWallet";

            await Wallet.CreateWalletAsync("otherPoolName", walletName, "default", null, null);
            var wallet = await Wallet.OpenWalletAsync(walletName, null, null);

            var trusteeDidJson = "{\"seed\":\"000000000000000000000000Trustee1\"}";
            var trusteeDidResult = await Signus.CreateAndStoreMyDidAsync(wallet, trusteeDidJson);
            var trusteeDid = trusteeDidResult.Did;

            var myDidResult = await Signus.CreateAndStoreMyDidAsync(wallet, "{}");
            var myDid = myDidResult.Did;

            var nymRequest = await Ledger.BuildNymRequestAsync(trusteeDid, myDid, null, null, null);            
            
            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Ledger.SignAndSubmitRequestAsync(_pool, wallet, trusteeDid, nymRequest)
            );

            Assert.AreEqual(ErrorCode.WalletIncompatiblePoolError, ex.ErrorCode);
        }
    }
}
