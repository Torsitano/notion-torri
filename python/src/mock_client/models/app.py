import datetime
from typing import Any, Dict, List, Type, TypeVar, Union, cast

from attrs import define as _attrs_define
from attrs import field as _attrs_field
from dateutil.parser import isoparse

from ..models.app_category import AppCategory
from ..models.app_state import AppState
from ..types import UNSET, Unset

T = TypeVar("T", bound="App")


@_attrs_define
class App:
    """
    Attributes:
        added_by (str):
        category (AppCategory):
        creation_time (datetime.datetime):
        id (int):
        is_custom (bool):
        is_hidden (bool):
        name (str):
        primary_owner (str):
        state (AppState):
        url (str):
        description (Union[None, Unset, str]):
        image_url (Union[None, Unset, str]):
        last_usage_time (Union[None, Unset, datetime.datetime]):
        sources (Union[None, Unset, str]):
        tags (Union[None, Unset, str]):
        users (Union[None, Unset, str]):
    """

    added_by: str
    category: AppCategory
    creation_time: datetime.datetime
    id: int
    is_custom: bool
    is_hidden: bool
    name: str
    primary_owner: str
    state: AppState
    url: str
    description: Union[None, Unset, str] = UNSET
    image_url: Union[None, Unset, str] = UNSET
    last_usage_time: Union[None, Unset, datetime.datetime] = UNSET
    sources: Union[None, Unset, str] = UNSET
    tags: Union[None, Unset, str] = UNSET
    users: Union[None, Unset, str] = UNSET
    additional_properties: Dict[str, Any] = _attrs_field(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        added_by = self.added_by

        category = self.category.value

        creation_time = self.creation_time.isoformat()

        id = self.id

        is_custom = self.is_custom

        is_hidden = self.is_hidden

        name = self.name

        primary_owner = self.primary_owner

        state = self.state.value

        url = self.url

        description: Union[None, Unset, str]
        if isinstance(self.description, Unset):
            description = UNSET
        else:
            description = self.description

        image_url: Union[None, Unset, str]
        if isinstance(self.image_url, Unset):
            image_url = UNSET
        else:
            image_url = self.image_url

        last_usage_time: Union[None, Unset, str]
        if isinstance(self.last_usage_time, Unset):
            last_usage_time = UNSET
        elif isinstance(self.last_usage_time, datetime.datetime):
            last_usage_time = self.last_usage_time.isoformat()
        else:
            last_usage_time = self.last_usage_time

        sources: Union[None, Unset, str]
        if isinstance(self.sources, Unset):
            sources = UNSET
        else:
            sources = self.sources

        tags: Union[None, Unset, str]
        if isinstance(self.tags, Unset):
            tags = UNSET
        else:
            tags = self.tags

        users: Union[None, Unset, str]
        if isinstance(self.users, Unset):
            users = UNSET
        else:
            users = self.users

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "addedBy": added_by,
                "category": category,
                "creationTime": creation_time,
                "id": id,
                "isCustom": is_custom,
                "isHidden": is_hidden,
                "name": name,
                "primaryOwner": primary_owner,
                "state": state,
                "url": url,
            }
        )
        if description is not UNSET:
            field_dict["description"] = description
        if image_url is not UNSET:
            field_dict["imageUrl"] = image_url
        if last_usage_time is not UNSET:
            field_dict["lastUsageTime"] = last_usage_time
        if sources is not UNSET:
            field_dict["sources"] = sources
        if tags is not UNSET:
            field_dict["tags"] = tags
        if users is not UNSET:
            field_dict["users"] = users

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        added_by = d.pop("addedBy")

        category = AppCategory(d.pop("category"))

        creation_time = isoparse(d.pop("creationTime"))

        id = d.pop("id")

        is_custom = d.pop("isCustom")

        is_hidden = d.pop("isHidden")

        name = d.pop("name")

        primary_owner = d.pop("primaryOwner")

        state = AppState(d.pop("state"))

        url = d.pop("url")

        def _parse_description(data: object) -> Union[None, Unset, str]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            return cast(Union[None, Unset, str], data)

        description = _parse_description(d.pop("description", UNSET))

        def _parse_image_url(data: object) -> Union[None, Unset, str]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            return cast(Union[None, Unset, str], data)

        image_url = _parse_image_url(d.pop("imageUrl", UNSET))

        def _parse_last_usage_time(data: object) -> Union[None, Unset, datetime.datetime]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            try:
                if not isinstance(data, str):
                    raise TypeError()
                last_usage_time_type_0 = isoparse(data)

                return last_usage_time_type_0
            except:  # noqa: E722
                pass
            return cast(Union[None, Unset, datetime.datetime], data)

        last_usage_time = _parse_last_usage_time(d.pop("lastUsageTime", UNSET))

        def _parse_sources(data: object) -> Union[None, Unset, str]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            return cast(Union[None, Unset, str], data)

        sources = _parse_sources(d.pop("sources", UNSET))

        def _parse_tags(data: object) -> Union[None, Unset, str]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            return cast(Union[None, Unset, str], data)

        tags = _parse_tags(d.pop("tags", UNSET))

        def _parse_users(data: object) -> Union[None, Unset, str]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            return cast(Union[None, Unset, str], data)

        users = _parse_users(d.pop("users", UNSET))

        app = cls(
            added_by=added_by,
            category=category,
            creation_time=creation_time,
            id=id,
            is_custom=is_custom,
            is_hidden=is_hidden,
            name=name,
            primary_owner=primary_owner,
            state=state,
            url=url,
            description=description,
            image_url=image_url,
            last_usage_time=last_usage_time,
            sources=sources,
            tags=tags,
            users=users,
        )

        app.additional_properties = d
        return app

    @property
    def additional_keys(self) -> List[str]:
        return list(self.additional_properties.keys())

    def __getitem__(self, key: str) -> Any:
        return self.additional_properties[key]

    def __setitem__(self, key: str, value: Any) -> None:
        self.additional_properties[key] = value

    def __delitem__(self, key: str) -> None:
        del self.additional_properties[key]

    def __contains__(self, key: str) -> bool:
        return key in self.additional_properties
