// This isn't super well written for local testing since we're not doing top level
// await/async. So to test code locally, just run `ts-node local.ts`, and it'll work

import { handler } from './handler'

handler().then( () => {
    console.log( 'finished' )
} )