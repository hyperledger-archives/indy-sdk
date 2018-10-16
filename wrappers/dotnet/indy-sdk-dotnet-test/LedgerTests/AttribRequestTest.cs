using Hyperledger.Indy.DidApi;
using Hyperledger.Indy.LedgerApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.LedgerTests
{
    [TestClass]
    public class AttribRequestTest : IndyIntegrationTestWithPoolAndSingleWallet
    {
        private string endpoint = "{\"endpoint\":{\"ha\":\"127.0.0.1:5555\"}}";
        private string hash = "83d907821df1c87db829e96569a11f6fc2e7880acba5e43d07ab786959e13bd3";
        private string enc = "aa3f41f619aa7e5e6b6d0de555e05331787f9bf9aa672b94b57ab65b9b66c3ea960b18a98e3834b1fc6cebf49f463b81fd6e3181";


        [TestMethod]
        public async Task TestBuildAttribRequestWorksForRawData()
        {
            string expectedResult = string.Format("\"identifier\":\"{0}\"," +
                    "\"operation\":{{" +
                    "\"type\":\"100\"," +
                    "\"dest\":\"{1}\"," +
                    "\"raw\":\"{2}\"" +
                    "}}", DID_TRUSTEE, DID_TRUSTEE, endpoint);

            string attribRequest = await Ledger.BuildAttribRequestAsync(DID_TRUSTEE, DID_TRUSTEE, null, endpoint, null);

            Assert.IsTrue(attribRequest.Replace("\\", "").Contains(expectedResult));
        }

        [TestMethod]
        public async Task TestBuildAttribRequestWorksForHashValue()
        {
            string expectedResult = string.Format("\"identifier\":\"{0}\"," +
                    "\"operation\":{{" +
                    "\"type\":\"100\"," +
                    "\"dest\":\"{1}\"," +
                    "\"hash\":\"{2}\"" +
                    "}}", DID_TRUSTEE, DID_TRUSTEE, hash);

            string attribRequest = await Ledger.BuildAttribRequestAsync(DID_TRUSTEE, DID_TRUSTEE, hash, null, null);

            Assert.IsTrue(attribRequest.Replace("\\", "").Contains(expectedResult));
        }

        [TestMethod]
        public async Task TestBuildAttribRequestWorksForEncValue()
        {
            string expectedResult = string.Format("\"identifier\":\"{0}\"," +
                    "\"operation\":{{" +
                    "\"type\":\"100\"," +
                    "\"dest\":\"{1}\"," +
                    "\"enc\":\"{2}\"" +
                    "}}", DID_TRUSTEE, DID_TRUSTEE, enc);

            string attribRequest = await Ledger.BuildAttribRequestAsync(DID_TRUSTEE, DID_TRUSTEE, null, null, enc);

            Assert.IsTrue(attribRequest.Replace("\\", "").Contains(expectedResult));
        }

        [TestMethod]
        public async Task TestBuildAttribRequestWorksForMissedAttribute()
        {
            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                Ledger.BuildAttribRequestAsync(DID_TRUSTEE, DID_TRUSTEE, null, null, null)
            );
        }

        [TestMethod]
        public async Task TestBuildGetAttribRequestWorksForRawValue()
        {
            string raw = "endpoint";

            string expectedResult = string.Format("\"identifier\":\"{0}\"," +
                    "\"operation\":{{" +
                    "\"type\":\"104\"," +
                    "\"dest\":\"{1}\"," +
                    "\"raw\":\"{2}\"" +
                    "}}", DID_TRUSTEE, DID_TRUSTEE, raw);

            string attribRequest = await Ledger.BuildGetAttribRequestAsync(DID_TRUSTEE, DID_TRUSTEE, raw, null, null);

            Assert.IsTrue(attribRequest.Contains(expectedResult));
        }

        [TestMethod]
        public async Task TestBuildGetAttribRequestWorksForHashValue()
        {
            string raw = "endpoint";

            string expectedResult = string.Format("\"identifier\":\"{0}\"," +
                    "\"operation\":{{" +
                    "\"type\":\"104\"," +
                    "\"dest\":\"{1}\"," +
                    "\"hash\":\"{2}\"" +
                    "}}", DID_TRUSTEE, DID_TRUSTEE, hash);

            string attribRequest = await Ledger.BuildGetAttribRequestAsync(DID_TRUSTEE, DID_TRUSTEE, null, hash, null);

            Assert.IsTrue(attribRequest.Contains(expectedResult));
        }

        [TestMethod]
        public async Task TestBuildGetAttribRequestWorksForEncValue()
        {
            string raw = "endpoint";

            string expectedResult = string.Format("\"identifier\":\"{0}\"," +
                    "\"operation\":{{" +
                    "\"type\":\"104\"," +
                    "\"dest\":\"{1}\"," +
                    "\"enc\":\"{2}\"" +
                    "}}", DID_TRUSTEE, DID_TRUSTEE, enc);

            string attribRequest = await Ledger.BuildGetAttribRequestAsync(DID_TRUSTEE, DID_TRUSTEE, null, null, enc);

            Assert.IsTrue(attribRequest.Contains(expectedResult));
        }

        [TestMethod]
        public async Task TestBuildGetAttribRequestWorksForDefaultSubmitter()
        {
            await Ledger.BuildGetAttribRequestAsync(null, DID_TRUSTEE, "endpoint", null, null);
        }

        [TestMethod]
        public async Task TestSendAttribRequestWorksWithoutSignature()
        {
            var trusteeDidResult = await Did.CreateAndStoreMyDidAsync(wallet, TRUSTEE_IDENTITY_JSON);
            var trusteeDid = trusteeDidResult.Did;

            var attribRequest = await Ledger.BuildAttribRequestAsync(trusteeDid, trusteeDid, null, endpoint, null);
            var response = await Ledger.SubmitRequestAsync(pool, attribRequest);
            CheckResponseType(response, "REQNACK");
        }

        [TestMethod]
        public async Task TestAttribRequestWorksForRawValue()
        {
            var trusteeDidResult = await Did.CreateAndStoreMyDidAsync(wallet, TRUSTEE_IDENTITY_JSON);
            var trusteeDid = trusteeDidResult.Did;

            var myDidResult = await Did.CreateAndStoreMyDidAsync(wallet, "{}");
            var myDid = myDidResult.Did;
            var myVerkey = myDidResult.VerKey;

            var nymRequest = await Ledger.BuildNymRequestAsync(trusteeDid, myDid, myVerkey, null, null);
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, trusteeDid, nymRequest);

            var attribRequest = await Ledger.BuildAttribRequestAsync(myDid, myDid, null, endpoint, null);
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, myDid, attribRequest);

            var getAttribRequest = await Ledger.BuildGetAttribRequestAsync(myDid, myDid, "endpoint", null, null);
            var getAttribResponse = await PoolUtils.EnsurePreviousRequestAppliedAsync(pool, getAttribRequest, response => {
                var getAttribResponseObject = JObject.Parse(response);
                return endpoint == getAttribResponseObject["result"]["data"].ToString();
            });

            Assert.IsNotNull(getAttribResponse);
        }

        [TestMethod]
        public async Task TestAttribRequestWorksForHashValue()
        {
            var trusteeDidResult = await Did.CreateAndStoreMyDidAsync(wallet, TRUSTEE_IDENTITY_JSON);
            var trusteeDid = trusteeDidResult.Did;

            var myDidResult = await Did.CreateAndStoreMyDidAsync(wallet, "{}");
            var myDid = myDidResult.Did;
            var myVerkey = myDidResult.VerKey;

            var nymRequest = await Ledger.BuildNymRequestAsync(trusteeDid, myDid, myVerkey, null, null);
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, trusteeDid, nymRequest);

            var attribRequest = await Ledger.BuildAttribRequestAsync(myDid, myDid, hash, null, null);
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, myDid, attribRequest);

            var getAttribRequest = await Ledger.BuildGetAttribRequestAsync(myDid, myDid, null, hash, null);
            var getAttribResponse = await PoolUtils.EnsurePreviousRequestAppliedAsync(pool, getAttribRequest, response => {
                var getAttribResponseObject = JObject.Parse(response);
                return hash == getAttribResponseObject["result"]["data"].ToString();
            });

            Assert.IsNotNull(getAttribResponse);
        }

        [TestMethod]
        public async Task TestAttribRequestWorksForEncValue()
        {
            var trusteeDidResult = await Did.CreateAndStoreMyDidAsync(wallet, TRUSTEE_IDENTITY_JSON);
            var trusteeDid = trusteeDidResult.Did;

            var myDidResult = await Did.CreateAndStoreMyDidAsync(wallet, "{}");
            var myDid = myDidResult.Did;
            var myVerkey = myDidResult.VerKey;

            var nymRequest = await Ledger.BuildNymRequestAsync(trusteeDid, myDid, myVerkey, null, null);
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, trusteeDid, nymRequest);

            var attribRequest = await Ledger.BuildAttribRequestAsync(myDid, myDid, null, null, enc);
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, myDid, attribRequest);

            var getAttribRequest = await Ledger.BuildGetAttribRequestAsync(myDid, myDid, null, null, enc);
            var getAttribResponse = await PoolUtils.EnsurePreviousRequestAppliedAsync(pool, getAttribRequest, response => {
                var getAttribResponseObject = JObject.Parse(response);
                return enc == getAttribResponseObject["result"]["data"].ToString();
            });

            Assert.IsNotNull(getAttribResponse);
        }


        [TestMethod]
        public async Task TestBuildAttribRequestWorksForInvalidIdentifier()
        {
            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                Ledger.BuildAttribRequestAsync("invalid_base58_identifier", DID_TRUSTEE, null, endpoint, null)
            );
        }
    }
}
