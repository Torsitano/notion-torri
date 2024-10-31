import createClient from 'openapi-fetch'
import type { paths } from "./types/schema"
import { Client as NotionClient } from '@notionhq/client'
import { logger } from './handler'
import { GetSecretValueCommand, SecretsManagerClient } from '@aws-sdk/client-secrets-manager'
import { listNotionApps } from './notion'
import { listToriiApps } from './torii'

/**
 * Retrieves a secret value from AWS Secrets Manager.
 *
 * @param {string} secretName - The name of the secret to retrieve.
 * @returns {Promise<string>} - The secret value as a string.
 * @throws {Error} - If the secret value is missing or retrieval fails.
 */
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

/**
 * Creates and returns an instance of the Torii API client, authenticated using a secret from AWS Secrets Manager.
 *
 * @returns {Promise<ReturnType<typeof createClient<paths>>>} - An authenticated Torii client instance.
 * @throws {Error} - If retrieving the secret or creating the client fails.
 */
export async function toriiClient() {
    const toriiUrl = process.env.TORII_URL ?? 'http://localhost:9000'
    const toriiSecret = process.env.TORII_SECRET ?? 'torii-api-key'
    const toriiApiKey = await getSecret( toriiSecret )

    logger.debug( `Torii URL: ${toriiUrl}` )
    const torii = createClient<paths>( {
        baseUrl: toriiUrl,
        headers: {
            Authorization: `Bearer ${toriiApiKey}`,
        }
    } )

    return torii
}

/**
 * Creates and returns an instance of the Notion API client, authenticated using a secret from AWS Secrets Manager.
 *
 * @returns {Promise<NotionClient>} - An authenticated Notion client instance.
 * @throws {Error} - If retrieving the secret or creating the client fails.
 */
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

/**
 * Builds a map from an iterable collection of objects, where each object is mapped by a property value.
 *
 * @template T - The type of the iterable items.
 * @param {Iterable<T>} iterable - An iterable collection of objects.
 * @param {(item: T) => string} getKey - A function that extracts the key from each item.
 * @returns {Map<string, T>} - A map with keys derived from item properties.
 * @throws {Error} - If the key extracted from any item is not a string.
 */
export function buildMapFromProperty<T>(
    iterable: Iterable<T>,
    getKey: ( item: T ) => string
): Map<string, T> {
    const map = new Map<string, T>()

    for ( let item of iterable ) {
        const key = getKey( item )
        map.set( key, item )
    }

    return map
}



// The input types of these are tied to the return types of other functions. This means 
// that if those were to ever change, like what would happen if we flattened the object
// returned by Notion into something more reasonable, this would automagically throw an
// error from the change if the specified property wasn't there. This prevents refactoring
// mistakes and makes code changes significantly more safe

/**
 * Builds a map of Notion apps, using the name of each app as the key.
 *
 * @param {Awaited<ReturnType<typeof listNotionApps>>} notionApps - The list of Notion apps.
 * @returns {Map<string, typeof notionApps[0]>} - A map with app names as keys.
 * @throws {Error} - If any app does not have a name or if any key is not a string.
 */
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

/**
 * Builds a map of Torii apps, using the name of each app as the key.
 *
 * @param {Awaited<ReturnType<typeof listToriiApps>>} toriiApps - The list of Torii apps.
 * @returns {Map<string, typeof toriiApps[0]>} - A map with app names as keys.
 * @throws {Error} - If any key is not a string.
 */
export function buildToriiMap( toriiApps: Awaited<ReturnType<typeof listToriiApps>> ) {
    return buildMapFromProperty( toriiApps, ( item ) => {
        return item.name
    } )
}