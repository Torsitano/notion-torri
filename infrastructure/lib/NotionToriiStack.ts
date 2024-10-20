
import { RemovalPolicy, Stack, StackProps } from 'aws-cdk-lib'
import { Construct } from 'constructs'
import { RustFunction } from 'cargo-lambda-cdk'
import path from 'path'
import { AttributeType, Billing, TableEncryptionV2, TableV2 } from 'aws-cdk-lib/aws-dynamodb'
import { HttpApi, HttpMethod } from 'aws-cdk-lib/aws-apigatewayv2'
import { HttpLambdaIntegration } from 'aws-cdk-lib/aws-apigatewayv2-integrations'
import { Tracing } from 'aws-cdk-lib/aws-lambda'


export class NotionToriiStack extends Stack {
    //@ts-ignore
    constructor ( scope: Construct, id: string, props?: StackProps ) {
        super( scope, id, props )

        const mockFunction = new RustFunction( this, 'ToriiMock', {
            manifestPath: path.join( __dirname, '../mock/' ),
            environment: {},
            memorySize: 128,
            tracing: Tracing.ACTIVE,
            bundling: {

            }
        } )

        const apiGw = new HttpApi( this, 'ApiGw', {
            apiName: 'RustApiv2',
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

        toriiTable.grantReadWriteData( mockFunction.role! )

    }
}
