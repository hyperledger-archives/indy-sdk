using Hyperledger.Indy.DidApi;
using Hyperledger.Indy.LedgerApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.LedgerTests
{
    [TestClass]
    public class PoolUpgradeRequestTest : IndyIntegrationTestWithPoolAndSingleWallet
    {
        [TestMethod]
        public async Task TestBuildPoolUpgradeRequestWorksForStartAction()
        {
            var expectedResult = string.Format("\"identifier\":\"{0}\"," +
                "\"operation\":{{\"type\":\"109\"," +
                "\"name\":\"upgrade-java\"," +
                "\"version\":\"2.0.0\"," +
                "\"action\":\"start\"," +
                "\"sha256\":\"f284b\"," +
                "\"schedule\":{{}}," +
                "\"reinstall\":false," +
                "\"force\":false}}", DID);

            var request = await Ledger.BuildPoolUpgradeRequestAsync(DID, "upgrade-java", "2.0.0", "start", "f284b", -1,
                    "{}", null, false, false, null);

            Assert.IsTrue(request.Contains(expectedResult));
        }

        [TestMethod]
        public async Task TestBuildPoolUpgradeRequestWorksForPackage()
        {
            var expectedResult = string.Format("\"identifier\":\"{0}\"," +
                "\"operation\":{{\"type\":\"109\"," +
                "\"name\":\"upgrade-java\"," +
                "\"version\":\"2.0.0\"," +
                "\"action\":\"start\"," +
                "\"sha256\":\"f284b\"," +
                "\"schedule\":{{}}," +
                "\"reinstall\":false," +
                "\"force\":false,"+
                "\"package\":\"some_package\"}}", DID);

            var request = await Ledger.BuildPoolUpgradeRequestAsync(DID, "upgrade-java", "2.0.0", "start", "f284b", -1,
                    "{}", null, false, false, "some_package");

            Assert.IsTrue(request.Contains(expectedResult));
        }

        [TestMethod]
        public async Task testBuildPoolUpgradeRequestWorksForCancelAction()
        {
            var expectedResult = string.Format("\"identifier\":\"{0}\"," +
                "\"operation\":{{\"type\":\"109\"," +
                "\"name\":\"upgrade-java\"," +
                "\"version\":\"2.0.0\"," +
                "\"action\":\"cancel\"," +
                "\"sha256\":\"f284b\"," +
                "\"schedule\":{{}}," +
                "\"reinstall\":false," +
                "\"force\":false}}", DID);

            var request = await Ledger.BuildPoolUpgradeRequestAsync(DID, "upgrade-java", "2.0.0", "cancel", "f284b", -1,
                    "{}", null, false, false, null);

            Assert.IsTrue(request.Contains(expectedResult));
        }

        [TestMethod]
        public async Task TestPoolUpgradeRequestWorks()
        {
            var nextYear = DateTime.Now.Year + 1;

            var didResult = await Did.CreateAndStoreMyDidAsync(wallet, TRUSTEE_IDENTITY_JSON);
            var did = didResult.Did;

            //start
            var schedule = string.Format("{{\"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv\":\"{0}-01-25T12:49:05.258870+00:00\",\n" +
                            "                   \"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb\":\"{0}-01-25T13:49:05.258870+00:00\",\n" +
                            "                   \"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya\":\"{0}-01-25T14:49:05.258870+00:00\",\n" +
                            "                   \"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA\":\"{0}-01-25T15:49:05.258870+00:00\"}}",
                    nextYear);
            var request = await Ledger.BuildPoolUpgradeRequestAsync(did, "upgrade-java", "2.0.0", "start",
                    "f284bdc3c1c9e24a494e285cb387c69510f28de51c15bb93179d9c7f28705398", -1, schedule, null, false, false, null);
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, did, request);

            //cancel
            request = await Ledger.BuildPoolUpgradeRequestAsync(did, "upgrade-java", "2.0.0", "cancel",
                    "ac3eb2cc3ac9e24a494e285cb387c69510f28de51c15bb93179d9c7f28705398", -1, null, null, false, false, null);
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, did, request);
        }
    }
}
