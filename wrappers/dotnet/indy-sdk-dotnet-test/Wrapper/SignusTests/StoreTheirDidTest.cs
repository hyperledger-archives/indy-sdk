﻿using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Test.Wrapper.SignusTests
{
    [TestClass]
    public class StoreTheirDidTest : IndyIntegrationTestBase
    {
        private Wallet _wallet;
        private string _walletName = "signusWallet";
        private string _did = "8wZcEriaNLNKtteJvx7f8i";
        private string _verkey = "GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa";

        [TestInitialize]
        public async Task CreateWallet()
        {
            await Wallet.CreateWalletAsync("default", _walletName, "default", null, null);
            _wallet = await Wallet.OpenWalletAsync(_walletName, null, null);
        }

        [TestCleanup]
        public async Task DeleteWallet()
        {
            if (_wallet != null)
                await _wallet.CloseAsync();

            await Wallet.DeleteWalletAsync(_walletName, null);
        }
        
        [TestMethod]
        public async Task TestStoreTheirDidWorks()
        {
            await Signus.StoreTheirDidAsync(_wallet, string.Format("{{\"did\":\"{0}\"}}", _did));
        }

        [TestMethod]
        public async Task TestCreateMyDidWorksForInvalidIdentityJson()
        {
            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Signus.StoreTheirDidAsync(_wallet, "{\"field\":\"value\"}")
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestStoreTheirDidWorksWithVerkey()
        {
            var json = string.Format("{{\"did\":\"{0}\", \"verkey\":\"{1}\"}}", _did, _verkey);

            await Signus.StoreTheirDidAsync(_wallet, json);
        }

        [TestMethod]
        public async Task TestStoreTheirDidWorksWithoutDid()
        {
            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Signus.StoreTheirDidAsync(_wallet, string.Format("{{\"verkey\":\"{0}\"}}", _verkey))
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestStoreTheirDidWorksForCorrectCryptoType()
        {
            var json = string.Format("{{\"did\":\"{0}\", " +
                "\"verkey\":\"{1}\", " +
                "\"crypto_type\": \"ed25519\"}}", _did, _verkey);

            await Signus.StoreTheirDidAsync(_wallet, json);
        }

        [TestMethod]
        public async Task TestStoreTheirDidWorksForInvalidCryptoType()
        {
            var json = string.Format("{{\"did\":\"{0}\", " +
                "\"verkey\":\"{1}\", " +
                "\"crypto_type\": \"some_type\"}}", _did, _verkey);

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Signus.StoreTheirDidAsync(_wallet, json)
            );

            Assert.AreEqual(ErrorCode.SignusUnknownCryptoError, ex.ErrorCode);
        }


    }
}
