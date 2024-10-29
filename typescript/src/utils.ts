import createClient from 'openapi-fetch'
import type { paths } from "./types/schema"
import { Client as NotionClient } from '@notionhq/client'
import { logger } from './handler'
import { GetSecretValueCommand, SecretsManagerClient } from '@aws-sdk/client-secrets-manager'
import { listNotionApps } from './notion'
import { listToriiApps } from './torii'

export async function getSecret( secretName: string ) {
    const secretClient = new SecretsManagerClient( {
        region: process.env.REGION ?? 'us-east-1'
    } )

    let secretValue = await secretClient.send( new GetSecretValueCommand( {
        SecretId: secretName
    } ) )


    if ( secretValue.SecretString ) {
        return secretValue.SecretString
    }
    else if ( secretValue.SecretBinary ) {
        const decodedSecret = Buffer.from( secretValue.SecretBinary ).toString( 'utf8' )
        return decodedSecret
    }
    else {
        throw new Error( 'Missing value' )
    }
}


export async function toriiClient() {
    const torii_url = process.env.TORII_URL ?? 'http://localhost:9000'
    const toriiSecret = process.env.NOTION_SECRET ?? 'notion-api-key'
    const toriiApiKey = await getSecret( toriiSecret )

    logger.debug( `Torii URL: ${torii_url}` )
    const torii = createClient<paths>( {
        baseUrl: torii_url,
        headers: {
            Authorization: `Bearer ${toriiApiKey}`
        }
    } )

    return torii
}

export async function notionClient() {

    const notionSecret = process.env.NOTION_SECRET ?? 'notion-api-key'
    const notionApiKey = await getSecret( notionSecret )

    const notion = new NotionClient( {
        auth: notionApiKey
    } )

    return notion
}

// Builds a map where the key is a value from a property of each item
// T must be an iterable where each object contains the string provided for key
// The additional complextity of accepting a closure is required because of the nesting
// done in the Notion response. Alternatively we could flatten that object, but it would
// create additional complexity when we look at syncing back to Notion, so it's easier
// to work with the objects in their normal form
export function buildMapFromProperty<T>(
    iterable: Iterable<T>,
    getKey: ( item: T ) => string
): Map<string, T> {
    const map = new Map<string, T>()

    for ( let item of iterable ) {
        const key = getKey( item )
        if ( typeof key === "string" ) {
            map.set( key, item )
        } else {
            logger.error( 'Unexpected Key on item', { item } )
            throw new Error( "The extracted key must be a string" )
        }
    }

    return map
}



// The input types of these are tied to the return types of other functions. This means 
// that if those were to ever change, like what would happen if we flattened the object
// returned by Notion into something more reasonable, this would automagically throw an
// error from the change if the specified property wasn't there. This prevents refactoring
// mistakes and makes code changes significantly more safe
export function buildNotionMap( notionApps: Awaited<ReturnType<typeof listNotionApps>> ) {
    return buildMapFromProperty( notionApps, ( item ) => {
        logger.debug( 'Item in BuildMapFromProperty', { builditem: item } )
        const title = item.properties.Name.title

        if ( title.length == 0 ) {
            logger.error( 'Notion app missing name', { app: item } )
        }

        return title[ 0 ].plain_text
    } )
}

export function buildToriiMap( toriiApps: Awaited<ReturnType<typeof listToriiApps>> ) {
    return buildMapFromProperty( toriiApps, ( item ) => {
        return item.name
    } )
}