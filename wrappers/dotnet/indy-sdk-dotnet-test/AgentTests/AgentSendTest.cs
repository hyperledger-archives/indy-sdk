using Hyperledger.Indy.AgentApi;
using Hyperledger.Indy.SignusApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.AgentTests
{
    [TestClass]
    public class AgentSendTest : AgentIntegrationTestBase
    {
        [TestMethod]
        public async Task TestAgentSendWorks()
        {
            var endpoint = "127.0.0.1:9609";

            var myDidResult = await Signus.CreateAndStoreMyDidAsync(wallet, "{}");

            var identityJson = string.Format(AGENT_IDENTITY_JSON_TEMPLATE,  myDidResult.Did, myDidResult.Pk, myDidResult.VerKey, endpoint);
            await Signus.StoreTheirDidAsync(wallet, identityJson);

            var listener = await AgentListener.ListenAsync(endpoint);
            await listener.AddIdentityAsync(pool, wallet, myDidResult.Did);

            var clientConnection = await AgentConnection.ConnectAsync(pool, wallet, myDidResult.Did, myDidResult.Did);

            var connectionEvent = await listener.WaitForConnectionAsync();
            var serverConnection = connectionEvent.Connection;

            var waitListenerConnectionTask = listener.WaitForConnectionAsync(); //Start waiting for additional connections - we'll never get one in this test, however.

            var clientToServerMessage = "msg_from_client";
            var serverToClientMessage = "msg_from_server";                             

            await clientConnection.SendAsync(clientToServerMessage);

            var waitServerMessageTask = serverConnection.WaitForMessageAsync();

            var completedTask = await Task.WhenAny(waitListenerConnectionTask, waitServerMessageTask); //Wait for either an additional connection or message and proceed when one has arrived.
            Assert.AreEqual(completedTask, waitServerMessageTask);

            var serverMessageEvent = await waitServerMessageTask;
            Assert.AreEqual(clientToServerMessage, serverMessageEvent.Message);

            await serverConnection.SendAsync(serverToClientMessage);

            var clientMessageEvent = await clientConnection.WaitForMessageAsync();
            Assert.AreEqual(serverToClientMessage, clientMessageEvent.Message);
        }
    }
}
