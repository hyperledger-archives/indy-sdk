using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Test.Wrapper.LedgerTests
{
    [TestClass]
    public class NymRequestTest : IndyIntegrationTest
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
        public void TestBuildNymRequestWorksForOnlyRequiredFields()
        {
            var identifier = "Th7MpTaRZVRYnPiabds81Y";
            var dest = "FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4";

            var expectedResult = string.Format("\"identifier\":\"{0}\",\"operation\":{{\"type\":\"1\",\"dest\":\"{1}\"}}", identifier, dest);

            var nymRequest = Ledger.BuildNymRequestAsync(identifier, dest, null, null, null).Result;

            Assert.IsTrue(nymRequest.Contains(expectedResult));
        }

        [TestMethod]
        public void TestBuildNymRequestWorksForOnlyOptionalFields()
        {
            var identifier = "Th7MpTaRZVRYnPiabds81Y";
            var dest = "FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4";
            var verkey = "Anfh2rjAcxkE249DcdsaQl";
            var role = "STEWARD";
            var alias = "some_alias";

            var expectedResult = string.Format("\"identifier\":\"{0}\"," +
                    "\"operation\":{{" +
                    "\"type\":\"1\"," +
                    "\"dest\":\"{1}\"," +
                    "\"verkey\":\"{2}\"," +
                    "\"alias\":\"{3}\"," +
                    "\"role\":\"2\"" +
                    "}}", identifier, dest, verkey, alias);

            var nymRequest = Ledger.BuildNymRequestAsync(identifier, dest, verkey, alias, role).Result;

            Assert.IsTrue(nymRequest.Contains(expectedResult));
        }

        [TestMethod]
        public void TestBuildGetNymRequestWorks()
        {
            var identifier = "Th7MpTaRZVRYnPiabds81Y";
            var dest = "FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4";

            var expectedResult = String.Format("\"identifier\":\"{0}\",\"operation\":{{\"type\":\"105\",\"dest\":\"{1}\"}}", identifier, dest);

            var nymRequest = Ledger.BuildGetNymRequestAsync(identifier, dest).Result;

            Assert.IsTrue(nymRequest.Contains(expectedResult));
        }

        [TestMethod]
        public async Task TestNymRequestWorksWithoutSignature()
        {
            var didJson = "{\"seed\":\"00000000000000000000000000000My1\"}";

            var didResult = Signus.CreateAndStoreMyDidAsync(_wallet, didJson).Result;
            var did = didResult.Did;

            var nymRequest = Ledger.BuildNymRequestAsync(did, did, null, null, null).Result;

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Ledger.SubmitRequestAsync(_pool, nymRequest)
            );

            Assert.AreEqual(ErrorCode.LedgerInvalidTransaction, ex.ErrorCode);
        }

        [TestMethod]
        public void TestSendNymRequestsWorksForOnlyRequiredFields()
        {
            var trusteeDidJson = "{\"seed\":\"000000000000000000000000Trustee1\"}";
            var trusteeDidResult = Signus.CreateAndStoreMyDidAsync(_wallet, trusteeDidJson).Result;
            var trusteeDid = trusteeDidResult.Did;

            var myDidJson = "{\"seed\":\"00000000000000000000000000000My1\"}";
            var myDidResult = Signus.CreateAndStoreMyDidAsync(_wallet, myDidJson).Result;
            var myDid = myDidResult.Did;

            var nymRequest = Ledger.BuildNymRequestAsync(trusteeDid, myDid, null, null, null).Result;
            var nymResponse = Ledger.SignAndSubmitRequestAsync(_pool, _wallet, trusteeDid, nymRequest).Result;

            Assert.IsNotNull(nymResponse);
        }

        [TestMethod]
        public void TestSendNymRequestsWorksForOptionalFields()
        {
            var trusteeDidJson = "{\"seed\":\"000000000000000000000000Trustee1\"}";
            var trusteeDidResult = Signus.CreateAndStoreMyDidAsync(_wallet, trusteeDidJson).Result;
            var trusteeDid = trusteeDidResult.Did;

            var myDidJson = "{\"seed\":\"00000000000000000000000000000My1\"}";
            var myDidResult = Signus.CreateAndStoreMyDidAsync(_wallet, myDidJson).Result;
            var myDid = myDidResult.Did;
            var myVerKey = myDidResult.VerKey;
            var role = "STEWARD";
            var alias = "some_alias";

            var nymRequest = Ledger.BuildNymRequestAsync(trusteeDid, myDid, myVerKey, alias, role).Result;
            var nymResponse = Ledger.SignAndSubmitRequestAsync(_pool, _wallet, trusteeDid, nymRequest).Result;

            Assert.IsNotNull(nymResponse);
        }

        [TestMethod]
        public void TestGetNymRequestWorks()
        {
            var didJson = "{\"seed\":\"000000000000000000000000Trustee1\"}";
            var didResult = Signus.CreateAndStoreMyDidAsync(_wallet, didJson).Result;
            var did = didResult.Did;

            var getNymRequest = Ledger.BuildGetNymRequestAsync(did, did).Result;
            var getNymResponse = Ledger.SubmitRequestAsync(_pool, getNymRequest).Result;

            var getNymResponseObj = JObject.Parse(getNymResponse);

            Assert.AreEqual(did, (string)getNymResponseObj["result"]["dest"]);
        }

        [TestMethod]
        public async Task TestSendNymRequestsWorksForWrongSignerRole()
        {
            var trusteeDidJson = "{\"seed\":\"000000000000000000000000Trustee1\"}";
            var trusteeDidResult = Signus.CreateAndStoreMyDidAsync(_wallet, trusteeDidJson).Result;
            var trusteeDid = trusteeDidResult.Did;

            var myDidJson = "{}";
            var myDidResult = Signus.CreateAndStoreMyDidAsync(_wallet, myDidJson).Result;
            var myDid = myDidResult.Did;

            var nymRequest = Ledger.BuildNymRequestAsync(trusteeDid, myDid, null, null, null).Result;
            Ledger.SignAndSubmitRequestAsync(_pool, _wallet, trusteeDid, nymRequest).Wait();

            var myDidJson2 = "{}";
            var myDidResult2 = Signus.CreateAndStoreMyDidAsync(_wallet, myDidJson2).Result;
            var myDid2 = myDidResult2.Did;

            var nymRequest2 = Ledger.BuildNymRequestAsync(myDid, myDid2, null, null, null).Result;

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Ledger.SignAndSubmitRequestAsync(_pool, _wallet, myDid, nymRequest2)
            );

            Assert.AreEqual(ErrorCode.LedgerInvalidTransaction, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestSendNymRequestsWorksForUnknownSigner()
        {
            var trusteeDidJson = "{\"seed\":\"000000000000000000000000Trustee9\"}";
            var trusteeDidResult = Signus.CreateAndStoreMyDidAsync(_wallet, trusteeDidJson).Result;
            var trusteeDid = trusteeDidResult.Did;

            var myDidJson = "{}";
            var myDidResult = Signus.CreateAndStoreMyDidAsync(_wallet, myDidJson).Result;
            var myDid = myDidResult.Did;

            var nymRequest = Ledger.BuildNymRequestAsync(trusteeDid, myDid, null, null, null).Result;

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Ledger.SignAndSubmitRequestAsync(_pool, _wallet, trusteeDid, nymRequest)
            );

            Assert.AreEqual(ErrorCode.LedgerInvalidTransaction, ex.ErrorCode);
        }

        [TestMethod]
        public void TestNymRequestsWorks()
        {
            var trusteeDidJson = "{\"seed\":\"000000000000000000000000Trustee1\"}";
            var trusteeDidResult = Signus.CreateAndStoreMyDidAsync(_wallet, trusteeDidJson).Result;
            var trusteeDid = trusteeDidResult.Did;

            var myDidJson = "{\"seed\":\"00000000000000000000000000000My1\"}";
            var myDidResult = Signus.CreateAndStoreMyDidAsync(_wallet, myDidJson).Result;
            var myDid = myDidResult.Did;
            var myVerKey = myDidResult.VerKey;

            var nymRequest = Ledger.BuildNymRequestAsync(trusteeDid, myDid, myVerKey, null, null).Result;
            Ledger.SignAndSubmitRequestAsync(_pool, _wallet, trusteeDid, nymRequest).Wait();

            var getNymRequest = Ledger.BuildGetNymRequestAsync(myDid, myDid).Result;
            var getNymResponse = Ledger.SubmitRequestAsync(_pool, getNymRequest).Result;

            var getNymResponseObj = JObject.Parse(getNymResponse);

            Assert.AreEqual("REPLY", (string)getNymResponseObj["op"]);
            Assert.AreEqual("105", (string)getNymResponseObj["result"]["type"]);
            Assert.AreEqual(myDid, (string)getNymResponseObj["result"]["dest"]);
        }

        [TestMethod]
        public async Task TestSendNymRequestsWorksForWrongRole()
        {
            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Ledger.BuildNymRequestAsync("Th7MpTaRZVRYnPiabds81Y",
                "FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4", null, null, "WRONG_ROLE")
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }
    }
}
