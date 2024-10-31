import requests
from pydantic import TypeAdapter
from interfaces import ToriiApp, CreateAppRequestBody, KnownApp, UpdateAppRequestBody

from logger import logger


class ToriiClient:
    """
    A client for interacting with the Torii API.

    Attributes:
        base_url (str): The base URL of the Torii API.
        headers (dict[str, str]): Default headers for authentication and content type.
        http (requests.Session): Shared request session for reuse
    """

    base_url: str
    headers: dict[str, str]
    http: requests.Session

    def __init__(self, base_url: str, auth_token: str) -> None:
        """
        Initializes the ToriiClient with a base URL and bearer token.

        Args:
            base_url (str): The base URL of the Torii API.
            auth_token (str): The bearer token for API access. Only provide the token value,
            `Bearer` should not be included
        """
        self.base_url: str = base_url
        self.headers: dict[str, str] = {
            "Authorization": f"Bearer {auth_token}",
            "Content-Type": "application/json",
        }

        self.http = requests.Session()
        self.http.headers.update(self.headers)

    def list_apps(self) -> list[ToriiApp]:
        """
        Retrieves a list of all apps.

        Returns:
            list[App]: A list of apps from the Torii API.
        """
        response = self.http.get(f"{self.base_url}/v1.0/apps")
        response.raise_for_status()
        logger.debug(f"Response", {"response", response})

        ta = TypeAdapter(list[ToriiApp])
        apps = ta.validate_python(response.json())

        return apps

    def add_app(self, app_id: int) -> ToriiApp:
        """
        Adds an existing app to the Torii platform.

        Args:
            app_id (int): The ID of the app to add.

        Returns:
            App: The added app object.
        """
        response = self.http.post(f"{self.base_url}/v1.0/apps", json={"idApp": app_id})
        response.raise_for_status()
        logger.debug(f"Response: {response.json()}")

        app = ToriiApp.model_validate(response.json())

        return app

    def create_app(self, app_data: CreateAppRequestBody) -> ToriiApp:
        """
        Creates a new custom app on the Torii platform.

        Args:
            app_data (CreateAppRequestBody): The data for the app to create.

        Returns:
            App: The created app object.
        """
        response = self.http.post(
            f"{self.base_url}/v1.0/apps/custom",
            data=app_data.model_dump_json(),
        )
        response.raise_for_status()
        logger.debug(f"Response: {response.json()}")

        app = ToriiApp.model_validate(response.json())

        return app

    def list_known_apps(self) -> list[KnownApp]:
        """
        Retrieves a list of all known apps on the Torii platform. Use to identify what
        can be passed to `add_app()`

        Returns:
            list[KnownApp]: A list of known apps in Torii.
        """
        response = self.http.get(f"{self.base_url}/v1.0/apps/known")
        response.raise_for_status()
        logger.debug(f"Response: {response.json()}")

        ta = TypeAdapter(list[KnownApp])
        apps = ta.validate_python(response.json())

        return apps

    def search_apps(self, query: str) -> list[ToriiApp]:
        """
        Searches for apps with a name containing the query string.

        Args:
            query (str): The search query.

        Returns:
            list[App]: A list of apps matching the query.
        """
        response = self.http.get(
            f"{self.base_url}/v1.0/apps/search", params={"query": query}
        )
        response.raise_for_status()
        logger.debug(f"Response: {response.json()}")

        ta = TypeAdapter(list[ToriiApp])
        apps = ta.validate_python(response.json())

        return apps

    def get_app(self, app_id: int) -> ToriiApp:
        """
        Retrieves and App using it's ID.

        Args:
            app_id (int): The ID of the app to retrieve.

        Returns:
            App: The app object.
        """
        response = self.http.get(f"{self.base_url}/v1.0/apps/{app_id}")
        response.raise_for_status()
        logger.debug(f"Response: {response.json()}")

        app = ToriiApp.model_validate(response.json())

        return app

    def update_app(self, app_id: int, app_data: UpdateAppRequestBody) -> ToriiApp:
        """
        Updates an app.

        Args:
            app_id (int): The ID of the app to update.
            app_data (UpdateAppRequestBody): The updated data for the app.

        Returns:
            App: The updated app object.
        """
        response = self.http.put(
            f"{self.base_url}/v1.0/apps/{app_id}", data=app_data.model_dump_json()
        )
        response.raise_for_status()
        logger.debug(f"Response: {response.json()}")

        app = ToriiApp.model_validate(response.json())

        return app

    def delete_app(self, app_id: int) -> str:
        """
        Deletes an app by its ID.

        Args:
            app_id (int): The ID of the app to delete.

        Returns:
            str: Confirmation message indicating successful deletion.
        """
        response = self.http.delete(f"{self.base_url}/v1.0/apps/{app_id}")
        response.raise_for_status()
        logger.debug(f"Response: {response.json()}")
        return f"App with ID {app_id} deleted successfully."
