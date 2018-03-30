using Hyperledger.Indy.LedgerApi;
using Hyperledger.Indy.DidApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.LedgerTests
{
    [TestClass]
    public class AttribRequestTest : IndyIntegrationTestWithPoolAndSingleWallet
    {        
        private const string _identifier = "Th7MpTaRZVRYnPiabds81Y";
        private const string _dest = "FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4";
        private const string _endpoint = "{\"endpoint\":{\"ha\":\"127.0.0.1:5555\"}}";
               
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
            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                Ledger.BuildAttribRequestAsync(_identifier, _dest, null, null, null)
            );
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

            string attribRequest = await Ledger.BuildGetAttribRequestAsync(_identifier, _dest, raw, string.Empty, string.Empty);

            Assert.IsTrue(attribRequest.Contains(expectedResult));
        }

        [TestMethod]
        public async Task TestSendAttribRequestWorksWithoutSignature()
        {
            var trusteeDidResult = await Did.CreateAndStoreMyDidAsync(wallet, TRUSTEE_IDENTITY_JSON);
            var trusteeDid = trusteeDidResult.Did;

            var attribRequest = await Ledger.BuildAttribRequestAsync(trusteeDid, trusteeDid, null, _endpoint, null);

            var ex = await Assert.ThrowsExceptionAsync<InvalidLedgerTransactionException>(() =>
                Ledger.SubmitRequestAsync(pool, attribRequest)
            );
        }

        [TestMethod]
        public async Task  TestAttribRequestWorks()
        {
            var trusteeDidResult = await Did.CreateAndStoreMyDidAsync(wallet, TRUSTEE_IDENTITY_JSON);
            var trusteeDid = trusteeDidResult.Did;

            var myDidResult = await Did.CreateAndStoreMyDidAsync(wallet, "{}");
            var myDid = myDidResult.Did;
            var myVerkey = myDidResult.VerKey;

            var nymRequest = await Ledger.BuildNymRequestAsync(trusteeDid, myDid, myVerkey, null, null);
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, trusteeDid, nymRequest);

            var attribRequest = await Ledger.BuildAttribRequestAsync(myDid, myDid, null, _endpoint, null);
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, myDid, attribRequest);

            var getAttribRequest = await Ledger.BuildGetAttribRequestAsync(myDid, myDid, "endpoint", string.Empty, string.Empty);
            var getAttribResponse = await Ledger.SubmitRequestAsync(pool, getAttribRequest);

            var jsonObject = JObject.Parse(getAttribResponse);

            Assert.AreEqual(_endpoint, jsonObject["result"]["data"]);
        }

        [TestMethod]
        public async Task TestBuildAttribRequestWorksForInvalidIdentifier()
        {
            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                Ledger.BuildAttribRequestAsync("invalid_base58_identifier", _dest, null, _endpoint, null)
            );
        }
    }
}
