# from mock_client import AuthenticatedClient

# client = AuthenticatedClient(base_url = "http://localhost:9000", token = 'test')


# print(client)
import boto3
from mypy_boto3_lambda import LambdaClient

lambda_client: LambdaClient = boto3.client('lambda')

lambda_client.invoke(FunctionName='NotionToriiTs')