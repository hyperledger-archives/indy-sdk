using System;
using System.Collections.Generic;
using System.Text;

namespace Hyperledger.Indy.AgentApi
{
    /// <summary>
    /// Result of parsing a message.
    /// </summary>
    public class ParseMsgResult
    {
        /// <summary>
        /// Initializes a new ParseMsgResult.
        /// </summary>
        /// <param name="senderKey">The key of the sender of the message.</param>
        /// <param name="msg">The parsed message.</param>
        internal ParseMsgResult(string senderKey, byte[] msg)
        {
            SenderKey = senderKey;
            Msg = msg;
        }

        /// <summary>
        /// Gets the key of the sender.
        /// </summary>
        public string SenderKey { get; private set; }

        /// <summary>
        /// Gets the parsed message.
        /// </summary>
        public byte[] Msg { get; private set; }
    }
}
