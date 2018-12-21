using Hyperledger.Indy.PaymentsApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.PairwiseTests
{
    [TestClass]
    public class ParseVerifyPaymentResponseTest : PaymentIntegrationTestBase
    {
        [TestMethod]
        public async Task TestParseVerifyPaymentResponseWorksForUnknownPaymentMethod()
        {
            var ex = await Assert.ThrowsExceptionAsync<UnknownPaymentMethodException>(() =>
                 Payments.ParseVerifyPaymentResponseAsync(paymentMethod, emptyObject)
             );
        }
    }
}
