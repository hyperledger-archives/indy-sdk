using Base58Check;
using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Test.Wrapper.SignusTests
{
    [TestClass]
    public class CreateMyDidTest : IndyIntegrationTest
    {
        private Wallet _wallet;

        private string _seed = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        private string _did = "8wZcEriaNLNKtteJvx7f8i";
        private string _expectedVerkey = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";
        private string _existsCryptoType = "ed25519";

        [TestInitialize]
        public void CreateWallet()
        {
            Wallet.CreateWalletAsync("default", "signusWallet", "default", null, null).Wait();
            _wallet = Wallet.OpenWalletAsync("signusWallet", null, null).Result;
        }

        [TestCleanup]
        public void DeleteWallet()
        {
            _wallet.CloseAsync().Wait();
            Wallet.DeleteWalletAsync("signusWallet", null).Wait();
        }
        
        [TestMethod]
        public void TestCreateMyDidWorksForEmptyJson()
        {
            var json = "{}";

            var result = Signus.CreateAndStoreMyDidAsync(_wallet, json).Result;
            Assert.IsNotNull(result);

            Assert.AreEqual(16, Base58CheckEncoding.DecodePlain(result.Did).Length);
            Assert.AreEqual(32, Base58CheckEncoding.DecodePlain(result.VerKey).Length);
        }

        [TestMethod]
        public void TestCreateMyDidWorksForSeed()
        {
            var json = string.Format("{{\"seed\":\"{0}\"}}", _seed);

            var result = Signus.CreateAndStoreMyDidAsync(_wallet, json).Result;
            Assert.IsNotNull(result);

            Assert.AreEqual("NcYxiDXkpYi6ov5FcYDi1e", result.Did);
            Assert.AreEqual(_expectedVerkey, result.VerKey);
        }

        [TestMethod]
        public void TestCreateMyDidWorksAsCid()
        {
            var json = string.Format("{{\"seed\":\"{0}\",\"cid\":true}}", _seed);

            var result = Signus.CreateAndStoreMyDidAsync(_wallet, json).Result;
            Assert.IsNotNull(result);

            Assert.AreEqual(_expectedVerkey, result.Did);
            Assert.AreEqual(_expectedVerkey, result.VerKey);
        }

        [TestMethod]
        public void TestCreateMyDidWorksForPassedDid()
        {
            var json = string.Format("{{\"did\":\"{0}\",\"cid\":false}}", _did);

            var result = Signus.CreateAndStoreMyDidAsync(_wallet, json).Result;
            Assert.IsNotNull(result);

            Assert.AreEqual(_did, result.Did);
        }

        [TestMethod]
        public void TestCreateMyDidWorksForCorrectCryptoType()
        {
            var json = string.Format("{{\"crypto_type\":\"{0}\"}}", _existsCryptoType);

            var result = Signus.CreateAndStoreMyDidAsync(_wallet, json).Result;
            Assert.IsNotNull(result);
        }

        [TestMethod]
        public async Task testCreateMyDidWorksForInvalidSeed()
        {
            var json = "{\"seed\":\"aaaaaaaaaaa\"}";

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Signus.CreateAndStoreMyDidAsync(_wallet, json)
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestCreateMyDidWorksForInvalidCryptoType()
        {
            var json = "{\"crypto_type\":\"crypto_type\"}";

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Signus.CreateAndStoreMyDidAsync(_wallet, json)
            );

            Assert.AreEqual(ErrorCode.SignusUnknownCryptoError, ex.ErrorCode);
        }

        [TestMethod]
        public void TestCreateMyDidWorksForAllParams()
        {
            var json = string.Format("{{\"did\":\"{0}\",\"seed\":\"{1}\",\"crypto_type\":\"{2}\",\"cid\":true}}", _did, _seed, _existsCryptoType);

            var result = Signus.CreateAndStoreMyDidAsync(_wallet, json).Result;
            Assert.IsNotNull(result);

            Assert.AreEqual(_did, result.Did);
            Assert.AreEqual(_expectedVerkey, result.VerKey);
        }

    }
}
