import createClient from 'openapi-fetch'
import type { paths } from "./types/schema"


const client = createClient<paths>( {
    baseUrl: 'http://localhost:9000'
} )

client.GET( '/v1.0/apps' )