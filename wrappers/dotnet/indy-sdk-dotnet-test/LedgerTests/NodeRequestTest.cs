using Hyperledger.Indy.DidApi;
using Hyperledger.Indy.LedgerApi;
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
                "\"blskey\":\"4N8aUNHSgjQVgkpm8nhNEfDf6txHznoYREg9kirmJrkivgL4oSEimFF6nsQ6M41QvhM2Z33nves5vfSn9n1UwNFJBYtWVnHYMATn76vLuL3zU88KyeAYcHfsih3He6UHcXDxcaecHVz6jhCYz1P2UZn2bDVruL5wXpehgBfBaLKm3Ba\"," +
                "\"blskey_pop\":\"RahHYiCvoNCtPTrVtP7nMC5eTYrsUA8WjXbdhNc8debh1agE9bGiJxWBXYNFbnJXoXhWFMvyqhqhRoq737YQemH5ik9oL7R4NTTCz2LEZhkgLJzB3QRQqJyBNyv7acbdHrAT8nQ9UkLbaVL9NBpnWXBTw4LEMePaSHEw66RzPNdAX1\"" +
            "}";


        private string _stewardDidJson = string.Format("{{\"seed\":\"{0}\"}}", "000000000000000000000000Steward1");
                
        [TestMethod]
        public async Task TestBuildNodeRequestWorks()
        {
            var expectedResult = string.Format("\"identifier\":\"{0}\"," +
                    "\"operation\":{{" +
                    "\"type\":\"0\"," +
                    "\"dest\":\"{1}\"," +
                    "\"data\":{2}" +
                    "}}", DID, _dest, _data);

            var nodeRequest = await Ledger.BuildNodeRequestAsync(DID, _dest, _data);

            Assert.IsTrue(nodeRequest.Replace("\\", "").Contains(expectedResult));
        }

        [TestMethod]
        public async Task TestSendNodeRequestWorksWithoutSignature()
        {
            var didResult = await Did.CreateAndStoreMyDidAsync(wallet, _stewardDidJson);
            var did = didResult.Did;

            var nodeRequest = await Ledger.BuildNodeRequestAsync(did, did, _data);
            var response = await Ledger.SubmitRequestAsync(pool, nodeRequest);
            CheckResponseType(response, "REQNACK");
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
                    "\"blskey\":\"4N8aUNHSgjQVgkpm8nhNEfDf6txHznoYREg9kirmJrkivgL4oSEimFF6nsQ6M41QvhM2Z33nves5vfSn9n1UwNFJBYtWVnHYMATn76vLuL3zU88KyeAYcHfsih3He6UHcXDxcaecHVz6jhCYz1P2UZn2bDVruL5wXpehgBfBaLKm3Ba\"," +
                    "\"blskey_pop\":\"RahHYiCvoNCtPTrVtP7nMC5eTYrsUA8WjXbdhNc8debh1agE9bGiJxWBXYNFbnJXoXhWFMvyqhqhRoq737YQemH5ik9oL7R4NTTCz2LEZhkgLJzB3QRQqJyBNyv7acbdHrAT8nQ9UkLbaVL9NBpnWXBTw4LEMePaSHEw66RzPNdAX1\"" +
                "}";

            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                Ledger.BuildNodeRequestAsync(DID, _dest, data)
            );
        }

        [TestMethod]
        public async Task TestBuildNodeRequestWorksForMissedField()
        {
            var data = "{}";

            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                Ledger.BuildNodeRequestAsync(DID, _dest, data)
            );
        }

        [TestMethod]
        public async Task TestSendNodeRequestWorksForWrongRole()
        {
            var didResult = await Did.CreateAndStoreMyDidAsync(wallet, TRUSTEE_IDENTITY_JSON);
            var did = didResult.Did;

            var nodeRequest = await Ledger.BuildNodeRequestAsync(did, did, _data);

            var response = await Ledger.SignAndSubmitRequestAsync(pool, wallet, did, nodeRequest);
            CheckResponseType(response, "REJECT");
            
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
