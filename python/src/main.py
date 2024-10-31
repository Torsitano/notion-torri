from datetime import datetime
from aws_lambda_powertools.utilities.typing import LambdaContext
from aws_lambda_powertools.utilities.data_classes import EventBridgeEvent
from notion import (
    add_torii_app_to_notion,
    list_notion_apps,
    update_notion_from_torii,
)
from torii import add_notion_app_to_torii, update_torii_from_notion
from utils import build_notion_map, build_torii_map, notion_client, torii_client
import os
from logger import logger
from typing import TypeVar, cast

T = TypeVar("T")
U = TypeVar("U")

# Setup clients
torii = torii_client()
notion = notion_client()

database_id = os.getenv("DATABASE_ID", "12a290a45b058079a76cefd082dbcf7b")


@logger.inject_lambda_context
def handler(event: EventBridgeEvent, context: LambdaContext):

    logger.info("STARTED")
    logger.debug("Lambda Payload", {"event": event}, {"context": context})

    notion_apps = list_notion_apps(notion, database_id)
    torii_apps = torii.list_apps()
    logger.info("Apps retrieved")

    notion_map = build_notion_map(notion_apps)
    torii_map = build_torii_map(torii_apps)

    # Get items that are missing
    missing_in_torii = get_missing(notion_map, torii_map)
    missing_in_notion = get_missing(torii_map, notion_map)

    # Get items that need to be updated
    update_in_torii = get_update_needed(
        notion_map, "last_edited_time", torii_map, "last_updated_at"
    )
    update_in_notion = get_update_needed(
        torii_map, "last_updated_at", notion_map, "last_edited_time"
    )

    add_notion_app_to_torii(missing_in_torii, torii)
    update_torii_from_notion(update_in_torii, torii_map, torii)

    add_torii_app_to_notion(notion, missing_in_notion, database_id)
    update_notion_from_torii(update_in_notion, notion_map, notion)


def get_missing(left: dict[str, T], right: dict[str, U]) -> list[T]:
    """
    Identifies items in the `left` dictionary that are missing in the `right` dictionary based on keys.

    This function takes two dictionaries, `left` and `right`, and compares them by keys. It returns
    a list of values from `left` where the corresponding keys do not exist in `right`. Each missing
    key is logged.

    Args:
        left (dict[str, T]): The source dictionary containing items to check.
        right (dict[str, U]): The dictionary against which `left` is checked for missing keys.

    Returns:
        list[T]: A list of values from `left` where the keys are missing in `right`.

    Note:
        This function assumes that `left` and `right` have comparable keys
    """
    missing: list[T] = []

    for key in left:
        if key not in right:
            missing.append(left[key])
            logger.info("Missing: %s", key)
    return missing


def get_update_needed(
    left: dict[str, T], left_key: str, right: dict[str, U], right_key: str
) -> list[T]:
    """
    Finds items in the `left` dictionary that exist in the `right` dictionary but
    have been updated more recently than the corresponding items in `right`.

    Args:
        left (dict[str, T]): The left dictionary containing items to compare.
        left_key (str): The key in the left items for the last updated time.
        right (dict[str, U]): The right dictionary containing items to compare against.
        right_key (str): The key in the right items for the last updated time.

    Returns:
        list[T]: A list of items from the left dictionary that need updating.
    """
    need_update: list[T] = []

    for key, left_item in left.items():
        right_item = cast(U, right.get(key))

        if right_item:

            # We are asserting that these are datetime with cast. This will
            # be a runtime error if someone passed a key that had a different
            # type of value
            left_updated_time = cast(datetime, getattr(left_item, left_key))
            right_updated_time = cast(datetime, getattr(right_item, right_key))

            if left_updated_time > right_updated_time:
                need_update.append(left_item)

    return need_update
