using Hyperledger.Indy.DidApi;
using Hyperledger.Indy.LedgerApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.LedgerTests
{
    [TestClass]
    public class MultiSignRequestTest : IndyIntegrationTestWithPoolAndSingleWallet
    {
        [TestMethod]
        public async Task TestMultiSignWorks()
        {
            var result = await Did.CreateAndStoreMyDidAsync(wallet, TRUSTEE_IDENTITY_JSON);
            var did1 = result.Did;

            result = await Did.CreateAndStoreMyDidAsync(wallet, MY1_IDENTITY_JSON);
            var did2 = result.Did;

            var msg = string.Format("{{\n" +
                    "                \"reqId\":1496822211362017764,\n" +
                    "                \"identifier\":\"{0}\",\n" +
                    "                \"operation\":{{\n" +
                    "                    \"type\":\"1\",\n" +
                    "                    \"dest\":\"VsKV7grR1BUE29mG2Fm2kX\",\n" +
                    "                    \"verkey\":\"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa\"\n" +
                    "                }}\n" +
                    "            }}", did1);

            var signedMessageJson = await Ledger.MultiSignRequestAsync(wallet, did1, msg);
            signedMessageJson = await Ledger.MultiSignRequestAsync(wallet, did2, signedMessageJson);

            var signedMessage = JObject.Parse(signedMessageJson);

            Assert.AreEqual("3YnLxoUd4utFLzeXUkeGefAqAdHUD7rBprpSx2CJeH7gRYnyjkgJi7tCnFgUiMo62k6M2AyUDtJrkUSgHfcq3vua",
                    signedMessage["signatures"][did1]);
            Assert.AreEqual("4EyvSFPoeQCJLziGVqjuMxrbuoWjAWUGPd6LdxeZuG9w3Bcbt7cSvhjrv8SX5e8mGf8jrf3K6xd9kEhXsQLqUg45",
                    signedMessage["signatures"][did2]);
        }

        [TestMethod]
        public async Task TestMultiSignWorksForUnknownDid()
        {
            var msg = "{\"reqId\":1496822211362017764}";
            var ex = await Assert.ThrowsExceptionAsync<WalletItemNotFoundException>(() =>
                Ledger.MultiSignRequestAsync(wallet, DID, msg)
            );
        }

        [TestMethod]
        public async Task TestMultiSignWorksForInvalidMessageFormat()
        {
            var result = await Did.CreateAndStoreMyDidAsync(wallet, TRUSTEE_IDENTITY_JSON);
            var did = result.Did;

            var msg = "\"reqId\":1496822211362017764";
            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                Ledger.MultiSignRequestAsync(wallet, did, msg)
            );
        }
    }
}
