import fs from 'fs'
import lib from 'lib'
import _ from 'lodash'
import { something } from 'lib2'
import { fetchImportantThing } from 'lib3'

fs.writeFileSync(something(), JSON.stringify(_.filter(fetchImportantThing(), _.identity)))