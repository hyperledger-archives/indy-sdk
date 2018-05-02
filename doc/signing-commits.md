# Signing commits

If you are here because you forgot to sign your commits, fear not. Check out [how to sign previous commits](#how-to-sign-previous-commits)

We use developer certificate of origin (DCO) in all hyperledger repositories, so to get your pull requests accepted, you must certify your commits by signing off on each commit with a GPG key.

Below we have made a simple walkthrough to do this, with the link to Github's documentation at the bottom.

### Get a GPG key:

1. Check to see if you already have a key: `$ gpg --list-secret-keys --keyid-format LONG`

1. If no key is found, create a new one. If you already have a gpg key, skip to the next step.

  - `$ gpg --gen-key`
  - follow the prompts, using the your email that is connected with Github. Using a key size of 4096 is recommended
  - Once the key's been created, list the new key as in step one


## Configure your key with Git:
  -  The terminal should print something like this:


    gpg --list-secret-keys --keyid-format LONG
    /Users/hubot/.gnupg/secring.gpg
    ------------------------------------
    sec   4096R/<<<<key-id>>>> 2016-03-10 [expires: 2017-03-10]
    uid                          Hubot
    ssb   4096R/42B317FD4BA89E7A 2016-03-10


  - Export the key using the key-id above: `$ gpg --armor --export <key-id>`
  - Copy the terminal output beginning with `-----BEGIN PGP PUBLIC KEY BLOCK-----` and ending with `-----END PGP PUBLIC KEY BLOCK-----`.
  - Add the key to github by navigating to github.com -> settings -> ssh and gpg keys
  - Add the key to your local installation of git: `git config --global user.signingkey <key-id>`
  - To add your GPG key to your bash profile, paste the text below:
      `echo 'export GPG_TTY=$(tty)' >> ~/.bashrc`
  - [optional, but recommended] To configure your Git client to sign commits by default for  this local repository, run `git config commit.gpgsign true`.


* Sign your commit
  - `$ git commit -S -m your commit message`
  - To see if your commits have been signed off, run `$ git log --show-signature`

## How to Sign Previous Commits
 1. Use `git-log --show-signature` to see which commits need to be signed.
 1. Go into interactive rebase mode using `$ git rebase -i HEAD~X` where X is the number of commits up to the most current commit you would like to see.
 1. You will see a list of the commits in a text file. On the line after each commit you need to sign, add `exec git commit --amend --no-edit -S`
    ```
    pick 12345 commit message
    exec git commit --amend --no-edit -S
    pick 67890 commit message
    exec git commit --amend --no-edit -S
    ```
  1. If you need to re-sign a bunch of previous commits at once, find the most recent unsigned commit using `git log` and use that commit's HASH in this command: `git rebase --exec 'git commit --amend --no-edit -n -S' -i HASH`  


For more information, see: [Github - Signing Commits with GPG](https://help.github.com/articles/signing-commits-with-gpg/)
