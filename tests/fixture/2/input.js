import _ from 'lodash'
import {a, b} from 'lib'
import {createRef} from 'react'

console.log('the value of a is', a)

export const x = createRef(b)

export const version = _.unescape('1.2.3')