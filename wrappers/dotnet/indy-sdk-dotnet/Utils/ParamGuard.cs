using System;

namespace Hyperledger.Indy.Utils
{
    /// <summary>
    /// Common guards.
    /// </summary>
    internal static class ParamGuard
    {
        /// <summary>
        /// Guards against null values.
        /// </summary>
        /// <param name="param">The parameter to check.</param>
        /// <param name="paramName">The name of the parameter.</param>
        /// <exception cref="ArgumentNullException">Thrown if param was null.</exception>
        public static void NotNull(object param, string paramName)
        {
            if (param == null)
                throw new ArgumentNullException(paramName);
        }

        /// <summary>
        /// Guards against null and strings containing nothing but whitespace.
        /// </summary>
        /// <param name="param">The parameter to check.</param>
        /// <param name="paramName">The name of the parameter.</param>
        /// <exception cref="ArgumentException">Thrown if param was null or contained whitespace.</exception>
        public static void NotNullOrWhiteSpace(string param, string paramName)
        {
            if (string.IsNullOrWhiteSpace(param))
                throw new ArgumentException("A value must be provided.", paramName);
        }
    }
}
