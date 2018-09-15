
# VCX NodeJS Wrapper

## Contribution Guide

Make sure you have these packages installed:

* StandardJS
* Typescript
* TSLint


Also this has a dependency on:
* libvcx debian
Because it creates a symlink (/usr/lib/libvcx.so) 

Run this commands before submitting your PR:

```
npm run lint
```

## Documentation:
 Run these commands:
```
npm install
npm ci
npm run doc-gen
```
* A directory will be created locally `./docs` which contains an `index.html` file which can be used to navigate the generated documents.

