using Hyperledger.Indy.LedgerApi;
using Hyperledger.Indy.DidApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.LedgerTests
{
    [TestClass]
    public class SignRequestTest : IndyIntegrationTestWithPoolAndSingleWallet
    {        
        [TestMethod]
        public async Task TestSignWorks()
        {
            var msg = "{\n" +
                    "                \"reqId\":1496822211362017764,\n" +
                    "                \"identifier\":\"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL\",\n" +
                    "                \"operation\":{\n" +
                    "                    \"type\":\"1\",\n" +
                    "                    \"dest\":\"VsKV7grR1BUE29mG2Fm2kX\",\n" +
                    "                    \"verkey\":\"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa\"\n" +
                    "                }\n" +
                    "            }";

            var expectedSignature = "\"signature\":\"65hzs4nsdQsTUqLCLy2qisbKLfwYKZSWoyh1C6CU59p5pfG3EHQXGAsjW4Qw4QdwkrvjSgQuyv8qyABcXRBznFKW\"";

            var result = await Did.CreateAndStoreMyDidAsync(wallet, TRUSTEE_IDENTITY_JSON);
            var did = result.Did;

            var signedMessage = await Ledger.SignRequestAsync(wallet, did, msg);

            Assert.IsTrue(signedMessage.Contains(expectedSignature));
        }

        [TestMethod]
        public async Task TestSignWorksForUnknowDid()
        {
            var msg = "{\"reqId\":1496822211362017764}";

            var ex = await Assert.ThrowsExceptionAsync<WalletValueNotFoundException>(() =>
                Ledger.SignRequestAsync(wallet, DID1, msg)
            );

        }

        [TestMethod]
        public async Task TestSignWorksForInvalidMessageFormat()
        {
            var result = await Did.CreateAndStoreMyDidAsync(wallet, TRUSTEE_IDENTITY_JSON);
            var did = result.Did;

            var msg = "\"reqId\":1496822211362017764";

            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
               Ledger.SignRequestAsync(wallet, did, msg)
            );
        }
    }
}
