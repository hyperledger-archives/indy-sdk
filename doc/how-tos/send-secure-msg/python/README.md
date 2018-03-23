# Send a Secure Message
Indy-SDK Developer Walkthrough #3, Python Edition

[ [Java](../java/README.md) | [.NET](../dotnet/README.md) | [Node.js](../node/README.md) | [Objective C](../objectivec/README.md) ]


## Prerequisites

Setup your workstation and indy development virtual machine. See [prerequisites](../prerequisites.md).

## Steps

### Step 1

In your normal workstation OS (not the VM), open a python editor of your
choice and paste the code from [template.py](template.py)
into a new doc. We will be modifying this code in later steps.

Save the doc as `msgme.py`

This is a very simple app that allows you to act as a sender or a receiver
of a message. Take a minute to understand its basic structure: it runs a
loop, asking you for input.

It recognizes three commands: `read`, `prep`, and `quit`.

### Step 2

Run the app on your workstation if you have python3 installed there, or
in your indy development VM, otherwise: `python3 msgme.py`.

You should see some simple text output. Try a few commands.

Type `quit` when you're done.

### Step 3

Now we get to begin adding interesting features.

The first thing we need to do is give the app the DIDs and the keys it
needs to communicate. Open [step3.py](step3.py) in a text editor and copy
its contents into `msgme.py`, replacing the stub of
the `init()` function from your template.

We will also have to import some dependencies, since this function uses
indy and thus depends on the SDK. Go to the top of `msgme.py` and add
a new import statement:

  ```python
from indy import crypto, did, wallet
```

Save the updated version of `msgme.py`. Now take a minute and study the changes.

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
vagrant@development:/src/indy-sdk/samples/python/src$ python3 msgme1.py
Who are you? Alice
wallet = 1
my_did and verkey = ChST7uE2KH3hscqycs5mVf 7NmnhUTnqqh1xydTSZNZCp1wTt3HAua7gA5T2odzLSTf
Other party's DID and verkey? ^C
```

We will eventually run this app twice--in one window as Alice, and in another
window as Bob. Each instance of the app will generate its own keys and
you can use copy/paste to share them with the other window.

### Step 5

Now we need to add secure encryption using indy crypto's [`auth_crypt()`](https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/crypto.rs#L272)
primitive.

Copy the contents of [step5.py](step5.py) into `msgme.py`, replacing the stub of
the `prep()` function from your template.

Save the updated version of `msgme.py`. Now take a minute and study the changes.

The `prep()` function is designed to be called with a string argument entered
on the command line.

For example, when the user types `prep Hello, world`, the `msg` parameter of `prep()` receives the value `"Hello, world"`. Prep saves this message into a text file (so you can inspect it--not because it needs to do any file I/O). It then turns around and reads the bytes it has just written, and calls `auth_crypt()` to convert those bytes into a version that is encrypted especially for the private channel between two identities. The receiver will know that the sender created the message, that only the receiver can interpret it, and that it has not been tampered with.

If you like, try running the updated app in your development VM again. When
prompted for the DID and verkey of the other party, just type two strings (e.g., `x y`) and press enter. Then try typing `prep Hello, world` and inspecting the `input.txt` and `encrypted.dat` files that are created.

Press **CTRL+C** to kill the app.

### Step 6

The final feature that our app needs is the ability to read encrypted data.
Copy the contents of [step6.py](step6.py) into `msgme.py`, replacing the stub of the `prep()`
function from your template.

Save the updated version of `msgme.py`. Now take a minute and study the
changes.

The read function is very simple. It just copies the content of the
`encrypted.dat` file from disk into memory and then calls indy crypto's
`auth_decrypt()` function on the byte array. The output is a tuple that
contains the verkey of the sender and the decrypted value if the
message was encrypted for the key of the recipient--or an exception if
not.

### Step 7

Now let's put this all together.

In your indy development VM, create two shells (command prompts). Both should have the location of your code as their current working directory. Start one copy of your scripts in the first window (`python3 msgme.py`), and another copy in the second window.

In the first window, when prompted for your name, say "Alice".  In the
second window, say "Bob".

In the Alice window, you should now see a line that specifies a DID and
verkey for Alice. It should look something like this:

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

Try modifying `encrypted.dat` with a binary editor. This simulates tampering by
an eavesdropper. When Bob attempts to decrypt the message, decryption should
fail.

Try transmitting the encrypted message over a different channel. For example,
after Alice writes the message, email `encrypted.dat` or copy it over the network
or send it as an attachment via Skype or slack. Then copy the received package
into the correct folder with the name `encrypted.dat` and ask Bob to read it.

Try modifying the script so it uses `anon_crypt()` instead of `auth_crypt()`. Notice the outcome.
