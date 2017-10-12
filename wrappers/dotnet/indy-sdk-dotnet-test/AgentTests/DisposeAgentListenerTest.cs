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
        public async Task PrepareForListener(string endpoint)
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
            var endpoint = "127.0.0.1:9614";
            await PrepareForListener(endpoint);

            using (var listener = await AgentListener.ListenAsync(endpoint))
            {
                await listener.CloseAsync();
            }
        }

        [TestMethod]
        public async Task DisposeCanBeCalledRepeatedly()
        {
            var endpoint = "127.0.0.1:9615";
            await PrepareForListener(endpoint);

            var listener = await AgentListener.ListenAsync(endpoint);
            listener.Dispose();
            listener.Dispose();
        }

        [TestMethod]
        [Ignore]//Appears endpoint cannot be re-connected to.  Requires further testing.
        public async Task EndpointCanBeReUsedAfterDispose()
        {
            var endpoint = "127.0.0.1:9616";
            await PrepareForListener(endpoint);

            var listener = await AgentListener.ListenAsync(endpoint);
            listener.Dispose();

            using (var newListener = await AgentListener.ListenAsync(endpoint))
            {
            }
        }

        [TestMethod]
        public async Task CanCloseAfterDispose()
        {
            var endpoint = "127.0.0.1:9617";
            await PrepareForListener(endpoint);

            var listener = await AgentListener.ListenAsync(endpoint);
            listener.Dispose();
            await listener.CloseAsync();
        }
    }
}
