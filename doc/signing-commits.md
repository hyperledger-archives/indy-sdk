# Signing commits
We use developer certificate of origin (DCO) in all hyperledger repositories, so to get your pull requests accepted, you must certify you created your commits by signing off on each commit with a GPG key.

The easiest way to do this is to configure your local repository to automatically sign all of your commits.

Below we have made a simple walkthrough to do this, with links to Github's documentation at the bottom.

* Check for an existing GPG key:

     `$ gpg --list-secret-keys --keyid-format LONG`

* If no key is found, create a new one. If you already have a gpg key, skip to the next step.

  - `$ gpg --gen-key`
  - follow the prompts, using the your email that is connected with Github. Using a key size of 4096 is recommended
  - Once the key's been created, list the new key in step one

  - The terminal should print something like this:

    ```
    gpg --list-secret-keys --keyid-format LONG
    /Users/hubot/.gnupg/secring.gpg
    ------------------------------------
    sec   4096R/<<<<key-id>>>> 2016-03-10 [expires: 2017-03-10]
    uid                          Hubot
    ssb   4096R/42B317FD4BA89E7A 2016-03-10
    ```

* Configure your key with Git:
  - export the key: `$ gpg --armor --export <key-id>`
  - copy the terminal output beginning with `-----BEGIN PGP PUBLIC KEY BLOCK-----` and ending with `-----END PGP PUBLIC KEY BLOCK-----`.
  - add the key to github by navigating to github.com -> settings -> ssh and gpg keys
  - add the key to your local installation of git: `git config --global user.signingkey <key-id>`
  - To add your GPG key to your bash profile, paste the text below:
      `echo 'export GPG_TTY=$(tty)' >> ~/.bashrc`




[Github - Signing Commits with GPG](https://help.github.com/articles/signing-commits-with-gpg/)
