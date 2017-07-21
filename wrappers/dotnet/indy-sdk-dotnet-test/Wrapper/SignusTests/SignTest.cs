using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Test.Wrapper.SignusTests
{
    [TestClass]
    public class SignTest : IndyIntegrationTestBase
    {
        private Wallet _wallet;

        private string _did;

        [TestInitialize]
        public void CreateWalletWithDid()
        {
            Wallet.CreateWalletAsync("default", "signusWallet", "default", null, null).Wait();
            _wallet = Wallet.OpenWalletAsync("signusWallet", null, null).Result;

            var result = Signus.CreateAndStoreMyDidAsync(_wallet, "{\"seed\":\"000000000000000000000000Trustee1\"}").Result;

            _did = result.Did;
        }

        [TestCleanup]
        public void DeleteWallet()
        {
            _wallet.CloseAsync().Wait();
            Wallet.DeleteWalletAsync("signusWallet", null).Wait();
        }
        
        [TestMethod]
        public void TestSignWorks()
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

            var signedMessage = Signus.SignAsync(_wallet, _did, msg).Result;

            Assert.IsTrue(signedMessage.Contains(expectedSignature));
        }

        [TestMethod]
        public async Task TestSignWorksForUnknownDid()
        {
            var msg = "{\"reqId\":1496822211362017764}";

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Signus.SignAsync(_wallet, "8wZcEriaNLNKtteJvx7f8i", msg)
            );

            Assert.AreEqual(ErrorCode.WalletNotFoundError, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestSignWorksForInvalidMessageFormat()
        {
            var msg = "reqId:1495034346617224651";

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Signus.SignAsync(_wallet, _did, msg)
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }
    }
}
