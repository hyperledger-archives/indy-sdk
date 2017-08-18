using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System;
using System.Threading.Tasks;
using static Indy.Sdk.Dotnet.Wrapper.Agent;

namespace Indy.Sdk.Dotnet.Test.Wrapper.AgentTests
{
    [TestClass]
    public class AgentCloseConnectionTest : AgentIntegrationTestBase
    {
        private static TaskCompletionSource<Connection> _serverToClientConnectionTaskCompletionSource;

        private new static ConnectionOpenedHandler _incomingConnectionObserver = (listener, connection, senderDid, receiverDid) =>
        {
            Console.WriteLine("New connection " + connection);

            _serverToClientConnectionTaskCompletionSource.SetResult(connection);

            return _messageObserverForIncoming;
        };

        [TestInitialize]
        public void Initialize()
        {
            _serverToClientConnectionTaskCompletionSource = new TaskCompletionSource<Connection>();
        }

        [TestMethod]
        public async Task TestAgentCloseConnectionWorksForOutgoing()
        {
            var endpoint = "127.0.0.1:9603";

            var myDid = await Signus.CreateAndStoreMyDidAsync(_wallet, "{}");

            var identityJson = string.Format("{{\"did\":\"{0}\", \"pk\":\"{1}\", \"verkey\":\"{2}\", \"endpoint\":\"{3}\"}}",
                    myDid.Did, myDid.Pk, myDid.VerKey, endpoint);

            await Signus.StoreTheirDidAsync(_wallet, identityJson);

            var activeListener = await Agent.AgentListenAsync(endpoint, _incomingConnectionObserver);

            await activeListener.AddIdentityAsync(_pool, _wallet, myDid.Did);

            var connection = await Agent.AgentConnectAsync(_pool, _wallet, myDid.Did, myDid.Did, _messageObserver);

            await connection.CloseAsync();

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                connection.SendAsync("msg")
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestAgentCloseConnectionWorksForIncoming()
        {
            var endpoint = "127.0.0.1:9613";

            var myDid = await Signus.CreateAndStoreMyDidAsync(_wallet, "{}");

            var identityJson = string.Format("{{\"did\":\"{0}\", \"pk\":\"{1}\", \"verkey\":\"{2}\", \"endpoint\":\"{3}\"}}",
                    myDid.Did, myDid.Pk, myDid.VerKey, endpoint);

            await Signus.StoreTheirDidAsync(_wallet, identityJson);

            var activeListener = await Agent.AgentListenAsync(endpoint, _incomingConnectionObserver);

            await activeListener.AddIdentityAsync(_pool, _wallet, myDid.Did);

            var connection = await Agent.AgentConnectAsync(_pool, _wallet, myDid.Did, myDid.Did, _messageObserver);

            var serverToClientConnection = await _serverToClientConnectionTaskCompletionSource.Task;
            await serverToClientConnection.CloseAsync();


            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                serverToClientConnection.SendAsync("msg")
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

    }
}
