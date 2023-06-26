import fs from 'fs'
import lib from 'lib'
import _ from 'lodash'
import cjsModule0 from 'lib2'

const {something}=cjsModule0

fs.writeFileSync(something(), JSON.stringify(_.filter([], _.identity)))