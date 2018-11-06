using Hyperledger.Indy.PaymentsApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.PairwiseTests
{
    [TestClass]
    public class ParseGetPaymentSourcesResponseTest : PaymentIntegrationTestBase
    {
        [TestMethod]
        public async Task TestParseGetPaymentSourcesResponseTestWorksForUnknownPaymentMethod()
        {
            var ex = await Assert.ThrowsExceptionAsync<UnknownPaymentMethodException>(() =>
                 Payments.ParseGetPaymentSourcesAsync(paymentMethod, emptyObject)
             );
        }
    }
}
