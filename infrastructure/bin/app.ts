#!/usr/bin/env node
import 'source-map-support/register'
import * as cdk from 'aws-cdk-lib'
import { NotionToriiStack } from '../lib/NotionToriiStack'
import 'dotenv/config'

const app = new cdk.App()



new NotionToriiStack( app, 'NotionToriiStack', {
    env: {
        account: '698852667105',
        region: 'us-east-1'
    }
} )

