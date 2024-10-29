from mock_client import AuthenticatedClient

client = AuthenticatedClient(base_url = "http://localhost:9000", token = 'test')


print(client)