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

export async function updateAppInNotion( id: string, appData: Partial<CreateNotionApp> ) {
    try {
        const response = await notion.pages.update( {
            page_id: id,
            properties: {
                ...( appData.name && {
                    Name: {
                        title: [
                            {
                                text: {
                                    content: appData.name
                                }
                            }
                        ]
                    }
                } ),
                ...( appData.url && {
                    URL: {
                        url: appData.url
                    }
                } ),
                ...( appData.state && {
                    State: {
                        select: {
                            name: appData.state
                        }
                    }
                } ),
                ...( appData.description && {
                    Description: {
                        rich_text: [
                            {
                                text: {
                                    content: appData.description
                                }
                            }
                        ]
                    }
                } )
            }
        } )

        logger.info( "Successfully updated app in Notion database", { response } )
        return response
    } catch ( error ) {
        logger.error( "Failed to update app in Notion database", { error } )
        throw error
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

export async function updateNotionFromTorii( needUpdates: ToriiApp[], existingAppsMap: Map<string, PageObjectResponseWithAppProps> ) {

    for ( let item of needUpdates ) {
        // We're asserting that this app does exist in the map, because otherwise it wouldn't have been 
        // passed to this function for updates. If this were ever something production, better error handling
        // would be needed here
        const notionApp = existingAppsMap.get( item.name )!

        const notionDescription = notionApp.properties.Description?.rich_text[ 0 ]?.plain_text
        const notionState = notionApp.properties.State.select.name
        const notionUrl = notionApp.properties.URL.url

        // If we only go off of last updated time without checking properties for differences,
        // it will constantly look like the other source has more up to date information since
        // a PUT/POST of unchanged info will still likely generate a new updated at timestamp
        if (
            item.description == notionDescription
            && item.state == notionState
            && item.url == notionUrl
        ) {
            logger.debug( 'All values are same, continuing' )
            continue
        }

        logger.info( `Updating app ${item.name} in Notion` )

        await updateAppInNotion( notionApp.id, {
            description: item.description ?? '',
            state: item.state as NotionItemState,
            url: item.url
        } )
    }

}