namespace Hyperledger.Indy.Test.AgentTests
{
    public abstract class AgentIntegrationTestBase : IndyIntegrationTestWithPoolAndSingleWallet
    {
        protected const string AGENT_IDENTITY_JSON_TEMPLATE = "{{\"did\":\"{0}\", \"pk\":\"{1}\", \"verkey\":\"{2}\", \"endpoint\":\"{3}\"}}";
    }
}
