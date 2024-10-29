import { logger, torii } from './handler'
import { PageObjectResponseWithAppProps, ToriiApp } from './interfaces'
import { components } from './types/schema'
import { buildMapFromProperty } from './utils'

// The error handling on this page is very verbose, but given that it's a super basic SDK
// generated from the OpenAPI definition it's sufficient for this use case. A generic function
// could be created that accepts arbitrary endpoints, but it's not worth addressing the type
// complexity at the moment. Also a few extra lines for error handling is worth the
// static type safety guaranteed by using this.

export async function listToriiApps(): Promise<ToriiApp[]> {
    const { data, error, response } = await torii.GET( '/v1.0/apps' )

    logger.debug( 'Torii response', { data: response } )

    if ( !( response.status >= 200 && response.status < 300 ) ) {
        logger.error( 'Error listing apps from Torii:', { errorMessage: response.statusText } )
        throw new Error( response.statusText )
    }

    if ( error ) {
        logger.error( 'Error:', { errorMessage: error } )
        throw new Error( error )
    }

    // The API returns an empty list if nothing is found, so this is
    // guaranteed to not be `undefined`
    return data!
}

export async function createToriiApp( app: components[ 'schemas' ][ 'CreateAppHttpRequestBody' ] ) {

    const { data, error, response } = await torii.POST( '/v1.0/apps/custom', {
        body: app
    } )

    if ( !( response.status >= 200 && response.status < 300 ) ) {
        logger.error( 'Error creating custom app in Torii:', { errorMessage: response.statusText } )
        logger.error( 'App:', { app } )
        throw new Error( response.statusText )
    }

    if ( error ) {
        logger.error( 'Error:', { errorMessage: error } )
        throw new Error( error )
    }

    // The API returns the app if successful, if it's not an error is thrown above
    return data!
}

export async function addToriiApp( id: number ) {

    const { data, error, response } = await torii.POST( '/v1.0/apps', {
        body: {
            idApp: id
        }
    } )

    if ( !( response.status >= 200 && response.status < 300 ) ) {
        logger.error( 'Error adding known app in Torii:', { errorMessage: response.statusText } )
        throw new Error( response.statusText )
    }

    if ( error ) {
        logger.error( 'Error:', { errorMessage: error } )
        throw new Error( error )
    }

    // The API returns the app if successful, if it's not an error is thrown above
    return data!
}

export async function listKnownToriiApps() {
    const { data, error, response } = await torii.GET( '/v1.0/apps/known' )

    logger.debug( 'Torii response', { data: response } )

    if ( !( response.status >= 200 && response.status < 300 ) ) {
        logger.error( 'Error listing known apps from Torii:', { errorMessage: response.statusText } )
        throw new Error( response.statusText )
    }

    if ( error ) {
        logger.error( 'Error:', { errorMessage: error } )
        throw new Error( error )
    }

    // Known apps is static and should be impossible to fail the request except for auth failure,
    // which would be caught above
    return data!
}

export async function updateToriiApp( id: number, updateBody: components[ "schemas" ][ "UpdateAppHttpRequestBody" ] ) {
    const { data, error, response } = await torii.PUT( '/v1.0/apps/{id}', {
        params: {
            path: {
                id
            }
        },
        body: updateBody
    } )

    if ( !( response.status >= 200 && response.status < 300 ) ) {
        logger.error( 'Error listing known apps from Torii:', { errorMessage: response.statusText } )
        throw new Error( response.statusText )
    }

    if ( error ) {
        logger.error( 'Error:', { errorMessage: error } )
        throw new Error( error )
    }

    // The API returns the app if successful, if it's not an error is thrown above
    return data!
}

export async function addNotionAppToTorii( missing: PageObjectResponseWithAppProps[] ) {

    const knownApps = await listKnownToriiApps()
    const knownAppsMap = buildMapFromProperty( knownApps, ( app ) => {
        return app.name
    } )

    for ( let app of missing ) {

        const appName = app.properties.Name.title[ 0 ].plain_text

        // If the app is included in the list of "known apps" in Torii, this will be the required properties of 
        // the app. If it's not known, it will be undefined, which means a custom app will need to be created.
        const maybeKnown = knownAppsMap.get( appName )

        if ( maybeKnown ) {
            logger.info( `App ${appName} missing in Torii but is known. Adding...` )

            await addToriiApp( maybeKnown.id )
        } else {

            logger.info( `Custom App ${appName} missing in Torii. Adding...` )

            // Currently just using other since category is not defined for Notion
            try {
                await createToriiApp( {
                    state: app.properties.State.select.name,
                    category: 'Other',
                    name: appName,
                    description: app.properties.Description?.rich_text[ 0 ]?.plain_text,
                    url: app.properties.URL.url
                } )
            } catch ( err ) {
                logger.error( `Unable to add app ${appName} to Torii`, { error: err } )
                // Error message logged in create app function
                continue
            }
        }
    }
}

export async function updateToriiFromNotion( needUpdates: PageObjectResponseWithAppProps[] ) {
    const existingApps = await listToriiApps()
    const existingAppsMap = buildMapFromProperty( existingApps, ( app ) => {
        return app.name
    } )

    for ( let item of needUpdates ) {
        const itemName = item.properties.Name.title[ 0 ].plain_text

        // We're asserting that this app does exist in the map, because otherwise it wouldn't have been 
        // passed to this function for updates. If this were ever something production, better error handling
        // would be needed here
        const toriiApp = existingAppsMap.get( itemName )!

        const notionDescription = item.properties.Description?.rich_text[ 0 ]?.plain_text
        const notionState = item.properties.State.select.name
        const notionUrl = item.properties.URL.url

        // If we only go off of last updated time without checking properties for differences,
        // it will constantly look like the other source has more up to date information since
        // a PUT/POST of unchanged info will still likely generate a new updated at timestamp
        if (
            toriiApp.description == notionDescription
            && toriiApp.state == notionState
            && toriiApp.url == notionUrl
        ) {
            logger.debug( 'All values are same, continuing' )
            continue
        }

        logger.info( `Updating app ${itemName} in Torii` )

        // Several of these are possibly undefined, but the updateBody interface expects
        // potentially undefined arguments. Anything undefined will just be a no-op
        const updateBody: components[ "schemas" ][ "UpdateAppHttpRequestBody" ] = {
            description: notionDescription,
            state: notionState,
            url: notionUrl
        }

        const toriiAppId = toriiApp.id

        await updateToriiApp( toriiAppId, updateBody )
    }



}