using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Test.Wrapper.AgentTests
{
    [TestClass]
    public class AgentSendTest : AgentIntegrationTestBase
    {
        [TestMethod]
        public async Task TestAgentSendWorks()
        {
            var endpoint = "127.0.0.1:9609";

            var myDidResult = await Signus.CreateAndStoreMyDidAsync(_wallet, "{}");

            var identityJson = string.Format("{{\"did\":\"{0}\", \"pk\":\"{1}\", \"verkey\":\"{2}\", \"endpoint\":\"{3}\"}}",
                    myDidResult.Did, myDidResult.Pk, myDidResult.VerKey, endpoint);
            await Signus.StoreTheirDidAsync(_wallet, identityJson);

            var listener = await AgentListener.ListenAsync(endpoint);
            await listener.AddIdentityAsync(_pool, _wallet, myDidResult.Did);

            var clientConnection = await AgentConnection.ConnectAsync(_pool, _wallet, myDidResult.Did, myDidResult.Did);

            var connectionEvent = await listener.WaitForConnection();
            var serverConnection = connectionEvent.Connection;

            var clientToServerMessage = "msg_from_client";
            var serverToClientMessage = "msg_from_server";

            await clientConnection.SendAsync(clientToServerMessage);

            var serverMessageEvent = await serverConnection.WaitForMessage();

            Assert.AreEqual(clientToServerMessage, serverMessageEvent.Message);

            await serverConnection.SendAsync(serverToClientMessage);

            var clientMessageEvent = await clientConnection.WaitForMessage();

            Assert.AreEqual(serverToClientMessage, clientMessageEvent.Message);
        }
    }
}
