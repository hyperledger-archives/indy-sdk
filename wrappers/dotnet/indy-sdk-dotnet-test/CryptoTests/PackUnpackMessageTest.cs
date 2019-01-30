using System;
using System.Collections.Generic;
using System.Text;
using System.Threading.Tasks;
using Hyperledger.Indy.CryptoApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;

namespace Hyperledger.Indy.Test.CryptoTests
{
    [TestClass]
    public class PackUnpackMessageAsyncAsyncTest : IndyIntegrationTestWithSingleWallet
    {
        [TestMethod]
        public async Task TestPackMessageSuccessfully()
        {
            const string message = "hello world";

            var receiversArray = new JArray {VERKEY_MY1, VERKEY_MY2, VERKEY_TRUSTEE};

            var myVk = await Crypto.CreateKeyAsync(wallet, MY1_IDENTITY_KEY_JSON);

            var packedMessage = await Crypto.PackMessageAsync(wallet, receiversArray.ToString(), null,
                Encoding.UTF8.GetBytes(message));

            Assert.IsNotNull(packedMessage);
        }

        [TestMethod]
        public async Task TestPackMessageSuccessfullyWithOneReceiver()
        {
            const string message = "hello world";

            var receiversArray = new JArray();
            receiversArray.Add(VERKEY_MY1);

            await Crypto.CreateKeyAsync(wallet, MY1_IDENTITY_KEY_JSON);

            var packedMessage = await Crypto.PackMessageAsync(wallet, receiversArray.ToString(), null,
                Encoding.UTF8.GetBytes(message));

            Assert.IsNotNull(packedMessage);
        }

        [TestMethod]
        public async Task TestPackSuccessWithSenderVerkey()
        {
            const string message = "hello world";

            var receiversArray = new JArray {VERKEY_MY1};

            var myVk = await Crypto.CreateKeyAsync(wallet, MY1_IDENTITY_KEY_JSON);

            var packedMessage = await Crypto.PackMessageAsync(wallet, receiversArray.ToString(), myVk,
                Encoding.UTF8.GetBytes(message));

            Assert.IsNotNull(packedMessage);
        }

        [TestMethod]
        public async Task TestPackErrorsWithIncorrectSenderVerkey()
        {
            const string message = "hello world";

            var receiversArray = new JArray {VERKEY_MY1};

            var myVk = await Crypto.CreateKeyAsync(wallet, MY1_IDENTITY_KEY_JSON);

            await Assert.ThrowsExceptionAsync<WalletItemNotFoundException>(async () => await Crypto.PackMessageAsync(
                wallet, receiversArray.ToString(), VERKEY_MY2, Encoding.UTF8.GetBytes(message)));
        }

        [TestMethod]
        public async Task TestPackMessageErrorsWithNoReceivers()
        {
            const string message = "hello world";

            var receiversArray = new JArray();

            await Crypto.CreateKeyAsync(wallet, MY1_IDENTITY_KEY_JSON);

            await Assert.ThrowsExceptionAsync<InvalidStructureException>(async () => await Crypto.PackMessageAsync(
                wallet, receiversArray.ToString(), null, Encoding.UTF8.GetBytes(message)));
        }

        [TestMethod]
        public async Task TestPackMessageErrorsInvalidReceivers()
        {
            const string message = "hello world";

            var receiversArray = new JArray {"VERKEY_MY1"};

            var myVk = await Crypto.CreateKeyAsync(wallet, MY1_IDENTITY_KEY_JSON);

            await Assert.ThrowsExceptionAsync<InvalidStructureException>(async () => await Crypto.PackMessageAsync(
                wallet, receiversArray.ToString(), null, Encoding.UTF8.GetBytes(message)));
        }

        [TestMethod]
        public async Task TestUnpackMessageErrorsWithInvalidStructure()
        {
            const string packedMessage = "gibberish";
            await Assert.ThrowsExceptionAsync<InvalidStructureException>(async () => await Crypto.UnpackMessageAsync(
                wallet, Encoding.UTF8.GetBytes(packedMessage)));
        }

        [TestMethod]
        public async Task TestUnpackMessageSuccessfully()
        {
            const string message = "hello world";

            var receiversArray = new JArray {VERKEY_MY1, VERKEY_MY2, VERKEY_TRUSTEE};

            var myVk = await Crypto.CreateKeyAsync(wallet, MY1_IDENTITY_KEY_JSON);

            var packedMessage = await Crypto.PackMessageAsync(wallet, receiversArray.ToString(), null,
                Encoding.UTF8.GetBytes(message));
            var unpackedMessage = await Crypto.UnpackMessageAsync(wallet, packedMessage);

            Assert.IsNotNull(unpackedMessage);
        }

        [TestMethod]
        public async Task TestUnpackSuccessWithSenderVerkey()
        {
            const string message = "hello world";

            var receiversArray = new JArray {VERKEY_MY1};

            var myVk = await Crypto.CreateKeyAsync(wallet, MY1_IDENTITY_KEY_JSON);

            var packedMessage = await Crypto.PackMessageAsync(wallet, receiversArray.ToString(), myVk,
                Encoding.UTF8.GetBytes(message));
            var unpackedMessage = await Crypto.UnpackMessageAsync(wallet, packedMessage);

            Assert.IsNotNull(unpackedMessage);
        }
    }
}