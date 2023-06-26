import fs from 'fs'
import lib from 'lib'
import _ from 'lodash'
import { something } from 'lib2'

fs.writeFileSync(something(), JSON.stringify(_.filter([], _.identity)))