using Hyperledger.Indy.LedgerApi;
using Hyperledger.Indy.PoolApi;
using Hyperledger.Indy.SignusApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.LedgerTests
{
    [TestClass]
    public class NodeRequestTest : IndyIntegrationTestBase
    {
        private Pool _pool;
        private Wallet _wallet;
        private string _walletName = "ledgerWallet";
        private string _identifier = "Th7MpTaRZVRYnPiabds81Y";
        private string _dest = "Th7MpTaRZVRYnPiabds81Y";

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
        public async Task TestBuildNodeRequestWorks()
        {
            var data = "{\"node_ip\":\"10.0.0.100\"," +
                    "\"node_port\":910," +
                    "\"client_ip\":\"10.0.0.100\"," +
                    "\"client_port\":911," +
                    "\"alias\":\"some\"," +
                    "\"services\":[\"VALIDATOR\"]," +
                    "\"blskey\":\"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW\"}";

            var expectedResult = string.Format("\"identifier\":\"{0}\"," +
                    "\"operation\":{{" +
                    "\"type\":\"0\"," +
                    "\"dest\":\"{1}\"," +
                    "\"data\":{2}" +
                    "}}", _identifier, _dest, data);

            var nodeRequest = await Ledger.BuildNodeRequestAsync(_identifier, _dest, data);

            Assert.IsTrue(nodeRequest.Replace("\\", "").Contains(expectedResult));
        }

        [TestMethod]
        public async Task TestSendNodeRequestWorksWithoutSignature()
        {
            var didJson = "{\"seed\":\"000000000000000000000000Steward1\"}";
            var didResult = await Signus.CreateAndStoreMyDidAsync(_wallet, didJson);
            var did = didResult.Did;

            var data = "{\"node_ip\":\"10.0.0.100\"," +
                    "\"node_port\":910," +
                    "\"client_ip\":\"10.0.0.100\"," +
                    "\"client_port\":910," +
                    "\"alias\":\"some\"," +
                    "\"services\":[\"VALIDATOR\"]," +
                    "\"blskey\":\"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW\"}";

            var nodeRequest = await Ledger.BuildNodeRequestAsync(did, did, data);

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Ledger.SubmitRequestAsync(_pool, nodeRequest)
            );

            Assert.AreEqual(ErrorCode.LedgerInvalidTransaction, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestBuildNodeRequestWorksForWrongServiceType()
        {
            var data = "{\"node_ip\":\"10.0.0.100\"," +
                    "\"node_port\":910," +
                    "\"client_ip\":\"10.0.0.100\"," +
                    "\"client_port\":911," +
                    "\"alias\":\"some\"," +
                    "\"services\":[\"SERVICE\"]," +
                    "\"blskey\":\"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW\"}";

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Ledger.BuildNodeRequestAsync(_identifier, _dest, data)
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestBuildNodeRequestWorksForMissedField()
        {
            var data = "{\"node_ip\":\"10.0.0.100\"," +
                    "\"node_port\":910," +
                    "\"client_ip\":\"10.0.0.100\"," +
                    "\"client_port\":911," +
                    "\"services\":[\"VALIDATOR\"]}";

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Ledger.BuildNodeRequestAsync(_identifier, _dest, data)
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestSendNodeRequestWorksForWrongRole()
        {
            var didJson = "{\"seed\":\"000000000000000000000000Trustee1\"}";

            var didResult = await Signus.CreateAndStoreMyDidAsync(_wallet, didJson);
            var did = didResult.Did;

            var data = "{\"node_ip\":\"10.0.0.100\"," +
                 "\"node_port\":910," +
                 "\"client_ip\":\"10.0.0.100\"," +
                 "\"client_port\":911," +
                 "\"alias\":\"some\"," +
                 "\"services\":[\"VALIDATOR\"]," +
                    "\"blskey\":\"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW\"}";

            var nodeRequest = await Ledger.BuildNodeRequestAsync(did, did, data);

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Ledger.SignAndSubmitRequestAsync(_pool, _wallet, did, nodeRequest)
            );

            Assert.AreEqual(ErrorCode.LedgerInvalidTransaction, ex.ErrorCode);
        }

        [TestMethod]
        [Ignore]
        public async Task TestSendNodeRequestWorksForNewSteward()
        {
            var trusteeDidJson = "{\"seed\":\"000000000000000000000000Trustee1\"}";
            var trusteeDidResult = await Signus.CreateAndStoreMyDidAsync(_wallet, trusteeDidJson);
            var trusteeDid = trusteeDidResult.Did;

            var myDidJson = "{}";
            var myDidResult = await Signus.CreateAndStoreMyDidAsync(_wallet, myDidJson);
            var myDid = myDidResult.Did;
            var myVerkey = myDidResult.VerKey;

            var role = "STEWARD";

            var nymRequest = await Ledger.BuildNymRequestAsync(trusteeDid, myDid, myVerkey, null, role);
            await Ledger.SignAndSubmitRequestAsync(_pool, _wallet, trusteeDid, nymRequest);

            var data = "{\"node_ip\":\"10.0.0.100\"," +
                    "\"node_port\":910," +
                    "\"client_ip\":\"10.0.0.100\"," +
                    "\"client_port\":911," +
                    "\"alias\":\"some\"," +
                    "\"services\":[\"VALIDATOR\"]}";

            var dest = "A5iWQVT3k8Zo9nXj4otmeqaUziPQPCiDqcydXkAJBk1Y";

            var nodeRequest = await Ledger.BuildNodeRequestAsync(myDid, dest, data);
            await Ledger.SignAndSubmitRequestAsync(_pool, _wallet, myDid, nodeRequest);
        }


    }
}
