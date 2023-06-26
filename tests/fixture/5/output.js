import fs from 'fs'
import lib from 'lib'
import _ from 'lodash'
import cjsModule0 from 'lib2'
import cjsModule1 from 'lib3'

const {something}=cjsModule0
const Lib3=cjsModule1
const {fetchImportantThing}=cjsModule1

fs.writeFileSync(something(), JSON.stringify(_.filter(fetchImportantThing(), _.identity)))

Lib3.somethingElseImportant()