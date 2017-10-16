using Hyperledger.Indy.AgentApi;
using Hyperledger.Indy.SignusApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.AgentTests
{
    [TestClass]    
    public class DisposeAgentListenerTest : AgentIntegrationTestBase
    {
        public async Task PrepareForListener(string endpoint)
        {
            var myDidResult = await Signus.CreateAndStoreMyDidAsync(wallet, "{}");

            var identityJson = string.Format(AGENT_IDENTITY_JSON_TEMPLATE, myDidResult.Did, myDidResult.Pk, myDidResult.VerKey, endpoint);
            await Signus.StoreTheirDidAsync(wallet, identityJson);
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
        [Ignore] //Wait until proper error is implemented in SDK and handle.
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