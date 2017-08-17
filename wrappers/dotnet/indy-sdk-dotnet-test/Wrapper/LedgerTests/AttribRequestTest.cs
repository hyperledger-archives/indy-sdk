using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Test.Wrapper.LedgerTests
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
        public void OpenPool()
        {
            string poolName = PoolUtils.CreatePoolLedgerConfig();
            _pool = Pool.OpenPoolLedgerAsync(poolName, null).Result;

            Wallet.CreateWalletAsync(poolName, _walletName, "default", null, null).Wait();
            _wallet = Wallet.OpenWalletAsync(_walletName, null, null).Result;
        }

        [TestCleanup]
        public void ClosePool()
        {
            if(_pool != null)
                _pool.CloseAsync().Wait();

            if(_wallet != null)
            _wallet.CloseAsync().Wait();

            Wallet.DeleteWalletAsync(_walletName, null).Wait();
        }

        [TestMethod]
        public void TestBuildAttribRequestWorksForRawData()
        {
            string expectedResult = string.Format("\"identifier\":\"{0}\"," +
                    "\"operation\":{{" +
                    "\"type\":\"100\"," +
                    "\"dest\":\"{1}\"," +
                    "\"raw\":\"{2}\"" +
                    "}}", _identifier, _dest, _endpoint);

            string attribRequest = Ledger.BuildAttribRequestAsync(_identifier, _dest, null, _endpoint, null).Result;

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
        public void TestBuildGetAttribRequestWorks()
        {
            string raw = "endpoint";

            string expectedResult = string.Format("\"identifier\":\"{0}\"," +
                    "\"operation\":{{" +
                    "\"type\":\"104\"," +
                    "\"dest\":\"{1}\"," +
                    "\"raw\":\"{2}\"" +
                    "}}", _identifier, _dest, raw);

            string attribRequest = Ledger.BuildGetAttribRequestAsync(_identifier, _dest, raw).Result;

            Assert.IsTrue(attribRequest.Contains(expectedResult));
        }

        [TestMethod]
        public async Task TestSendAttribRequestWorksWithoutSignature()
        {
            var json = "{\"seed\":\"000000000000000000000000Trustee1\"}";

            var trusteeDidResult = Signus.CreateAndStoreMyDidAsync(_wallet, json).Result;
            var trusteeDid = trusteeDidResult.Did;

            var attribRequest = Ledger.BuildAttribRequestAsync(trusteeDid, trusteeDid, null, _endpoint, null).Result;

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Ledger.SubmitRequestAsync(_pool, attribRequest)
            );

            Assert.AreEqual(ErrorCode.LedgerInvalidTransaction, ex.ErrorCode);
        }

        [TestMethod]
        public void TestAttribRequestWorks()
        {
            var trusteeJson = "{\"seed\":\"000000000000000000000000Trustee1\"}";

            var trusteeDidResult = Signus.CreateAndStoreMyDidAsync(_wallet, trusteeJson).Result;
            var trusteeDid = trusteeDidResult.Did;

            var myDidResult = Signus.CreateAndStoreMyDidAsync(_wallet, "{}").Result;
            var myDid = myDidResult.Did;
            var myVerkey = myDidResult.VerKey;

            var nymRequest = Ledger.BuildNymRequestAsync(trusteeDid, myDid, myVerkey, null, null).Result;
            Ledger.SignAndSubmitRequestAsync(_pool, _wallet, trusteeDid, nymRequest).Wait();

            var attribRequest = Ledger.BuildAttribRequestAsync(myDid, myDid, null, _endpoint, null).Result;
            Ledger.SignAndSubmitRequestAsync(_pool, _wallet, myDid, attribRequest).Wait();

            var getAttribRequest = Ledger.BuildGetAttribRequestAsync(myDid, myDid, "endpoint").Result;
            var getAttribResponse = Ledger.SubmitRequestAsync(_pool, getAttribRequest).Result;

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
