using Hyperledger.Indy.LedgerApi;
using Hyperledger.Indy.PoolApi;
using Hyperledger.Indy.SignusApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.LedgerTests
{
    [TestClass]
    public class NymRequestTest : IndyIntegrationTestBase
    {
        private Pool _pool;
        private Wallet _wallet;
        private string _walletName = "ledgerWallet";
        private string _identifier = "Th7MpTaRZVRYnPiabds81Y";
        private string _dest = "FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4";


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
            if (_pool != null)
                await _pool.CloseAsync();

            if (_wallet != null)
                await _wallet.CloseAsync();

            await Wallet.DeleteWalletAsync(_walletName, null);
        }


        [TestMethod]
        public async Task TestBuildNymRequestWorksForOnlyRequiredFields()
        {
            var expectedResult = string.Format("\"identifier\":\"{0}\",\"operation\":{{\"dest\":\"{1}\",\"type\":\"1\"}}", _identifier, _dest);

            var nymRequest = await Ledger.BuildNymRequestAsync(_identifier, _dest, null, null, null);

            Assert.IsTrue(nymRequest.Contains(expectedResult));
        }

        [TestMethod]
        public async Task TestBuildNymRequestWorksForEmptyRole()
        {
            var expectedResult = string.Format("\"identifier\":\"{0}\",\"operation\":{{\"dest\":\"{1}\",\"role\":null,\"type\":\"1\"}}", _identifier, _dest);

            var nymRequest = await Ledger.BuildNymRequestAsync(_identifier, _dest, null, null, string.Empty);
            Assert.IsTrue(nymRequest.Contains(expectedResult));
        }
 

        [TestMethod]
        public async Task TestBuildNymRequestWorksForOnlyOptionalFields()
        {
            var verkey = "Anfh2rjAcxkE249DcdsaQl";
            var role = "STEWARD";
            var alias = "some_alias";

            var expectedResult = string.Format("\"identifier\":\"{0}\"," +
                    "\"operation\":{{" +
                    "\"alias\":\"{1}\"," +
                    "\"dest\":\"{2}\"," +
                    "\"role\":\"2\"," + 
                    "\"type\":\"1\"," +                    
                    "\"verkey\":\"{3}\"" +
                    "}}", _identifier, alias, _dest, verkey);

            var nymRequest = await Ledger.BuildNymRequestAsync(_identifier, _dest, verkey, alias, role);

            Assert.IsTrue(nymRequest.Contains(expectedResult));
        }

        [TestMethod]
        public async Task TestBuildGetNymRequestWorks()
        {
            var expectedResult = String.Format("\"identifier\":\"{0}\",\"operation\":{{\"type\":\"105\",\"dest\":\"{1}\"}}", _identifier, _dest);

            var nymRequest = await Ledger.BuildGetNymRequestAsync(_identifier, _dest);

            Assert.IsTrue(nymRequest.Contains(expectedResult));
        }

        [TestMethod]
        public async Task TestNymRequestWorksWithoutSignature()
        {
            var didResult = await Signus.CreateAndStoreMyDidAsync(_wallet, "{}");
            var did = didResult.Did;

            var nymRequest = await Ledger.BuildNymRequestAsync(did, did, null, null, null);

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Ledger.SubmitRequestAsync(_pool, nymRequest)
            );

            Assert.AreEqual(ErrorCode.LedgerInvalidTransaction, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestSendNymRequestsWorksForOnlyRequiredFields()
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
        public async Task TestSendNymRequestsWorksForOptionalFields()
        {
            var trusteeDidJson = "{\"seed\":\"000000000000000000000000Trustee1\"}";
            var trusteeDidResult = await Signus.CreateAndStoreMyDidAsync(_wallet, trusteeDidJson);
            var trusteeDid = trusteeDidResult.Did;

            var myDidResult = await Signus.CreateAndStoreMyDidAsync(_wallet, "{}");
            var myDid = myDidResult.Did;
            var myVerKey = myDidResult.VerKey;
            var role = "STEWARD";
            var alias = "some_alias";

            var nymRequest = await Ledger.BuildNymRequestAsync(trusteeDid, myDid, myVerKey, alias, role);
            var nymResponse = await Ledger.SignAndSubmitRequestAsync(_pool, _wallet, trusteeDid, nymRequest);

            Assert.IsNotNull(nymResponse);
        }

        [TestMethod]
        public async Task TestGetNymRequestWorks()
        {
            var didJson = "{\"seed\":\"000000000000000000000000Trustee1\"}";
            var didResult = await Signus.CreateAndStoreMyDidAsync(_wallet, didJson);
            var did = didResult.Did;

            var getNymRequest = await Ledger.BuildGetNymRequestAsync(did, did);
            var getNymResponse = await Ledger.SubmitRequestAsync(_pool, getNymRequest);

            var getNymResponseObj = JObject.Parse(getNymResponse);

            Assert.AreEqual(did, (string)getNymResponseObj["result"]["dest"]);
        }

        [TestMethod]
        public async Task TestSendNymRequestsWorksForWrongSignerRole()
        {
            var trusteeDidJson = "{\"seed\":\"000000000000000000000000Trustee1\"}";
            var trusteeDidResult = await Signus.CreateAndStoreMyDidAsync(_wallet, trusteeDidJson);
            var trusteeDid = trusteeDidResult.Did;

            var myDidJson = "{}";
            var myDidResult = await Signus.CreateAndStoreMyDidAsync(_wallet, myDidJson);
            var myDid = myDidResult.Did;

            var nymRequest = await Ledger.BuildNymRequestAsync(trusteeDid, myDid, null, null, null);
            await Ledger.SignAndSubmitRequestAsync(_pool, _wallet, trusteeDid, nymRequest);

            var myDidJson2 = "{}";
            var myDidResult2 = await Signus.CreateAndStoreMyDidAsync(_wallet, myDidJson2);
            var myDid2 = myDidResult2.Did;

            var nymRequest2 = await Ledger.BuildNymRequestAsync(myDid, myDid2, null, null, null);

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Ledger.SignAndSubmitRequestAsync(_pool, _wallet, myDid, nymRequest2)
            );

            Assert.AreEqual(ErrorCode.LedgerInvalidTransaction, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestSendNymRequestsWorksForUnknownSigner()
        {
            var trusteeDidJson = "{\"seed\":\"000000000000000000000000Trustee9\"}";
            var trusteeDidResult = await Signus.CreateAndStoreMyDidAsync(_wallet, trusteeDidJson);
            var trusteeDid = trusteeDidResult.Did;

            var myDidJson = "{}";
            var myDidResult = await Signus.CreateAndStoreMyDidAsync(_wallet, myDidJson);
            var myDid = myDidResult.Did;

            var nymRequest = await Ledger.BuildNymRequestAsync(trusteeDid, myDid, null, null, null);

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Ledger.SignAndSubmitRequestAsync(_pool, _wallet, trusteeDid, nymRequest)
            );

            Assert.AreEqual(ErrorCode.LedgerInvalidTransaction, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestNymRequestsWorks()
        {
            var trusteeDidJson = "{\"seed\":\"000000000000000000000000Trustee1\"}";
            var trusteeDidResult = await Signus.CreateAndStoreMyDidAsync(_wallet, trusteeDidJson);
            var trusteeDid = trusteeDidResult.Did;

            var myDidResult = await Signus.CreateAndStoreMyDidAsync(_wallet, "{}");
            var myDid = myDidResult.Did;
            var myVerKey = myDidResult.VerKey;

            var nymRequest = await Ledger.BuildNymRequestAsync(trusteeDid, myDid, myVerKey, null, null);
            await Ledger.SignAndSubmitRequestAsync(_pool, _wallet, trusteeDid, nymRequest);

            var getNymRequest = await Ledger.BuildGetNymRequestAsync(myDid, myDid);
            var getNymResponse = await Ledger.SubmitRequestAsync(_pool, getNymRequest);

            var getNymResponseObj = JObject.Parse(getNymResponse);

            Assert.AreEqual("REPLY", (string)getNymResponseObj["op"]);
            Assert.AreEqual("105", (string)getNymResponseObj["result"]["type"]);
            Assert.AreEqual(myDid, (string)getNymResponseObj["result"]["dest"]);
        }

        [TestMethod]
        public async Task TestSendNymRequestsWorksForWrongRole()
        {
            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Ledger.BuildNymRequestAsync(_identifier, _dest, null, null, "WRONG_ROLE")
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }
    }
}
