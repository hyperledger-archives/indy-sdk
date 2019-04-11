# Send Secure Message

This shows how to leverage indy-sdk's agent-to-agent communications
features to send and receive a secure message. Messaging primitives
are important in many interactions, although they are not explicitly
required in the other how-tos.

In case of troubles running the how-to, please read the [trouble shooting](../trouble-shooting.md) section.

## Prerequisites

Setup your workstation with an indy development virtual machine (VM). See [prerequisites](../prerequisites.md).

## Steps

### Step 1

In your normal workstation operating system (not the VM), open a text editor of your
choice and paste the *template* code of one of the available in the list bellow into 
a new file and saved it as `send_secure_msg.EXT`, replacing *EXT* with the proper file 
extension (e.g for python: `send_secure_msg.py`, for nodejs: `send_secure_msg.js`, and so on). 
We will be modifying this code in later steps.

[ [Python template](python/template.py) | [Java template](../not-yet-written.md) | [Rust template](rust/src/template.rs)]

This is a very simple app that allows you to act as a sender or a receiver
of a message. Take a minute to understand its basic structure: it runs a
loop, asking you for input.

It recognizes three commands: `read`, `prep`, and `quit`.

### Step 2

Run the app on your workstation.
You should see some simple text output. Try a few commands.

Type `quit` when you're done.

### Step 3

Now we get to begin adding interesting features.

The first thing we need to do is give the app the DIDs and the keys it
needs to communicate. 
Open the correspondent `step3` file below in a text editor and copy
its contents into `send_secure_msg` file created in the first step, replacing the stub of
the `init()` function from your template. Save it and study the changes.

[ [Python step3](python/step3.py) | [Rust step3](rust/src/step3.rs)]

First `init()` asks the user for their name. It then invokes indy-sdk's
`create_wallet()` function to make a wallet associated with a fictional
pool (ledger). It records the wallet handle. Once the wallet has been
created, it opens the wallet and asks indy-sdk to generate a DID and a
public verkey + private signing key pair, storing all that information
in the wallet.

Finally, `init()` asks the user to provide the DID and verkey of the
other party that will be exchanging messages, and it returns a tuple of
all the information it's accumulated, except for the secret signing key
that remains in the wallet.

### Step 4

In your development VM, run the app again.

You should be prompted for a name. Say "Alice". Then the app should show you the DID and verkey it generated, and ask you for the DID and verkey of the other party.

Press **CTRL+C** to kill the app.

```
Who are you? Alice
wallet = 1
my_did and verkey = ChST7uE2KH3hscqycs5mVf 7NmnhUTnqqh1xydTSZNZCp1wTt3HAua7gA5T2odzLSTf
Other party's DID and verkey? ^C
```

We will eventually run this app twice--in one window as Alice, and in another
window as Bob. Each instance of the app will generate its own keys and
you can use copy/paste to share them with the other window.

### Step 5

Now we need to add secure encryption using indy crypto's [`auth_crypt()`](https://github.com/hyperledger/indy-sdk/blob/eb7ea544ae8616883c6011a57d40f1b14cd5afeb/libindy/src/api/crypto.rs#L328) primitive.

Copy the contents of the correspondent `step5` file below into your `send_secure_msg` file, replacing the stub of the `prep()` function on it. Save the file and study the changes.

[ [Python step5](python/step5.py) | [Rust step5](rust/src/step5.rs)]

The `prep()` function is designed to be called with a string argument entered
on the command line.

For example, when the user types `prep Hello, world`, the `msg` parameter of `prep()` receives the value `"Hello, world"`. Prep encode and saves this message into a binary file to be read later in the next step, simulating a communication channel between two agents. Then it calls `auth_crypt()` to convert those bytes into a version that is encrypted especially for the private channel between two identities. The receiver will know that the sender created the message, that only the receiver can interpret it, and that it has not been tampered with.

If you like, try running the updated app in your development VM again. When
prompted for the DID and verkey of the other party, just paste values produced 
during a previous run, separated by space (e.g., `ChST7uE2KH3hscqycs5mVf 7NmnhUTnqqh1xydTSZNZCp1wTt3HAua7gA5T2odzLSTf`) and press enter and then try typing `prep Hello, world` to create the encrypted `message.dat` file.

Press **CTRL+C** to kill the app.

### Step 6

The final feature that our app needs is the ability to read encrypted data.
Copy the contents of the correspondent `step6` file into your `send_secure_msg` file, replacing the stub of the `prep()` function on it. Then save and study the changes. 

[ [Python step6](python/step6.py) | [Rust step6](rust/src/step6.rs)]

The read function is very simple. It just copies the content of the
`message.dat` file from disk into memory and then calls indy crypto's
`auth_decrypt()` function on the byte array. The output is a tuple that
contains the verkey of the sender and the decrypted value if the
message was encrypted for the key of the recipient, or an exception if
not.

### Step 7

Now let's put this all together. Try to run the completed demo and observe the whole sequence.

[ [Python complete](python/send_secure_msg.py) | [Rust complete](rust/src/send-secure-msg.rs)]

In your indy development VM, create two shells (command prompts). Both should have the location of your code as their current working directory. Run one instance of your `send_secure_msg` script in the first window, and another in the second window.

In the first window, when prompted for your name, say "Alice". In the
second window, say "Bob".

In the Alice window, you should now see a line that specifies a DID and
verkey for Alice. It should be something like this:

```
my_did and verkey = ChST7uE2KH3hscqycs5mVf 7NmnhUTnqqh1xydTSZNZCp1wTt3HAua7gA5T2odzLSTf
```

Copy these two strings (everything after `"my_did and verkey = "`) to your
clipboard.

Navigate to the Bob window. Bob's window should have a different did and verkey displayed, and should be prompting for Alice's info.

Paste Alice's information into Bob's window and press Enter.

Copy Bob's information and paste it into Alice's window.

**Note:** The process of copying and pasting between two windows is a simplistic way to model more sophisticated onboarding workflows in the Sovrin ecosystem. Within the Sovrin ecosystem parties receive a trusted mutual introduction, or where one scans a QR code from the other, or where they exchange information over the phone or face to face.

Both windows should now be displaying a prompt again.

In the Alice window, type `prep Hi, Bob.` and press **Enter**.

In the Bob window, type `read` and press **Enter**.

Bob's window should display a tuple that it decrypts from Alice; the first
element in the tuple is Alice's verkey (check its value to confirm); the
second is the text, `"Hi, Bob."`. Alice has sent Bob an encrypted message.

## More Experiments

Try modifying `message.dat` with a binary editor. This simulates tampering by
an eavesdropper. When Bob attempts to decrypt the message, decryption should
fail.

Try transmitting the encrypted message over a different channel. For example,
after Alice writes the message, email `message.dat` or copy it over the network
or send it as an attachment via Skype or Slack. Then copy the received package
into the correct folder with the name `message.dat` and ask Bob to read it.

Try modifying the script so it uses `anon_crypt()` instead of `auth_crypt()` and 
`anon_decrypt()` instead of `auth_decrypt()`. Notice the outcome.
