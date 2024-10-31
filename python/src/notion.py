from typing import Any
from notion_client import Client as NotionClient
from logger import logger
from interfaces import NotionDbItem, CreateNotionApp, UpdateNotionApp, ToriiApp
from pydantic import TypeAdapter


def list_notion_apps(notion: NotionClient, database_id: str):
    """
    Retrieves and validates a list of apps stored in a specified Notion database.

    This function queries the Notion API for entries in a given database and validates
    the returned data against the `NotionDbItem` model using Pydantic. It is particularly
    useful for ensuring that each item in the result conforms to the expected structure
    before further processing.

    Args:
        notion (NotionClient): An instance of the Notion client to interact with the Notion API.
        database_id (str): The ID of the Notion database to query for app entries.

    Returns:
        list[NotionDbItem]: A list of validated Notion database items, each structured as a `NotionDbItem` object.

    Raises:
        pydantic.ValidationError: If the items returned by the Notion API do not conform to the `NotionDbItem` model.
    """

    # type is defined incorrectly as `Any | Awaitable[Any]` since the client can support both
    response: dict[str, Any] = notion.databases.query(database_id=database_id)  # type: ignore

    logger.debug("Notion list Apps response", {"response": response})

    ta = TypeAdapter(list[NotionDbItem])
    db_items = ta.validate_python(response["results"])

    logger.debug("Notion DB Items", {"items": db_items})

    return db_items


def add_app_in_notion(
    notion: NotionClient, app: CreateNotionApp, database_id: str
) -> dict[str, Any]:
    """
    Adds an app entry to the specified Notion database.

    Args:
        notion (NotionClient): The Notion client instance.
        app (CreateNotionApp): The app data to add to the Notion database.
        database_id (str): The ID of the Notion database.

    Returns:
        dict: The response from Notion API after creating the page.
    """
    try:
        # type is defined incorrectly as `Any | Awaitable[Any]` since the client can support both
        response: dict[str, Any] = notion.pages.create(  # type: ignore
            parent={"database_id": database_id},
            properties={
                "Name": {"title": [{"text": {"content": app.name}}]},
                "URL": {"url": app.url},
                "State": {"select": {"name": str(app.state)}},
                "Description": {
                    "rich_text": [{"text": {"content": app.description or ""}}]
                },
            },
        )

        logger.info("Successfully added app to Notion database", {"response": response})
        return response
    except Exception as err:
        logger.error("Failed to add app to Notion database", {"error": err})
        raise


def update_app_in_notion(
    notion: NotionClient, app_data: UpdateNotionApp, page_id: str
) -> dict[str, Any]:
    """
    Updates an app entry in the specified Notion database.

    Args:
        notion (NotionClient): The Notion client instance.
        app_data (CreateNotionApp): Partial app data with fields to be updated.
        page_id (str): The ID of the Database item.

    Returns:
        dict: The response from Notion API after updating the page.
    """
    try:
        properties = {}

        if app_data.url:
            properties["URL"] = {"url": app_data.url}
        if app_data.state:
            properties["State"] = {"select": {"name": str(app_data.state)}}
        if app_data.description:
            properties["Description"] = {
                "rich_text": [{"text": {"content": app_data.description}}]
            }

        # type is defined incorrectly as `Any | Awaitable[Any]` since the client can support both
        response: dict[str, Any] = notion.pages.update(  # type: ignore
            page_id=page_id, properties=properties
        )

        logger.info(
            "Successfully updated app in Notion database", {"response": response}
        )
        return response
    except Exception as error:
        logger.error("Failed to update app in Notion database", {"error": error})
        raise


def add_torii_app_to_notion(
    notion: NotionClient, missing: list[ToriiApp], database_id: str
) -> None:
    """
    Adds a list of missing Torii apps to the specified Notion database.

    Args:
        notion (NotionClient): The Notion client instance.
        missing (list[ToriiApp]): A list of apps missing in the Notion database.
        database_id (str): The ID of the Notion database.
    """
    for item in missing:
        logger.info(f"App {item.name} missing in Notion. Adding...")

        try:
            app_data = CreateNotionApp.model_validate(
                {
                    "name": item.name,
                    "state": item.state,
                    "url": item.url,
                    "description": item.description or "",
                }
            )
            add_app_in_notion(notion, app_data, database_id)

        except Exception as err:
            logger.error("Failed to add Torii app to Notion database", {"error": err})
            continue


def update_notion_from_torii(
    need_updates: list[ToriiApp],
    existing_apps_map: dict[str, NotionDbItem],
    notion: NotionClient,
) -> None:
    """
    Updates Notion records based on changes from Torii apps.

    Args:
        need_updates (List[ToriiApp]): List of Torii apps that need updates in Notion.
        existing_apps_map (Dict[str, PageObjectResponseWithAppProps]): Mapping of app names to existing Notion app properties.
        notion (NotionClient): The Notion client instance for API interactions.
    """

    for item in need_updates:
        # Assume that the item exists in the map, otherwise handle appropriately
        notion_app = existing_apps_map.get(item.name)

        if not notion_app:
            logger.error(f"App '{item.name}' not found in Notion map. Skipping update.")
            continue

        # Retrieve current Notion app properties
        notion_description = (
            notion_app.properties.Description.rich_text[0].plain_text
            if notion_app.properties.Description
            and notion_app.properties.Description.rich_text
            else ""
        )
        notion_state = notion_app.properties.State.select.name
        notion_url = notion_app.properties.URL.url

        # Check if an update is necessary
        if (
            item.description == notion_description
            and item.state == notion_state
            and item.url == notion_url
        ):
            logger.debug("All values are same for app '%s', continuing.", item.name)
            continue

        logger.info("Updating app '%s' in Notion", item.name)

        update_data = UpdateNotionApp.model_validate(
            {
                "description": item.description or "",
                "state": item.state,
                "url": item.url,
            }
        )

        # Update the app in Notion
        update_app_in_notion(notion, update_data, notion_app.id)
