using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Test.Wrapper.LedgerTests
{
    [TestClass]
    public class AttribRequestTest : IndyIntegrationTest
    {
        private Pool _pool;
        private Wallet _wallet;
        private string _walletName = "ledgerWallet";

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
            _pool.CloseAsync().Wait();
            _wallet.CloseAsync().Wait();
            Wallet.DeleteWalletAsync(_walletName, null).Wait();
        }

        [TestMethod]
        public void TestBuildAttribRequestWorksForRawData()
        {
            string identifier = "Th7MpTaRZVRYnPiabds81Y";
            string dest = "FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4";
            string raw = "{\"endpoint\":{\"ha\":\"127.0.0.1:5555\"}}";

            string expectedResult = string.Format("\"identifier\":\"{0}\"," +
                    "\"operation\":{{" +
                    "\"type\":\"100\"," +
                    "\"dest\":\"{1}\"," +
                    "\"raw\":\"{{\"endpoint\":{{\"ha\":\"127.0.0.1:5555\"}}}}\"" +
                    "}}", identifier, dest);

            string attribRequest = Ledger.BuildAttribRequestAsync(identifier, dest, null, raw, null).Result;

            Assert.IsTrue(attribRequest.Replace("\\", "").Contains(expectedResult));
        }

        [TestMethod]
        public async Task TestBuildAttribRequestWorksForMissedAttribute()
        {
            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Ledger.BuildAttribRequestAsync("Th7MpTaRZVRYnPiabds81Y",
                "FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4", null, null, null)
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

        [TestMethod]
        public void TestBuildGetAttribRequestWorks()
        {
            string identifier = "Th7MpTaRZVRYnPiabds81Y";
            string dest = "FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4";
            string raw = "endpoint";

            string expectedResult = string.Format("\"identifier\":\"{0}\"," +
                    "\"operation\":{{" +
                    "\"type\":\"104\"," +
                    "\"dest\":\"{1}\"," +
                    "\"raw\":\"{2}\"" +
                    "}}", identifier, dest, raw);

            string attribRequest = Ledger.BuildGetAttribRequestAsync(identifier, dest, raw).Result;

            Assert.IsTrue(attribRequest.Contains(expectedResult));
        }

        [TestMethod]
        public async Task TestSendAttribRequestWorksWithoutSignature()
        {
            var json = "{\"seed\":\"000000000000000000000000Trustee1\"}";

            var trusteeDidResult = Signus.CreateAndStoreMyDidAsync(_wallet, json).Result;
            var trusteeDid = trusteeDidResult.Did;

            var endpoint = "{\"endpoint\":{\"ha\":\"127.0.0.1:5555\"}}";
            var attribRequest = Ledger.BuildAttribRequestAsync(trusteeDid, trusteeDid, null, endpoint, null).Result;

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

            var myJson = "{\"seed\":\"00000000000000000000000000000My1\"}";

           
            var myDidResult = Signus.CreateAndStoreMyDidAsync(_wallet, myJson).Result;
            var myDid = myDidResult.Did;
            var myVerkey = myDidResult.VerKey;

            var nymRequest = Ledger.BuildNymRequestAsync(trusteeDid, myDid, myVerkey, null, null).Result;
            Ledger.SignAndSubmitRequestAsync(_pool, _wallet, trusteeDid, nymRequest).Wait();

            var endpoint = "{\"endpoint\":{\"ha\":\"127.0.0.1:5555\"}}";

            var attribRequest = Ledger.BuildAttribRequestAsync(myDid, myDid, null, endpoint, null).Result;
            Ledger.SignAndSubmitRequestAsync(_pool, _wallet, myDid, attribRequest).Wait();

            var getAttribRequest = Ledger.BuildGetAttribRequestAsync(myDid, myDid, "endpoint").Result;
            var getAttribResponse = Ledger.SubmitRequestAsync(_pool, getAttribRequest).Result;

            var jsonObject = JObject.Parse(getAttribResponse);

            Assert.AreEqual(endpoint, jsonObject["result"]["data"]);
        }

        [TestMethod]
        public async Task TestBuildAttribRequestWorksForInvalidIdentifier()
        {
            var endpoint = "{\"endpoint\":{\"ha\":\"127.0.0.1:5555\"}}";  

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Ledger.BuildAttribRequestAsync("invalid_base58_identifier",
                "FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4", null, endpoint, null)
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }
    }
}
