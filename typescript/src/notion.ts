import { DatabaseObjectResponse, PageObjectResponse, PartialDatabaseObjectResponse, PartialPageObjectResponse } from '@notionhq/client/build/src/api-endpoints'
import { logger, notion } from './handler'
import { PageObjectResponseWithAppProps } from './interfaces'


export async function listNotionApps() {
    const response = await notion.databases.query( {
        database_id: process.env.DATABASE_ID!
    } )

    const apps = response.results.map( ( item ) => {
        let page = asPage( item )
        if ( !page ) {
            throw new Error( 'This script is only made to handle full page responses' )
        }

        return page
    } )

    logger.debug( "Apps returned from Notion", { data: apps } )

    return apps
}

type QueryItems = PageObjectResponse | PartialPageObjectResponse | PartialDatabaseObjectResponse | DatabaseObjectResponse

// Notion can return 4 different response types, for simplicity we currently only care
// about the PageObjectResponse. We are then augmenting that with the known properties
export function asPage( item: QueryItems ) {
    if ( "object" in item && item.object === "page" ) {
        return item as PageObjectResponseWithAppProps
    }

    return undefined
}