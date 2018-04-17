using Hyperledger.Indy.LedgerApi;
using Hyperledger.Indy.DidApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.LedgerTests
{
    [TestClass]
    public class NodeRequestTest : IndyIntegrationTestWithPoolAndSingleWallet
    {
        private const string _dest = "A5iWQVT3k8Zo9nXj4otmeqaUziPQPCiDqcydXkAJBk1Y";
        private const string _data = "{\"node_ip\":\"10.0.0.100\"," +
                "\"node_port\":910," +
                "\"client_ip\":\"10.0.0.100\"," +
                "\"client_port\":911," +
                "\"alias\":\"some\"," +
                "\"services\":[\"VALIDATOR\"]," +
                "\"blskey\":\"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW\"}";

        private string _stewardDidJson = string.Format("{{\"seed\":\"{0}\"}}", "000000000000000000000000Steward1");
                
        [TestMethod]
        public async Task TestBuildNodeRequestWorks()
        {
            var expectedResult = string.Format("\"identifier\":\"{0}\"," +
                    "\"operation\":{{" +
                    "\"type\":\"0\"," +
                    "\"dest\":\"{1}\"," +
                    "\"data\":{2}" +
                    "}}", DID1, _dest, _data);

            var nodeRequest = await Ledger.BuildNodeRequestAsync(DID1, _dest, _data);

            Assert.IsTrue(nodeRequest.Replace("\\", "").Contains(expectedResult));
        }

        [TestMethod]
        public async Task TestSendNodeRequestWorksWithoutSignature()
        {
            var didResult = await Did.CreateAndStoreMyDidAsync(wallet, _stewardDidJson);
            var did = didResult.Did;

            var nodeRequest = await Ledger.BuildNodeRequestAsync(did, did, _data);

            var ex = await Assert.ThrowsExceptionAsync<InvalidLedgerTransactionException>(() =>
                Ledger.SubmitRequestAsync(pool, nodeRequest)
            );
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

            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                Ledger.BuildNodeRequestAsync(DID1, _dest, data)
            );
        }

        [TestMethod]
        public async Task TestBuildNodeRequestWorksForMissedField()
        {
            var data = "{\"node_ip\":\"10.0.0.100\"," +
                    "\"node_port\":910," +
                    "\"client_ip\":\"10.0.0.100\"," +
                    "\"client_port\":911," +
                    "\"services\":[\"VALIDATOR\"]}";

            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                Ledger.BuildNodeRequestAsync(DID1, _dest, data)
            );
        }

        [TestMethod]
        public async Task TestSendNodeRequestWorksForWrongRole()
        {
            var didResult = await Did.CreateAndStoreMyDidAsync(wallet, TRUSTEE_IDENTITY_JSON);
            var did = didResult.Did;

            var data = "{\"node_ip\":\"10.0.0.100\"," +
                 "\"node_port\":910," +
                 "\"client_ip\":\"10.0.0.100\"," +
                 "\"client_port\":911," +
                 "\"alias\":\"some\"," +
                 "\"services\":[\"VALIDATOR\"]," +
                    "\"blskey\":\"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW\"}";

            var nodeRequest = await Ledger.BuildNodeRequestAsync(did, did, data);

            var ex = await Assert.ThrowsExceptionAsync<InvalidLedgerTransactionException>(() =>
                Ledger.SignAndSubmitRequestAsync(pool, wallet, did, nodeRequest)
            );
        }

        [TestMethod]
        [Ignore]
        public async Task TestSendNodeRequestWorksForNewSteward()
        {
            var trusteeDidResult = await Did.CreateAndStoreMyDidAsync(wallet, TRUSTEE_IDENTITY_JSON);
            var trusteeDid = trusteeDidResult.Did;

            var myDidResult = await Did.CreateAndStoreMyDidAsync(wallet, "{}");
            var myDid = myDidResult.Did;
            var myVerkey = myDidResult.VerKey;

            var nymRequest = await Ledger.BuildNymRequestAsync(trusteeDid, myDid, myVerkey, null, "STEWARD");
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, trusteeDid, nymRequest);


            var nodeRequest = await Ledger.BuildNodeRequestAsync(myDid, _dest, _data);
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, myDid, nodeRequest);
        }
    }
}
