using Hyperledger.Indy.AgentApi;
using Hyperledger.Indy.PoolApi;
using Hyperledger.Indy.SignusApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System;
using System.Collections.Generic;
using System.Text;
using System.Threading;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.AgentTests
{
    [TestClass]
    public class DisposeAgentListenerTest : AgentIntegrationTestBase
    {
        private string endpoint = "127.0.0.1:9607";        

        [TestInitialize]
        public async Task CreateListener()
        {
           var didJson = "{\"seed\":\"sovrin_agent_connect_works_for_a\"}";

            var myDidResult = await Signus.CreateAndStoreMyDidAsync(_wallet, didJson);

            var identityJson = string.Format("{{\"did\":\"{0}\", \"pk\":\"{1}\", \"verkey\":\"{2}\", \"endpoint\":\"{3}\"}}",
                    myDidResult.Did, myDidResult.Pk, myDidResult.VerKey, endpoint);

            await Signus.StoreTheirDidAsync(_wallet, identityJson);
        }

        [TestMethod]
        public async Task CanDisposeClosedListener()
        {
            using (var listener = await AgentListener.ListenAsync(endpoint))
            {
                await listener.CloseAsync();
            }
        }

        [TestMethod]
        public async Task DisposeCanBeCalledRepeatedly()
        {
            var listener = await AgentListener.ListenAsync(endpoint);
            listener.Dispose();
            listener.Dispose();
        }

        [TestMethod]
        public async Task EndpointCanBeReUsedAfterDispose()
        {
            var listener = await AgentListener.ListenAsync(endpoint);
            listener.Dispose();

            using (var newListener = await AgentListener.ListenAsync(endpoint))
            {
            }
        }

        [TestMethod]
        public async Task CanCloseAfterDispose()
        {
            var listener = await AgentListener.ListenAsync(endpoint);
            listener.Dispose();

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                listener.CloseAsync()
            );

            Assert.AreEqual(ErrorCode.WalletAlreadyExistsError, ex.ErrorCode);
        }
    }
}
