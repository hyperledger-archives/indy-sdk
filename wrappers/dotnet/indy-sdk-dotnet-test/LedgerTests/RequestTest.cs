using Hyperledger.Indy.LedgerApi;
using Hyperledger.Indy.DidApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.LedgerTests
{
    [TestClass]
    public class RequestTest : IndyIntegrationTestWithPoolAndSingleWallet
    {
        [TestMethod]
        public async Task TestSubmitRequestWorks()
        {
            var request = "{\"reqId\":1491566332010860,\n" +
                 "          \"identifier\":\"Th7MpTaRZVRYnPiabds81Y\",\n" +
                 "          \"operation\":{\n" +
                 "             \"type\":\"105\",\n" +
                 "             \"dest\":\"Th7MpTaRZVRYnPiabds81Y\"\n" +
                 "          },\n" +
                 "          \"signature\":\"4o86XfkiJ4e2r3J6Ufoi17UU3W5Zi9sshV6FjBjkVw4sgEQFQov9dxqDEtLbAJAWffCWd5KfAk164QVo7mYwKkiV\"}";

            var response = await Ledger.SubmitRequestAsync(pool, request);

            var responseObject = JObject.Parse(response);

            Assert.AreEqual("REPLY", (string)responseObject["op"]);
            Assert.AreEqual("105", (string)responseObject["result"]["type"]);
            Assert.AreEqual(1491566332010860L, (long)responseObject["result"]["reqId"]);
            Assert.AreEqual("{\"dest\":\"Th7MpTaRZVRYnPiabds81Y\",\"identifier\":\"V4SGRU86Z58d6TV7PBUe6f\",\"role\":\"2\",\"seqNo\":2,\"txnTime\":null,\"verkey\":\"~7TYfekw4GUagBnBVCqPjiC\"}", (string)responseObject["result"]["data"]);
            Assert.AreEqual("Th7MpTaRZVRYnPiabds81Y", (string)responseObject["result"]["identifier"]);
            Assert.AreEqual("Th7MpTaRZVRYnPiabds81Y", (string)responseObject["result"]["dest"]);
        }

        [TestMethod]
        public async Task TestSignAndSubmitRequestWorks()
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
        public async Task TestSignAndSubmitRequestWorksForNotFoundSigner()
        {
            var signerDidJson = "{\"seed\":\"00000000000000000000UnknowSigner\"}";

            var trusteeDidResult = await Did.CreateAndStoreMyDidAsync(wallet, signerDidJson);
            var signerDid = trusteeDidResult.Did;

            var myDidResult = await Did.CreateAndStoreMyDidAsync(wallet, "{}");
            var myDid = myDidResult.Did;

            var nymRequest = await Ledger.BuildNymRequestAsync(signerDid, myDid, null, null, null);            

            var ex = await Assert.ThrowsExceptionAsync<InvalidLedgerTransactionException>(() =>
               Ledger.SignAndSubmitRequestAsync(pool, wallet, signerDid, nymRequest)
            );
        }

        [TestMethod]
        public async Task TestSignAndSubmitRequestWorksForIncompatibleWalletAndPool()
        {
            var walletName = "incompatibleWallet";

            await Wallet.CreateWalletAsync("otherPoolName", walletName, "default", null, null);
            var wallet = await Wallet.OpenWalletAsync(walletName, null, null);

            var trusteeDidResult = await Did.CreateAndStoreMyDidAsync(wallet, TRUSTEE_IDENTITY_JSON);
            var trusteeDid = trusteeDidResult.Did;

            var myDidResult = await Did.CreateAndStoreMyDidAsync(wallet, "{}");
            var myDid = myDidResult.Did;

            var nymRequest = await Ledger.BuildNymRequestAsync(trusteeDid, myDid, null, null, null);            
            
            var ex = await Assert.ThrowsExceptionAsync<WrongWalletForPoolException>(() =>
                Ledger.SignAndSubmitRequestAsync(pool, wallet, trusteeDid, nymRequest)
            );            
        }
    }
}
