# swc-plugin-transform-cjs-imports

Have you ever had an error like this while trying to use ESM?

```
import { promisifyAll } from "bluebird";
         ^^^^^^^^^^^^
SyntaxError: Named export 'promisifyAll' not found. The requested module 'bluebird' is a CommonJS module, which may not support all module.exports as named exports.
CommonJS modules can always be imported via the default export, for example using:

import pkg from 'bluebird';
const { promisifyAll } = pkg;
```

We did too, and didn't want to update every line in our codebase to support this, so we made this plugin.

Install the plugin:

```
yarn add swc-plugin-transform-cjs-imports
```

Specify the list of modules that should be imported as CommonJS, and `swc-plugin-transform-cjs-imports` will automatically turn code such as:

```
import { promisifyAll } from 'bluebird'
```

into:

```
import _bluebird from 'bluebird'

const { promisifyAll } = _bluebird
```

This plugin only has one parameter, "modules", which is a list of the CommonJS modules you want to transform. 

For example, use this in your .swcrc to transform "bluebird":

```
{
    ...
    "experimental": {
      "keepImportAssertions": true,
      "plugins": [
        ["swc-plugin-transform-cjs-imports", {
          "modules": [
            "bluebird"
          ]
        }]
      ]
    }
    ...
}
```

Enjoy!
