from pydantic import BaseModel, Field
from dataclasses import dataclass
from enum import Enum
from typing import Any, List, Optional
import datetime


class AppState(Enum):
    CLOSED = "Closed"
    SANCTIONED = "Sanctioned"
    DISCOVERED = "Discovered"

    def __str__(self):
        return self.value  # Return custom string representation


class AppCategory(Enum):
    OPERATIONS = "Operations"
    SALES_AND_MARKETING = "Sales & Marketing"
    DEVELOPER_TOOLS = "Developer Tools"
    DESIGN = "Design"
    PROJECT_MANAGEMENT = "Project Management"
    CUSTOMER_SUCCESS = "Customer Success"
    HUMAN_RESOURCES = "Human Resources"
    IT_AND_SECURITY = "IT & Security"
    FINANCE = "Finance"
    PRODUCTIVITY = "Productivity"
    ANALYTICS_AND_BI = "Analytics & BI"
    OTHER = "Other"

    def __str__(self):
        return self.value  # Return custom string representation


@dataclass
class Text:
    content: str
    link: Optional[str]


@dataclass
class Annotations:
    bold: bool
    italic: bool
    strikethrough: bool
    underline: bool
    code: bool
    color: str


@dataclass
class RichText:
    type: str
    text: Text
    annotations: Annotations
    plain_text: str
    href: Optional[str]


@dataclass
class Description:
    id: str
    type: str
    rich_text: List[RichText]


@dataclass
class Name:
    id: str
    type: str
    title: List[RichText]


@dataclass
class StateSelect:
    id: str
    name: AppState
    color: str


@dataclass
class State:
    id: str
    type: str
    select: StateSelect


@dataclass
class Url:
    id: str
    type: str
    url: str


@dataclass
class NotionAppProperties:
    URL: Url
    State: State
    Description: Optional[Description]
    Name: Name


@dataclass
class NotionUser:
    object: str
    id: str


@dataclass
class NotionParent:
    type: str
    database_id: str


@dataclass
class NotionDbItem(BaseModel):
    object: str
    id: str
    created_time: datetime.datetime
    last_edited_time: datetime.datetime
    created_by: NotionUser
    last_edited_by: NotionUser
    cover: Any | None
    icon: Any | None
    parent: NotionParent
    archived: bool
    in_trash: bool
    properties: NotionAppProperties
    url: str
    public_url: str | None


@dataclass
class ToriiApp(BaseModel):
    id: int
    is_hidden: bool = Field(alias="isHidden")
    name: str
    state: AppState
    url: str
    image_url: Optional[str] = Field(alias="imageUrl")
    category: AppCategory
    users: Optional[str] = None
    description: Optional[str] = None
    tags: Optional[str] = None
    creation_time: datetime.datetime = Field(alias="creationTime")
    last_updated_at: datetime.datetime = Field(alias="lastUpdatedAt")
    last_usage_time: Optional[datetime.datetime] = Field(alias="lastUsageTime")
    added_by: str = Field(alias="addedBy")
    primary_owner: str = Field(alias="primaryOwner")
    is_custom: bool = Field(alias="isCustom")
    sources: Optional[str] = None

    class Config:
        populate_by_name = True  # Allows using both snake_case and camelCase
        # use_enum_values = True


@dataclass
class CreateAppRequestBody(BaseModel):
    name: str
    url: str
    state: AppState
    category: AppCategory
    description: str | None = None
    tags: str | None = None


@dataclass
class UpdateAppRequestBody(BaseModel):
    name: str | None = None
    url: str | None = None
    state: AppState | None = None
    category: AppCategory | None = None
    description: str | None = None
    tags: str | None = None


@dataclass
class AddAppRequestBody(BaseModel):
    id: str


@dataclass
class KnownApp(BaseModel):
    category: AppCategory
    id: int
    name: str
    url: str


class CreateNotionApp(BaseModel):
    name: str
    url: str
    state: AppState
    description: str | None = None


class UpdateNotionApp(BaseModel):
    url: str | None = None
    state: AppState | None = None
    description: str | None = None
