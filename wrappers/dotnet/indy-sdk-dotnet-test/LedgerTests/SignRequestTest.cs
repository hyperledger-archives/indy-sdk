using Hyperledger.Indy.LedgerApi;
using Hyperledger.Indy.SignusApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.LedgerTests
{
    [TestClass]
    public class SignRequestTest : IndyIntegrationTestBase
    {
        private Wallet _wallet;
        private string _did;
        private string _walletName = "ledgerWallet";

        [TestInitialize]
        public async Task CreateWalletWhitDid()
        {
            await Wallet.CreateWalletAsync("default", _walletName, "default", null, null);
            _wallet = await Wallet.OpenWalletAsync(_walletName, null, null);
    
            var result = await Signus.CreateAndStoreMyDidAsync(_wallet, "{\"seed\":\"000000000000000000000000Trustee1\"}");
            _did = result.Did;
        }

        [TestCleanup]
        public async Task DeleteWallet()
        {
            if(_wallet != null)
                await _wallet.CloseAsync();

            await Wallet.DeleteWalletAsync(_walletName, null);
        }

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

            var signedMessage = await Ledger.SignRequestAsync(_wallet, _did, msg);

            Assert.IsTrue(signedMessage.Contains(expectedSignature));
        }

        [TestMethod]
        public async Task TestSignWorksForUnknowDid()
        {
            var msg = "{\"reqId\":1496822211362017764}";

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Ledger.SignRequestAsync(_wallet, "8wZcEriaNLNKtteJvx7f8i", msg)
            );

            Assert.AreEqual(ErrorCode.WalletNotFoundError, ex.ErrorCode);

        }

        [TestMethod]
        public async Task TestSignWorksForInvalidMessageFormat()
        {
            var msg = "\"reqId\":1496822211362017764";

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
               Ledger.SignRequestAsync(_wallet, _did, msg)
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }
    }
}
