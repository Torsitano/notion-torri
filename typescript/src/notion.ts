import { DatabaseObjectResponse, PageObjectResponse, PartialDatabaseObjectResponse, PartialPageObjectResponse } from '@notionhq/client/build/src/api-endpoints'
import { logger, notion } from './handler'
import { CreateNotionApp, NotionItemState, PageObjectResponseWithAppProps, ToriiApp } from './interfaces'


type QueryItems = PageObjectResponse | PartialPageObjectResponse | PartialDatabaseObjectResponse | DatabaseObjectResponse

// Notion can return 4 different response types, for simplicity we currently only care
// about the PageObjectResponse. We are then augmenting that with the known properties
export function asPage( item: QueryItems ) {
    if ( "object" in item && item.object === "page" ) {
        return item as PageObjectResponseWithAppProps
    }

    return undefined
}

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

export async function addAppInNotion( app: CreateNotionApp ) {

    try {
        const response = await notion.pages.create( {
            parent: { database_id: process.env.DATABASE_ID! },
            properties: {
                Name: {
                    title: [
                        {
                            text: {
                                content: app.name
                            }
                        }
                    ]
                },
                URL: {
                    url: app.url
                },
                State: {
                    select: {
                        name: app.state
                    }
                },
                Description: {
                    rich_text: [
                        {
                            text: {
                                content: app.description ?? ''
                            }
                        }
                    ]
                }
            }
        } )

        logger.info( "Successfully added app to Notion database", { response } )
        return response
    } catch ( err ) {
        logger.error( "Failed to add app to Notion database", { error: err } )
        throw err
    }
}


export async function addToriiAppToNotion( missing: ToriiApp[] ) {
    for ( let item of missing ) {
        logger.info( `App ${item.name} missing in Notion. Adding...` )
        try {
            // `as NotionItemState` is because one is an enum and other is string literals
            await addAppInNotion( {
                name: item.name,
                state: item.state as NotionItemState,
                url: item.url,
                description: item.description ?? ''
            } )
        } catch ( err ) {
            logger.error( "Failed to add Torii app to Notion database", { error: err } )
            continue
        }
    }
}