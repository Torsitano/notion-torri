from enum import Enum


class AppCategory(str, Enum):
    ANALYTICSANDBI = "AnalyticsAndBi"
    CUSTOMERSUCCESS = "CustomerSuccess"
    DESIGN = "Design"
    DEVELOPERTOOLS = "DeveloperTools"
    FINANCE = "Finance"
    HUMANRESOURCES = "HumanResources"
    ITANDSECURITY = "ItAndSecurity"
    OPERATIONS = "Operations"
    OTHER = "Other"
    PRODUCTIVITY = "Productivity"
    PROJECTMANAGEMENT = "ProjectManagement"
    SALESANDMARKETING = "SalesAndMarketing"

    def __str__(self) -> str:
        return str(self.value)
