import { PageObjectResponse } from '@notionhq/client/build/src/api-endpoints'
import { components } from './types/schema'

// Any property can actually be null in Notion, but since `url`, `state`, and `name`
// are all required properties for Torii we are going to insist they are present here,
// since we control the data. For real applications this would need to handle null
// values better
export interface NotionAppProperties {
    URL: URL
    State: State
    Description: Description | null
    Name: Name
}

export interface Description {
    id: string
    type: string
    rich_text: RichText[]
}

export interface RichText {
    type: string
    text: Text
    annotations: Annotations
    plain_text: string
    href: null
}

export interface Annotations {
    bold: boolean
    italic: boolean
    strikethrough: boolean
    underline: boolean
    code: boolean
    color: string
}

export interface Text {
    content: string
    link: null
}

export interface Name {
    id: string
    type: string
    title: RichText[]
}

export interface State {
    id: string
    type: string
    select: Select
}

export interface Select {
    id: string
    name: NotionItemState
    color: string
}

export interface URL {
    id: string
    type: string
    url: string
}

export enum NotionItemState {
    CLOSED = 'Closed',
    SANCTIONED = 'Sanctioned',
    DISCOVERED = 'Discovered'
}

// We are excluding the built-in definition of `properties` and replacing it with our own from above
// This is because Notion's SDK can't accurately represent the type of our data, but we know
// the schema of the DB
export interface PageObjectResponseWithAppProps extends Omit<PageObjectResponse, 'properties'> {
    properties: NotionAppProperties
}

// The inferred type is obnoxious to pass around, so we redefine it here
export type ToriiApp = components[ "schemas" ][ "App" ]