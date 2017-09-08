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
    public class AttribRequestTest : IndyIntegrationTestBase
    {
        private Pool _pool;
        private Wallet _wallet;
        private string _walletName = "ledgerWallet";
        private string _identifier = "Th7MpTaRZVRYnPiabds81Y";
        private string _dest = "FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4";
        private string _endpoint = "{\"endpoint\":{\"ha\":\"127.0.0.1:5555\"}}";


        [TestInitialize]
        public async Task OpenPool()
        {
            string poolName = PoolUtils.CreatePoolLedgerConfig();
            _pool = await Pool.OpenPoolLedgerAsync(poolName, null);

            await Wallet.CreateWalletAsync(poolName, _walletName, "default", null, null);
            _wallet = await Wallet.OpenWalletAsync(_walletName, null, null);
        }

        [TestCleanup]
        public async Task ClosePool()
        {
            if(_pool != null)
                await _pool.CloseAsync();

            if(_wallet != null)
                await _wallet.CloseAsync();

            await Wallet.DeleteWalletAsync(_walletName, null);
        }

        [TestMethod]
        public async Task TestBuildAttribRequestWorksForRawData()
        {
            string expectedResult = string.Format("\"identifier\":\"{0}\"," +
                    "\"operation\":{{" +
                    "\"type\":\"100\"," +
                    "\"dest\":\"{1}\"," +
                    "\"raw\":\"{2}\"" +
                    "}}", _identifier, _dest, _endpoint);

            string attribRequest = await Ledger.BuildAttribRequestAsync(_identifier, _dest, null, _endpoint, null);

            Assert.IsTrue(attribRequest.Replace("\\", "").Contains(expectedResult));
        }

        [TestMethod]
        public async Task TestBuildAttribRequestWorksForMissedAttribute()
        {
            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Ledger.BuildAttribRequestAsync(_identifier, _dest, null, null, null)
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

        [TestMethod]
        public async Task  TestBuildGetAttribRequestWorks()
        {
            string raw = "endpoint";

            string expectedResult = string.Format("\"identifier\":\"{0}\"," +
                    "\"operation\":{{" +
                    "\"type\":\"104\"," +
                    "\"dest\":\"{1}\"," +
                    "\"raw\":\"{2}\"" +
                    "}}", _identifier, _dest, raw);

            string attribRequest = await Ledger.BuildGetAttribRequestAsync(_identifier, _dest, raw);

            Assert.IsTrue(attribRequest.Contains(expectedResult));
        }

        [TestMethod]
        public async Task TestSendAttribRequestWorksWithoutSignature()
        {
            var json = "{\"seed\":\"000000000000000000000000Trustee1\"}";

            var trusteeDidResult = await Signus.CreateAndStoreMyDidAsync(_wallet, json);
            var trusteeDid = trusteeDidResult.Did;

            var attribRequest = await Ledger.BuildAttribRequestAsync(trusteeDid, trusteeDid, null, _endpoint, null);

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Ledger.SubmitRequestAsync(_pool, attribRequest)
            );

            Assert.AreEqual(ErrorCode.LedgerInvalidTransaction, ex.ErrorCode);
        }

        [TestMethod]
        public async Task  TestAttribRequestWorks()
        {
            var trusteeJson = "{\"seed\":\"000000000000000000000000Trustee1\"}";

            var trusteeDidResult = await Signus.CreateAndStoreMyDidAsync(_wallet, trusteeJson);
            var trusteeDid = trusteeDidResult.Did;

            var myDidResult = await Signus.CreateAndStoreMyDidAsync(_wallet, "{}");
            var myDid = myDidResult.Did;
            var myVerkey = myDidResult.VerKey;

            var nymRequest = await Ledger.BuildNymRequestAsync(trusteeDid, myDid, myVerkey, null, null);
            await Ledger.SignAndSubmitRequestAsync(_pool, _wallet, trusteeDid, nymRequest);

            var attribRequest = await Ledger.BuildAttribRequestAsync(myDid, myDid, null, _endpoint, null);
            await Ledger.SignAndSubmitRequestAsync(_pool, _wallet, myDid, attribRequest);

            var getAttribRequest = await Ledger.BuildGetAttribRequestAsync(myDid, myDid, "endpoint");
            var getAttribResponse = await Ledger.SubmitRequestAsync(_pool, getAttribRequest);

            var jsonObject = JObject.Parse(getAttribResponse);

            Assert.AreEqual(_endpoint, jsonObject["result"]["data"]);
        }

        [TestMethod]
        public async Task TestBuildAttribRequestWorksForInvalidIdentifier()
        {
            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Ledger.BuildAttribRequestAsync("invalid_base58_identifier", _dest, null, _endpoint, null)
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }
    }
}
