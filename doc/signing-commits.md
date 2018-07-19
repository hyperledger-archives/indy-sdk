# Signing commits

If you are here because you forgot to sign your commits, fear not. Check out [how to sign previous commits](#how-to-sign-previous-commits)

We use developer certificate of origin (DCO) in all hyperledger repositories, so to get your pull requests accepted, you must certify your commits by signing off on each commit.

## Signing your current commit
  - `$ git commit -s -m "your commit message"`
  - To see if your commits have been signed off, run `$ git log --show-signature`
  - If you need to re-sign the most current commit, use `$ git commit --amend --no-edit -s`.

The `-s` flag signs the commit message with your name and email.

## How to Sign Previous Commits
   1. Use `git log --show-signature` to see which commits need to be signed.
   1. Go into interactive rebase mode using `$ git rebase -i HEAD~X` where X is the number of commits up to the most current commit you would like to see.
   1. You will see a list of the commits in a text file. **On the line after each commit you need to sign**, add `exec git commit --amend --no-edit -s` with the lowercase `-s` adding a text signature in the commit body. Example that signs both commits:
      ```
      pick 12345 commit message
      exec git commit --amend --no-edit -s
      pick 67890 commit message
      exec git commit --amend --no-edit -s
      ```
    1. If you need to re-sign a bunch of previous commits at once, find the earliest unsigned commit using `git log --show-signature` and use that the HASH of the commit before it in this command: `git rebase --exec 'git commit --amend --no-edit -n -s' -i HASH`. This will sign every commit from most recent to right before the HASH.
    1. You will probably need to do a force push `git push -f` if you had previously pushed unsigned commits to remote.
