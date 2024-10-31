
import { Duration, RemovalPolicy, Stack, StackProps } from 'aws-cdk-lib'
import { Construct } from 'constructs'
import { RustFunction } from 'cargo-lambda-cdk'
import path from 'path'
import { AttributeType, Billing, ProjectionType, TableEncryptionV2, TableV2 } from 'aws-cdk-lib/aws-dynamodb'
import { HttpApi, HttpMethod } from 'aws-cdk-lib/aws-apigatewayv2'
import { HttpLambdaIntegration } from 'aws-cdk-lib/aws-apigatewayv2-integrations'
import { Runtime, Tracing } from 'aws-cdk-lib/aws-lambda'
import { Secret } from 'aws-cdk-lib/aws-secretsmanager'
import { NodejsFunction } from 'aws-cdk-lib/aws-lambda-nodejs'


export class NotionToriiStack extends Stack {
    constructor ( scope: Construct, id: string, props?: StackProps ) {
        super( scope, id, props )

        const apiGw = new HttpApi( this, 'ApiGw', {
            apiName: 'ToriiMock',
        } )

        const mockFunction = new RustFunction( this, 'ToriiMock', {
            manifestPath: path.join( __dirname, '../../mock/' ),
            environment: {
                APP_ENVIRONMENT: 'dev',
                RUST_LOG: 'info,aws_config::meta::region=off'
            },
            memorySize: 128,
            tracing: Tracing.ACTIVE,
        } )

        const tsFunction = new NodejsFunction( this, 'TypeScriptfunction', {
            functionName: 'NotionToriiTs',
            entry: '../typescript/src/handler.ts',
            runtime: Runtime.NODEJS_LATEST,
            timeout: Duration.seconds( 15 ),
            environment: {
                TORII_URL: apiGw.url!,
                DATABASE_ID: '12a290a45b058079a76cefd082dbcf7b',
            }
        } )

        apiGw.addRoutes( {
            path: '/{proxy+}',
            methods: [ HttpMethod.ANY ],
            integration: new HttpLambdaIntegration(
                'apiIntegration',
                mockFunction
            )
        } )

        const toriiTable = new TableV2( this, 'ToriiTable', {
            partitionKey: {
                name: 'pk',
                type: AttributeType.STRING
            },
            encryption: TableEncryptionV2.awsManagedKey(),
            tableName: 'torii-table',
            removalPolicy: RemovalPolicy.DESTROY,
            billing: Billing.onDemand(),
        } )

        toriiTable.addGlobalSecondaryIndex( {
            indexName: 'name_index',
            partitionKey: {
                name: 'name',
                type: AttributeType.STRING
            },
            projectionType: ProjectionType.ALL
        } )

        toriiTable.grantReadWriteData( mockFunction.role! )

        const notionApiSecret = new Secret( this, 'NotionSecret', {
            secretName: 'notion-api-key'
        } )

        const toriiApiSecret = new Secret( this, 'ToriiSecret', {
            secretName: 'torii-api-key'
        } )

        toriiApiSecret.grantRead( mockFunction )
        notionApiSecret.grantRead( tsFunction )

        mockFunction.addEnvironment( 'TORII_SECRET', toriiApiSecret.secretName )
        tsFunction.addEnvironment( 'TORII_SECRET', toriiApiSecret.secretName )
        tsFunction.addEnvironment( 'NOTION_SECRET', notionApiSecret.secretName )


    }
}
