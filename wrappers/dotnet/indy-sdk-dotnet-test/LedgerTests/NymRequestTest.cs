using Hyperledger.Indy.DidApi;
using Hyperledger.Indy.LedgerApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.LedgerTests
{
    [TestClass]
    public class NymRequestTest : IndyIntegrationTestWithPoolAndSingleWallet
    {
        private const string _dest = "FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4";
        private const string _role = "STEWARD";
        private const string _alias = "some_alias";       

        [TestMethod]
        public async Task TestBuildNymRequestWorksForOnlyRequiredFields()
        {
            var expectedResult = string.Format("\"identifier\":\"{0}\",\"operation\":{{\"dest\":\"{1}\",\"type\":\"1\"}}", DID, _dest);

            var nymRequest = await Ledger.BuildNymRequestAsync(DID, _dest, null, null, null);

            Assert.IsTrue(nymRequest.Contains(expectedResult));
        }

        [TestMethod]
        public async Task TestBuildNymRequestWorksForEmptyRole()
        {
            var expectedResult = string.Format("\"identifier\":\"{0}\",\"operation\":{{\"dest\":\"{1}\",\"role\":null,\"type\":\"1\"}}", DID, _dest);

            var nymRequest = await Ledger.BuildNymRequestAsync(DID, _dest, null, null, string.Empty);
            Assert.IsTrue(nymRequest.Contains(expectedResult));
        } 

        [TestMethod]
        public async Task TestBuildNymRequestWorksForOnlyOptionalFields()
        {
            var expectedResult = string.Format("\"identifier\":\"{0}\"," +
                    "\"operation\":{{" +
                    "\"alias\":\"{1}\"," +
                    "\"dest\":\"{2}\"," +
                    "\"role\":\"2\"," + 
                    "\"type\":\"1\"," +                    
                    "\"verkey\":\"{3}\"" +
                    "}}", DID, _alias, _dest, VERKEY_TRUSTEE);

            var nymRequest = await Ledger.BuildNymRequestAsync(DID, _dest, VERKEY_TRUSTEE, _alias, _role);

            Assert.IsTrue(nymRequest.Contains(expectedResult));
        }

        [TestMethod]
        public async Task TestBuildGetNymRequestWorks()
        {
            var expectedResult = String.Format("\"identifier\":\"{0}\",\"operation\":{{\"type\":\"105\",\"dest\":\"{1}\"}}", DID, _dest);

            var nymRequest = await Ledger.BuildGetNymRequestAsync(DID, _dest);

            Assert.IsTrue(nymRequest.Contains(expectedResult));
        }

        [TestMethod]
        public async Task TestBuildGetNymRequestWorksForDefaultSubmitter()
        {
             await Ledger.BuildGetNymRequestAsync(null, _dest);
        }

        [TestMethod]
        public async Task TestNymRequestWorksWithoutSignature()
        {
            var didResult = await Did.CreateAndStoreMyDidAsync(wallet, "{}");
            var did = didResult.Did;

            var nymRequest = await Ledger.BuildNymRequestAsync(did, did, null, null, null);
            var response = await Ledger.SubmitRequestAsync(pool, nymRequest);

            CheckResponseType(response, "REQNACK");
        }

        [TestMethod]
        public async Task TestSendNymRequestsWorksForOnlyRequiredFields()
        {
            var trusteeDidResult = await Did.CreateAndStoreMyDidAsync(wallet, TRUSTEE_IDENTITY_JSON);
            var trusteeDid = trusteeDidResult.Did;

            var myDidResult = await Did.CreateAndStoreMyDidAsync(wallet, "{}");
            var myDid = myDidResult.Did;

            var nymRequest = await Ledger.BuildNymRequestAsync(trusteeDid, myDid, null, null, null);
            var nymResponse = await Ledger.SignAndSubmitRequestAsync(pool, wallet, trusteeDid, nymRequest);

            Assert.IsNotNull(nymResponse);
        }

        [TestMethod]
        public async Task TestSendNymRequestsWorksForOptionalFields()
        {
            var trusteeDidResult = await Did.CreateAndStoreMyDidAsync(wallet, TRUSTEE_IDENTITY_JSON);
            var trusteeDid = trusteeDidResult.Did;

            var myDidResult = await Did.CreateAndStoreMyDidAsync(wallet, "{}");
            var myDid = myDidResult.Did;
            var myVerKey = myDidResult.VerKey;

            var nymRequest = await Ledger.BuildNymRequestAsync(trusteeDid, myDid, myVerKey, _alias, _role);
            var nymResponse = await Ledger.SignAndSubmitRequestAsync(pool, wallet, trusteeDid, nymRequest);

            Assert.IsNotNull(nymResponse);
        }

        [TestMethod]
        public async Task TestGetNymRequestWorks()
        {
            var didResult = await Did.CreateAndStoreMyDidAsync(wallet, TRUSTEE_IDENTITY_JSON);
            var did = didResult.Did;

            var getNymRequest = await Ledger.BuildGetNymRequestAsync(did, did);
            var getNymResponse = await Ledger.SubmitRequestAsync(pool, getNymRequest);

            var getNymResponseObj = JObject.Parse(getNymResponse);

            Assert.AreEqual(did, (string)getNymResponseObj["result"]["dest"]);
        }

        [TestMethod]
        public async Task TestSendNymRequestsWorksForWrongSignerRole()
        {
            var trusteeDidResult = await Did.CreateAndStoreMyDidAsync(wallet, TRUSTEE_IDENTITY_JSON);
            var trusteeDid = trusteeDidResult.Did;

            var myDidResult = await Did.CreateAndStoreMyDidAsync(wallet, "{}");
            var myDid = myDidResult.Did;

            var nymRequest = await Ledger.BuildNymRequestAsync(trusteeDid, myDid, null, null, null);
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, trusteeDid, nymRequest);

            var myDidResult2 = await Did.CreateAndStoreMyDidAsync(wallet, "{}");
            var myDid2 = myDidResult2.Did;

            var nymRequest2 = await Ledger.BuildNymRequestAsync(myDid, myDid2, null, null, null);
            var response = await Ledger.SignAndSubmitRequestAsync(pool, wallet, myDid, nymRequest2);
            CheckResponseType(response, "REQNACK");
        }

        [TestMethod]
        public async Task TestSendNymRequestsWorksForUnknownSigner()
        {
            var trusteeDidJson = "{\"seed\":\"000000000000000000000000Trustee9\"}";
            var trusteeDidResult = await Did.CreateAndStoreMyDidAsync(wallet, trusteeDidJson);
            var trusteeDid = trusteeDidResult.Did;

            var myDidResult = await Did.CreateAndStoreMyDidAsync(wallet, "{}");
            var myDid = myDidResult.Did;

            var nymRequest = await Ledger.BuildNymRequestAsync(trusteeDid, myDid, null, null, null);
            var response = await Ledger.SignAndSubmitRequestAsync(pool, wallet, myDid, nymRequest);
            CheckResponseType(response, "REQNACK");
        }

        [TestMethod]
        public async Task TestNymRequestsWorks()
        {
            var trusteeDidResult = await Did.CreateAndStoreMyDidAsync(wallet, TRUSTEE_IDENTITY_JSON);
            var trusteeDid = trusteeDidResult.Did;

            var myDidResult = await Did.CreateAndStoreMyDidAsync(wallet, "{}");
            var myDid = myDidResult.Did;
            var myVerKey = myDidResult.VerKey;

            var nymRequest = await Ledger.BuildNymRequestAsync(trusteeDid, myDid, myVerKey, null, null);
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, trusteeDid, nymRequest);

            var getNymRequest = await Ledger.BuildGetNymRequestAsync(myDid, myDid);
            var getNymResponse = PoolUtils.EnsurePreviousRequestAppliedAsync(pool, getNymRequest, response => { return CompareResponseType(response, "REPLY"); });
            Assert.IsNotNull(getNymResponse);
        }

        [TestMethod]
        public async Task TestSendNymRequestsWorksForWrongRole()
        {
            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                Ledger.BuildNymRequestAsync(DID, _dest, null, null, "WRONG_ROLE")
            );
        }
    }
}
