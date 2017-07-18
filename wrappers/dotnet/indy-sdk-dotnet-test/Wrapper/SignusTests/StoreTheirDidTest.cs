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
    public class StoreTheirDidTest : IndyIntegrationTest
    {
        private Wallet _wallet;

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
        public void TestStoreTheirDidWorks()
        {
            Signus.StoreTheirDidAsync(_wallet, "{\"did\":\"8wZcEriaNLNKtteJvx7f8i\"}").Wait();
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
        public void TestStoreTheirDidWorksWithVerkey()
        {
            var json = "{\"did\":\"8wZcEriaNLNKtteJvx7f8i\", " +
                "\"verkey\":\"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa\"}";

            Signus.StoreTheirDidAsync(_wallet, json).Wait();
        }

        [TestMethod]
        public async Task TestStoreTheirDidWorksWithoutDid()
        {
            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Signus.StoreTheirDidAsync(_wallet, "{\"verkey\":\"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa\"}")
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

        [TestMethod]
        public void TestStoreTheirDidWorksForCorrectCryptoType()
        {
            var json = "{\"did\":\"8wZcEriaNLNKtteJvx7f8i\", " +
                "\"verkey\":\"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa\", " +
                "\"crypto_type\": \"ed25519\"}";

            Signus.StoreTheirDidAsync(_wallet, json).Wait();
        }

        [TestMethod]
        public async Task TestStoreTheirDidWorksForInvalidCryptoType()
        {
            var json = "{\"did\":\"8wZcEriaNLNKtteJvx7f8i\", " +
                "\"verkey\":\"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa\", " +
                "\"crypto_type\": \"some_type\"}";

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Signus.StoreTheirDidAsync(_wallet, json)
            );

            Assert.AreEqual(ErrorCode.SignusUnknownCryptoError, ex.ErrorCode);
        }


    }
}
