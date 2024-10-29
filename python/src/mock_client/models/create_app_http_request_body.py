from typing import Any, Dict, List, Type, TypeVar, Union, cast

from attrs import define as _attrs_define
from attrs import field as _attrs_field

from ..models.app_category import AppCategory
from ..models.app_state import AppState
from ..types import UNSET, Unset

T = TypeVar("T", bound="CreateAppHttpRequestBody")


@_attrs_define
class CreateAppHttpRequestBody:
    """
    Attributes:
        category (AppCategory):
        description (str):
        name (str):
        state (AppState):
        url (str):
        tags (Union[None, Unset, str]):
    """

    category: AppCategory
    description: str
    name: str
    state: AppState
    url: str
    tags: Union[None, Unset, str] = UNSET
    additional_properties: Dict[str, Any] = _attrs_field(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        category = self.category.value

        description = self.description

        name = self.name

        state = self.state.value

        url = self.url

        tags: Union[None, Unset, str]
        if isinstance(self.tags, Unset):
            tags = UNSET
        else:
            tags = self.tags

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "category": category,
                "description": description,
                "name": name,
                "state": state,
                "url": url,
            }
        )
        if tags is not UNSET:
            field_dict["tags"] = tags

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        category = AppCategory(d.pop("category"))

        description = d.pop("description")

        name = d.pop("name")

        state = AppState(d.pop("state"))

        url = d.pop("url")

        def _parse_tags(data: object) -> Union[None, Unset, str]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            return cast(Union[None, Unset, str], data)

        tags = _parse_tags(d.pop("tags", UNSET))

        create_app_http_request_body = cls(
            category=category,
            description=description,
            name=name,
            state=state,
            url=url,
            tags=tags,
        )

        create_app_http_request_body.additional_properties = d
        return create_app_http_request_body

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
