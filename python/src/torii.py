from interfaces import (
    AppCategory,
    CreateAppRequestBody,
    NotionDbItem,
    ToriiApp,
    UpdateAppRequestBody,
)
from utils import build_map_from_property
from torii_client import ToriiClient
from logger import logger


def add_notion_app_to_torii(missing: list[NotionDbItem], torii: ToriiClient):
    """
    Adds missing Notion apps to the Torii system, checking if they are already known before creating custom entries.

    This function iterates over a list of Notion apps that are missing in Torii and attempts to add each one.
    If an app is already known to Torii, it is added using its existing ID. If not, a custom app entry is created
    with additional metadata from Notion. Errors during the creation of custom apps are logged without interrupting
    the process.

    Args:
        missing (list[NotionDbItem]): A list of Notion app items to add to Torii.
        torii (ToriiClient): An instance of the Torii client used to interact with the Torii API.

    Raises:
        Exception: Logs and skips over any exceptions raised during the creation of custom apps in Torii.
    """
    # List known Torii apps and build a map based on the name
    known_apps = torii.list_known_apps()
    known_apps_map = build_map_from_property(known_apps, lambda app: app.name)

    for app in missing:
        app_name = app.properties.Name.title[0].plain_text

        # Check if the app is already known in Torii
        maybe_known = known_apps_map.get(app_name)

        if maybe_known:
            logger.info(f"App {app_name} missing in Torii but is known. Adding...")

            # Add known app by ID
            torii.add_app(maybe_known.id)
        else:
            logger.info(f"Custom App {app_name} missing in Torii. Adding...")

            try:
                # Add a custom app to Torii

                app_data = CreateAppRequestBody.model_validate(
                    {
                        "state": app.properties.State.select.name,
                        "category": AppCategory.OTHER,
                        "name": app_name,
                        "description": (
                            app.properties.Description.rich_text[0].plain_text
                            if app.properties.Description
                            else ""
                        ),
                        "url": app.properties.URL.url,
                        "tags": None,
                    }
                )

                torii.create_app(app_data)
            except Exception as err:
                logger.error(f"Unable to add app {app_name} to Torii", {"error": err})
                continue


def update_torii_from_notion(
    need_updates: list[NotionDbItem],
    existing_apps_map: dict[str, ToriiApp],
    torii: ToriiClient,
):
    """
    Updates existing Torii applications based on data from Notion, if discrepancies are found.

    This function iterates over Notion applications that have been identified as needing updates in Torii.
    It compares each Notion app's properties (description, state, URL) with the corresponding Torii app, and if
    any values differ, it sends an update request to Torii. Categories, tags, and names are not modified in
    this update process as these fields are not managed in Notion.

    Args:
        need_updates (list[NotionDbItem]): A list of Notion app items that need to be updated in Torii.
        existing_apps_map (dict[str, ToriiApp]): A mapping of Torii app names to `ToriiApp` instances,
            used to find the corresponding Torii app for each Notion app.
        torii (ToriiClient): An instance of the Torii client used to interact with the Torii API.

    Raises:
        None: Errors are logged and skipped without interrupting the loop, ensuring that all items in
        `need_updates` are processed even if individual updates encounter errors.
    """
    for item in need_updates:
        item_name = item.properties.Name.title[0].plain_text

        # This should be impossible since the original maps were used to identify differences, but
        # this makes the type checker satisfied
        torii_app = existing_apps_map.get(item_name)
        if not torii_app:
            logger.error(f"App {item_name} not found in existing Torii apps map.")
            continue

        notion_description = (
            item.properties.Description.rich_text[0].plain_text
            if item.properties.Description and item.properties.Description.rich_text
            else None
        )
        notion_state = item.properties.State.select.name
        notion_url = item.properties.URL.url

        # Compare and decide if an update is needed
        if (
            torii_app.description == notion_description
            and torii_app.state == notion_state
            and torii_app.url == notion_url
        ):
            logger.debug("All values are same, continuing")
            continue

        logger.info(f"Updating app {item_name} in Torii")

        # Category and Tags are not currently defined in Notion, and we aren't allowing the name
        # to be changed
        update_body = UpdateAppRequestBody.model_validate(
            {
                "description": notion_description,
                "state": notion_state,
                "url": notion_url,
                "name": None,
                "category": None,
                "tags": None,
            }
        )

        # Update the app in Torii
        torii.update_app(torii_app.id, update_body)
