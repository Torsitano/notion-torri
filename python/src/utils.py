import boto3
import os
from mypy_boto3_secretsmanager import SecretsManagerClient
from notion_client import Client as NotionClient
from interfaces import NotionDbItem, ToriiApp
from torii_client import ToriiClient
from logger import logger
from typing import Callable, Iterable, TypeVar


def get_secret(secret_name: str):
    """
    Retrieves a secret value from AWS Secrets Manager.

    Args:
        secret_name (str): The name of the secret to retrieve.

    Returns:
        str: The secret value as a string.

    Raises:
        botocore.exceptions.ClientError: If there's an error retrieving the secret from AWS Secrets Manager.
    """
    client: SecretsManagerClient = boto3.client(  # type: ignore - client is partially unknown due to overload
        "secretsmanager", region_name=os.getenv("REGION", "us-east-1")
    )
    response = client.get_secret_value(SecretId=secret_name)
    return response["SecretString"]


def notion_client():
    """
    Creates and returns an instance of the Notion client, authenticated using a secret from AWS Secrets Manager.

    The Notion API key is retrieved from Secrets Manager using the secret name defined in the environment
    variable `NOTION_SECRET`.If this environment variable is not set, a default value `notion-api-key` is used.

    Returns:
        NotionClient: An instance of the Notion client.
    """
    notion_secret = os.getenv("NOTION_SECRET", "notion-api-key")
    notion_api_key = get_secret(notion_secret)
    notion = NotionClient(auth=notion_api_key)

    return notion


def torii_client():
    """
    Creates and returns an instance of the Torii client, authenticated using a token from AWS Secrets Manager.

    The Torii API URL and API key are retrieved from environment variables `TORII_URL` and `TORII_SECRET`, respectively.
    If `TORII_URL` is not set, a default value of `http://localhost:9000` is used.

    Returns:
        ToriiClient: An instance of the Torii client.
    """
    torii_url = os.getenv("TORII_URL", "http://localhost:9000")
    torii_secret = os.getenv("TORII_SECRET", "torii-api-key")
    torii_api_key = get_secret(torii_secret)

    logger.debug(f"Torii URL: {torii_url}")

    torii = ToriiClient(torii_url, torii_api_key)

    return torii


T = TypeVar("T")


def build_map_from_property(
    iterable: Iterable[T], get_key: Callable[[T], str]
) -> dict[str, T]:
    """
    Builds a dictionary from an iterable of objects using a specified property as the key.

    Args:
        iterable (Iterable[T]): The collection of items.
        get_key (Callable[[T], str]): A function to extract the key from each item.

    Returns:
        Dict[str, T]: A dictionary mapping the key to each item.
    """
    map_dict: dict[str, T] = {}
    for item in iterable:
        key = get_key(item)
        map_dict[key] = item

    return map_dict


def build_notion_map(notion_apps: list[NotionDbItem]) -> dict[str, NotionDbItem]:
    return build_map_from_property(
        notion_apps,
        lambda item: item.properties.Name.title[0].plain_text,
    )


def build_torii_map(torii_apps: list[ToriiApp]) -> dict[str, ToriiApp]:
    return build_map_from_property(torii_apps, lambda item: item.name)
