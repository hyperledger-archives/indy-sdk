using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Test.Wrapper.SignusTests
{
    [TestClass]
    public class VerifyTest : IndyIntegrationTest
    {
        private Pool _pool;
        private Wallet _wallet;
        private string _trusteeDid;
        private string _trusteeVerkey;
        private string _identityJson;
        private string _newDid;

        [TestInitialize]
        public void CreateWalletWithDid()
        {
            var poolName = PoolUtils.CreatePoolLedgerConfig();

            _pool = Pool.OpenPoolLedgerAsync(poolName, "{}").Result;

            Wallet.CreateWalletAsync(poolName, "signusWallet", "default", null, null).Wait();
            _wallet = Wallet.OpenWalletAsync("signusWallet", null, null).Result;
            
            var json = "{\"seed\":\"000000000000000000000000Trustee1\",\"cid\":false}";

            var result = Signus.CreateAndStoreMyDidAsync(_wallet, json).Result;
            Assert.IsNotNull(result);

            _trusteeDid = result.Did;
            _trusteeVerkey = result.VerKey;
        }

        [TestCleanup]
        public void DeleteWallet()
        {
            _wallet.CloseAsync().Wait();
            Wallet.DeleteWalletAsync("signusWallet", null).Wait();
            _pool.CloseAsync().Wait();
        }

        private void CreateNewNymWithDidInLedger()
        {
            var json = "{\"seed\":\"00000000000000000000000000000My1\"}";

            var result = Signus.CreateAndStoreMyDidAsync(_wallet, json).Result;
            _newDid = result.Did;
            var newVerkey = result.VerKey;

            var nymRequest = Ledger.BuildNymRequestAsync(_trusteeDid, _newDid, newVerkey, null, null).Result;
            Ledger.SignAndSubmitRequestAsync(_pool, _wallet, _trusteeDid, nymRequest).Wait();
        }

        [TestMethod]
        public void TestVerifyWorksForVerkeyCachedInWallet()
        {
            _identityJson = string.Format("{{\"did\":\"{0}\",\"verkey\":\"{1}\"}}", _trusteeDid, _trusteeVerkey);
            Signus.StoreTheirDidAsync(_wallet, _identityJson).Wait();

            var msg = "{\n" +
                "                \"reqId\":1496822211362017764,\n" +
                "                \"identifier\":\"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL\",\n" +
                "                \"operation\":{\n" +
                "                    \"type\":\"1\",\n" +
                "                    \"dest\":\"VsKV7grR1BUE29mG2Fm2kX\",\n" +
                "                    \"verkey\":\"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa\"\n" +
                "                },\n" +
                "                \"signature\":\"65hzs4nsdQsTUqLCLy2qisbKLfwYKZSWoyh1C6CU59p5pfG3EHQXGAsjW4Qw4QdwkrvjSgQuyv8qyABcXRBznFKW\"\n" +
                "            }";

            var valid = Signus.VerifySignatureAsync(_wallet, _pool, _trusteeDid, msg).Result;
            Assert.IsTrue(valid);
        }

        [TestMethod]
        public void TestVerifyWorksForGetVerkeyFromLedger()
        {
            CreateNewNymWithDidInLedger();
            _identityJson = string.Format("{{\"did\":\"{0}\"}}", _newDid);
            Signus.StoreTheirDidAsync(_wallet, _identityJson).Wait();

            var msg = "{\"reqId\":1496822211362017764,\n" +
                "\"signature\":\"tibTuE59pZn1sCeZpNL5rDzpkpqV3EkDmRpFTizys9Gr3ZieLdGEGyq4h8jsVWW9zSaXSRnfYcVb1yTjUJ7vJai\"}";

            var valid = Signus.VerifySignatureAsync(_wallet, _pool, _newDid, msg).Result;
            Assert.IsTrue(valid);
        }

        [TestMethod]
        public void TestVerifyWorksForGetNymFromLedger()
        {
            CreateNewNymWithDidInLedger();

            var msg = "{\"reqId\":1496822211362017764,\n" +
                "\"signature\":\"tibTuE59pZn1sCeZpNL5rDzpkpqV3EkDmRpFTizys9Gr3ZieLdGEGyq4h8jsVWW9zSaXSRnfYcVb1yTjUJ7vJai\"}";

            var valid = Signus.VerifySignatureAsync(_wallet, _pool, _newDid, msg).Result;
            Assert.IsTrue(valid);
        }
        

        [TestMethod]
        public async Task TestVerifyWorksForInvalidMessageFormat()
        {
            var msg = "\"signature\":\"65hzs4nsdQsTUqLCLy2qisbKLfwYKZSWoyh1C6CU59p5pfG3EHQXGAsjW4Qw4QdwkrvjSgQuyv8qyABcXRBznFKW\"";


            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Signus.VerifySignatureAsync(_wallet, _pool, _trusteeDid, msg)
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestVerifyWorksForMessageWithoutSignature()
        {
            var msg = "{\n" +
                "                \"reqId\":1496822211362017764,\n" +
                "                \"identifier\":\"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL\",\n" +
                "                \"operation\":{\n" +
                "                    \"type\":\"1\",\n" +
                "                    \"dest\":\"VsKV7grR1BUE29mG2Fm2kX\",\n" +
                "                    \"verkey\":\"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa\"\n" +
                "                },\n" +
                "            }";

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Signus.VerifySignatureAsync(_wallet, _pool, _trusteeDid, msg)
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

        [TestMethod]
        public void TestVerifyWorksForOtherSigner()
        {
            _identityJson = string.Format("{{\"did\":\"{0}\", \"verkey\":\"{1}\"}}", _trusteeDid, _trusteeVerkey);

            Signus.StoreTheirDidAsync(_wallet, _identityJson).Wait();

            var createDidJson = "{\"seed\":\"000000000000000000000000Steward1\"}";

            var result = Signus.CreateAndStoreMyDidAsync(_wallet, createDidJson).Result;
            var stewardDid = result.Did;
            var stewardVerkey = result.VerKey;

            _identityJson = string.Format("{{\"did\":\"{0}\", \"verkey\":\"{1}\"}}", stewardDid, stewardVerkey);

            Signus.StoreTheirDidAsync(_wallet, _identityJson).Wait();

            var msg = "{\n" +
                    "                \"reqId\":1496822211362017764,\n" +
                    "                \"identifier\":\"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL\",\n" +
                    "                \"operation\":{\n" +
                    "                    \"type\":\"1\",\n" +
                    "                    \"dest\":\"VsKV7grR1BUE29mG2Fm2kX\",\n" +
                    "                    \"verkey\":\"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa\"\n" +
                    "                }\n" +
                    "            }";

            var signedMessage = Signus.SignAsync(_wallet, _trusteeDid, msg).Result;

            var valid = Signus.VerifySignatureAsync(_wallet, _pool, stewardDid, signedMessage).Result;
            Assert.IsFalse(valid);
        }
    }
}
