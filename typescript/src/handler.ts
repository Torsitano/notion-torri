import dotenv from 'dotenv'
import { ScheduledEvent, Context } from 'aws-lambda'
import { Logger } from '@aws-lambda-powertools/logger'
import { buildNotionMap, buildToriiMap, notionClient, toriiClient } from './utils'
import { Client as NotionClient } from '@notionhq/client'
import { addToriiAppToNotion, listNotionApps } from './notion'
import { addNotionAppToTorii, listToriiApps, updateToriiFromNotion } from './torii'
import type { paths } from "./types/schema"
import { Client } from 'openapi-fetch'
import { ToriiApp } from './interfaces'

dotenv.config()
export const logger = new Logger( { serviceName: 'notion-torii' } )

// Instantiating clients requires a call to Secrets Manager and we're not using top level await
// We define them here so we can use them globally without passing them around
export let torii: Client<paths, `${string}/${string}`>
export let notion: NotionClient



export async function handler( _event?: ScheduledEvent, _context?: Context ) {

    logger.info( 'STARTED!' )

    // If this Lambda was something getting a lot of activity, we'd want to avoid this so we
    // didn't have a call to Secrets Manager every instantiation, which eliminates benefits of
    // environment reuse. In our case, this will only be excuted as part of a scheduled job,
    // so it's not a relevant concern.
    torii = await toriiClient()
    notion = await notionClient()

    logger.info( 'Clients created' )

    const notionApps = await listNotionApps()
    logger.info( 'Notion apps retrieved' )
    const toriiApps: ToriiApp[] = await listToriiApps()
    logger.info( 'Torii apps retrieved' )


    const notionMap = buildNotionMap( notionApps )
    const toriiMap = buildToriiMap( toriiApps )

    logger.info( 'Maps built' )


    // We technically iterate through each map twice for update/missing, and it would be more efficient 
    // to build a left/rightlist in each function instead of calling it twice, but the number of items
    // we're dealing with is small enough that it's not worth the extra complexity
    const missingInTorii = getMissing( notionMap, toriiMap )
    const missingInNotion = getMissing( toriiMap, notionMap )

    // Look at this super cool function that ALSO offers guaranteed static type safety. It's impossible to
    // pass a key (second and fourth arguments) that isn't in the relevant object, you're certain to only
    // be able to use a property that exists
    const updateInTorii = getUpdateNeeded( notionMap, 'last_edited_time', toriiMap, 'lastUpdatedAt' )
    //@ts-ignore
    const updateInNotion = getUpdateNeeded( toriiMap, 'lastUpdatedAt', notionMap, 'last_edited_time' )


    await addNotionAppToTorii( missingInTorii )
    await updateToriiFromNotion( updateInTorii )
    await addToriiAppToNotion( missingInNotion )

}



// Accepts two maps, expecting that keys will identify what items are common.
// Anything in the `left` map but not in the `right` map will be returned as an array of `L`
function getMissing<L, R>( left: Map<string, L>, right: Map<string, R> ) {
    const missing: L[] = []

    for ( let key of left.keys() ) {
        if ( !right.get( key ) ) {
            missing.push( left.get( key )! )
            logger.info( `Missing: ${key}` )
        }
    }

    return missing
}

// Accepts two maps, expecting that keys will identify what items are common.
// Anything in the `left` that is also in the `right` map but updated more recently
// will be returned as an array of strings
function getUpdateNeeded<L, LK extends keyof L, R, RK extends keyof R>(
    left: Map<string, L>,
    leftKey: LK,
    right: Map<string, R>,
    rightKey: RK
) {
    const needUpdate: L[] = []

    for ( let item of left ) {
        const rightItem = right.get( item[ 0 ] )

        if ( rightItem ) {
            const leftUpdatedTime = Date.parse( String( item[ 1 ][ leftKey ] ) )
            const rightUpdatedTime = Date.parse( String( rightItem[ rightKey ] ) )

            if ( leftUpdatedTime > rightUpdatedTime ) {
                needUpdate.push( item[ 1 ] )
            }
        }
    }

    return needUpdate
}



handler().then( () => {
    console.log( 'finished' )
} )