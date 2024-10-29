from typing import Any, Dict, List, Type, TypeVar, Union, cast

from attrs import define as _attrs_define
from attrs import field as _attrs_field

from ..models.app_category import AppCategory
from ..models.app_state import AppState
from ..types import UNSET, Unset

T = TypeVar("T", bound="UpdateAppHttpRequestBody")


@_attrs_define
class UpdateAppHttpRequestBody:
    """
    Attributes:
        category (Union[AppCategory, None, Unset]):
        description (Union[None, Unset, str]):
        name (Union[None, Unset, str]):
        state (Union[AppState, None, Unset]):
        tags (Union[None, Unset, str]):
        url (Union[None, Unset, str]):
    """

    category: Union[AppCategory, None, Unset] = UNSET
    description: Union[None, Unset, str] = UNSET
    name: Union[None, Unset, str] = UNSET
    state: Union[AppState, None, Unset] = UNSET
    tags: Union[None, Unset, str] = UNSET
    url: Union[None, Unset, str] = UNSET
    additional_properties: Dict[str, Any] = _attrs_field(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        category: Union[None, Unset, str]
        if isinstance(self.category, Unset):
            category = UNSET
        elif isinstance(self.category, AppCategory):
            category = self.category.value
        else:
            category = self.category

        description: Union[None, Unset, str]
        if isinstance(self.description, Unset):
            description = UNSET
        else:
            description = self.description

        name: Union[None, Unset, str]
        if isinstance(self.name, Unset):
            name = UNSET
        else:
            name = self.name

        state: Union[None, Unset, str]
        if isinstance(self.state, Unset):
            state = UNSET
        elif isinstance(self.state, AppState):
            state = self.state.value
        else:
            state = self.state

        tags: Union[None, Unset, str]
        if isinstance(self.tags, Unset):
            tags = UNSET
        else:
            tags = self.tags

        url: Union[None, Unset, str]
        if isinstance(self.url, Unset):
            url = UNSET
        else:
            url = self.url

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update({})
        if category is not UNSET:
            field_dict["category"] = category
        if description is not UNSET:
            field_dict["description"] = description
        if name is not UNSET:
            field_dict["name"] = name
        if state is not UNSET:
            field_dict["state"] = state
        if tags is not UNSET:
            field_dict["tags"] = tags
        if url is not UNSET:
            field_dict["url"] = url

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()

        def _parse_category(data: object) -> Union[AppCategory, None, Unset]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            try:
                if not isinstance(data, str):
                    raise TypeError()
                category_type_1 = AppCategory(data)

                return category_type_1
            except:  # noqa: E722
                pass
            return cast(Union[AppCategory, None, Unset], data)

        category = _parse_category(d.pop("category", UNSET))

        def _parse_description(data: object) -> Union[None, Unset, str]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            return cast(Union[None, Unset, str], data)

        description = _parse_description(d.pop("description", UNSET))

        def _parse_name(data: object) -> Union[None, Unset, str]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            return cast(Union[None, Unset, str], data)

        name = _parse_name(d.pop("name", UNSET))

        def _parse_state(data: object) -> Union[AppState, None, Unset]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            try:
                if not isinstance(data, str):
                    raise TypeError()
                state_type_1 = AppState(data)

                return state_type_1
            except:  # noqa: E722
                pass
            return cast(Union[AppState, None, Unset], data)

        state = _parse_state(d.pop("state", UNSET))

        def _parse_tags(data: object) -> Union[None, Unset, str]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            return cast(Union[None, Unset, str], data)

        tags = _parse_tags(d.pop("tags", UNSET))

        def _parse_url(data: object) -> Union[None, Unset, str]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            return cast(Union[None, Unset, str], data)

        url = _parse_url(d.pop("url", UNSET))

        update_app_http_request_body = cls(
            category=category,
            description=description,
            name=name,
            state=state,
            tags=tags,
            url=url,
        )

        update_app_http_request_body.additional_properties = d
        return update_app_http_request_body

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
