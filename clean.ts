import { $, file } from 'bun';
// check every folder for a cargo.toml
// check if they have been changed less then a month ago
// then delete every folder named target inside there
// add all these numbers up and also display some of the names etc.
// also do the same for node_modules => look for a package json

const filename = 'foo.js; rm -rf /';

// as a file()
await $`grep 'bun' * > ${file('output.txt')}`;
