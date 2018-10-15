using Microsoft.VisualStudio.TestTools.UnitTesting;

namespace Hyperledger.Indy.Test.PairwiseTests
{
    [TestClass]
    public class PaymentIntegrationTestBase : IndyIntegrationTestWithSingleWallet
    {
        protected const string paymentMethod = "null";
	    protected const string paymentAddress = "pay:null:test";
	    protected const string emptyObject = "{}";
	    protected const string emptyArray = "[]";
	    protected const string inputs = "[\"pay:null:1\", \"pay:null:2\"]";
	    protected const string outputs = "[{\"recipient\": \"pay:null:1\", \"amount\":1}, {\"recipient\": \"pay:null:2\", \"amount\":2}]";
	    protected const string invalidInputs = "pay:null:1";
	    protected const string incompatibleInputs = "[\"pay:PAYMENT_METHOD_1:1\", \"pay:PAYMENT_METHOD_2:1\"]";
	    protected const string incompatibleOutputs = "[{\"recipient\": \"pay:PAYMENT_METHOD_1:1\", \"amount\":1}, {\"recipient\": \"pay:PAYMENT_METHOD_2:1\", \"amount\":1}]";
	    protected const string fees = "{\"txnType1\":1, \"txnType2\":2}";
	    protected const string receipt = "pay:null:0_PqVjwJC42sxCTJp";

    }
}
