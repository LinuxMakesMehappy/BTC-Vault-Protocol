#!/usr/bin/env python3
"""
Dashboard Configuration System

This module provides configuration for frontend display settings,
localization, and dashboard customization options.
"""

import os
from typing import Dict, List, Optional, Union, Any
from dataclasses import dataclass, field
from enum import Enum
import json

class Language(Enum):
    """Supported languages"""
    ENGLISH = "en"
    SPANISH = "es"
    FRENCH = "fr"
    GERMAN = "de"
    CHINESE = "zh"
    JAPANESE = "ja"
    KOREAN = "ko"
    PORTUGUESE = "pt"
    RUSSIAN = "ru"
    ARABIC = "ar"

class Theme(Enum):
    """Dashboard theme options"""
    LIGHT = "light"
    DARK = "dark"
    AUTO = "auto"

class Currency(Enum):
    """Display currency options"""
    USD = "USD"
    EUR = "EUR"
    GBP = "GBP"
    JPY = "JPY"
    CNY = "CNY"
    KRW = "KRW"

class ChartType(Enum):
    """Chart display types"""
    LINE = "line"
    BAR = "bar"
    PIE = "pie"
    DONUT = "donut"
    AREA = "area"

class RefreshInterval(Enum):
    """Data refresh intervals"""
    REAL_TIME = 1  # 1 second
    FAST = 5  # 5 seconds
    NORMAL = 30  # 30 seconds
    SLOW = 60  # 1 minute
    MANUAL = 0  # Manual refresh only

@dataclass
class LocalizationConfig:
    """Localization settings"""
    default_language: Language = Language.ENGLISH
    supported_languages: List[Language] = field(default_factory=lambda: [
        Language.ENGLISH, Language.SPANISH, Language.FRENCH, Language.GERMAN,
        Language.CHINESE, Language.JAPANESE, Language.KOREAN
    ])
    date_format: str = "YYYY-MM-DD"
    time_format: str = "HH:mm:ss"
    number_format: str = "en-US"  # Locale for number formatting
    currency_symbol_position: str = "before"  # "before" or "after"
    decimal_places: int = 2
    thousands_separator: str = ","
    decimal_separator: str = "."
    rtl_languages: List[Language] = field(default_factory=lambda: [Language.ARABIC])

@dataclass
class DisplaySettings:
    """Display and UI settings"""
    theme: Theme = Theme.AUTO
    primary_currency: Currency = Currency.USD
    show_portfolio_value: bool = True
    show_24h_change: bool = True
    show_percentage_change: bool = True
    show_absolute_change: bool = True
    compact_mode: bool = False
    show_tooltips: bool = True
    animations_enabled: bool = True
    sound_notifications: bool = False
    desktop_notifications: bool = True
    high_contrast_mode: bool = False
    font_size: str = "medium"  # small, medium, large
    sidebar_collapsed: bool = False

@dataclass
class ChartConfig:
    """Chart display configuration"""
    default_chart_type: ChartType = ChartType.LINE
    show_grid: bool = True
    show_legend: bool = True
    show_volume: bool = True
    candlestick_colors: Dict[str, str] = field(default_factory=lambda: {
        "up": "#00C851",
        "down": "#FF4444",
        "neutral": "#33B5E5"
    })
    line_colors: List[str] = field(default_factory=lambda: [
        "#007bff", "#28a745", "#ffc107", "#dc3545", "#6f42c1", "#fd7e14"
    ])
    background_color: str = "transparent"
    grid_color: str = "#e0e0e0"
    text_color: str = "#333333"
    height: int = 400
    responsive: bool = True

@dataclass
class WidgetConfig:
    """Dashboard widget configuration"""
    widget_id: str
    title: str
    type: str  # "chart", "metric", "table", "alert"
    position: Dict[str, int] = field(default_factory=lambda: {"x": 0, "y": 0, "w": 4, "h": 4})
    visible: bool = True
    refresh_interval: RefreshInterval = RefreshInterval.NORMAL
    data_source: str = ""
    settings: Dict[str, Any] = field(default_factory=dict)
    permissions: List[str] = field(default_factory=list)

@dataclass
class DashboardLayout:
    """Dashboard layout configuration"""
    layout_id: str
    name: str
    description: str = ""
    widgets: List[WidgetConfig] = field(default_factory=list)
    columns: int = 12
    row_height: int = 60
    margin: List[int] = field(default_factory=lambda: [10, 10])
    container_padding: List[int] = field(default_factory=lambda: [10, 10])
    is_default: bool = False
    is_public: bool = False
    created_by: str = ""
    created_at: Optional[str] = None
    updated_at: Optional[str] = None

@dataclass
class AlertConfig:
    """Alert and notification configuration"""
    price_alerts_enabled: bool = True
    portfolio_alerts_enabled: bool = True
    system_alerts_enabled: bool = True
    email_notifications: bool = False
    push_notifications: bool = True
    alert_sound: str = "default"
    alert_volume: float = 0.5
    price_change_threshold: float = 5.0  # percentage
    portfolio_change_threshold: float = 10.0  # percentage
    alert_cooldown: int = 300  # seconds between similar alerts

@dataclass
class PerformanceConfig:
    """Performance and optimization settings"""
    lazy_loading: bool = True
    virtual_scrolling: bool = True
    cache_duration: int = 300  # seconds
    max_chart_points: int = 1000
    compression_enabled: bool = True
    prefetch_data: bool = True
    batch_requests: bool = True
    request_timeout: int = 30  # seconds

@dataclass
class SecurityConfig:
    """Security and privacy settings"""
    session_timeout: int = 3600  # seconds
    auto_logout: bool = True
    mask_sensitive_data: bool = False
    audit_logging: bool = True
    two_factor_required: bool = False
    ip_whitelist: List[str] = field(default_factory=list)
    rate_limiting: bool = True
    csrf_protection: bool = True

@dataclass
class DashboardConfig:
    """Main dashboard configuration"""
    localization: LocalizationConfig = field(default_factory=LocalizationConfig)
    display: DisplaySettings = field(default_factory=DisplaySettings)
    charts: ChartConfig = field(default_factory=ChartConfig)
    layouts: List[DashboardLayout] = field(default_factory=list)
    alerts: AlertConfig = field(default_factory=AlertConfig)
    performance: PerformanceConfig = field(default_factory=PerformanceConfig)
    security: SecurityConfig = field(default_factory=SecurityConfig)
    api_endpoints: Dict[str, str] = field(default_factory=dict)
    feature_flags: Dict[str, bool] = field(default_factory=dict)
    custom_css: str = ""
    version: str = "1.0.0"

# Default Widget Configurations
DEFAULT_WIDGETS = [
    WidgetConfig(
        widget_id="portfolio_overview",
        title="Portfolio Overview",
        type="metric",
        position={"x": 0, "y": 0, "w": 6, "h": 3},
        refresh_interval=RefreshInterval.NORMAL,
        data_source="portfolio_api",
        settings={
            "show_total_value": True,
            "show_24h_change": True,
            "show_allocation": True
        }
    ),
    WidgetConfig(
        widget_id="price_chart",
        title="Price Chart",
        type="chart",
        position={"x": 6, "y": 0, "w": 6, "h": 6},
        refresh_interval=RefreshInterval.FAST,
        data_source="price_api",
        settings={
            "chart_type": "line",
            "time_range": "24h",
            "show_volume": True
        }
    ),
    WidgetConfig(
        widget_id="staking_rewards",
        title="Staking Rewards",
        type="table",
        position={"x": 0, "y": 3, "w": 6, "h": 4},
        refresh_interval=RefreshInterval.NORMAL,
        data_source="staking_api",
        settings={
            "show_apy": True,
            "show_rewards": True,
            "show_validators": True
        }
    ),
    WidgetConfig(
        widget_id="recent_transactions",
        title="Recent Transactions",
        type="table",
        position={"x": 0, "y": 7, "w": 12, "h": 4},
        refresh_interval=RefreshInterval.NORMAL,
        data_source="transaction_api",
        settings={
            "max_rows": 10,
            "show_status": True,
            "show_amounts": True
        }
    )
]

# Default Dashboard Layout
DEFAULT_LAYOUT = DashboardLayout(
    layout_id="default",
    name="Default Dashboard",
    description="Standard dashboard layout with essential widgets",
    widgets=DEFAULT_WIDGETS,
    is_default=True,
    is_public=True
)

# Default API Endpoints
DEFAULT_API_ENDPOINTS = {
    "portfolio_api": "/api/v1/portfolio",
    "price_api": "/api/v1/prices",
    "staking_api": "/api/v1/staking",
    "transaction_api": "/api/v1/transactions",
    "oracle_api": "/api/v1/oracle",
    "treasury_api": "/api/v1/treasury",
    "validator_api": "/api/v1/validators",
    "alerts_api": "/api/v1/alerts",
    "user_api": "/api/v1/user",
    "auth_api": "/api/v1/auth"
}

# Default Feature Flags
DEFAULT_FEATURE_FLAGS = {
    "advanced_charts": True,
    "real_time_data": True,
    "mobile_app": True,
    "dark_mode": True,
    "notifications": True,
    "export_data": True,
    "custom_layouts": True,
    "api_access": True,
    "multi_language": True,
    "two_factor_auth": False,
    "beta_features": False
}

class DashboardConfigManager:
    """Manager for dashboard configuration"""
    
    def __init__(self):
        self.config = DashboardConfig(
            layouts=[DEFAULT_LAYOUT],
            api_endpoints=DEFAULT_API_ENDPOINTS.copy(),
            feature_flags=DEFAULT_FEATURE_FLAGS.copy()
        )
        self._load_environment_overrides()
    
    def _load_environment_overrides(self):
        """Load configuration overrides from environment variables"""
        # Theme override
        theme = os.getenv("DASHBOARD_THEME")
        if theme:
            try:
                self.config.display.theme = Theme(theme.lower())
            except ValueError:
                pass
        
        # Language override
        language = os.getenv("DASHBOARD_LANGUAGE")
        if language:
            try:
                self.config.localization.default_language = Language(language.lower())
            except ValueError:
                pass
        
        # Currency override
        currency = os.getenv("DASHBOARD_CURRENCY")
        if currency:
            try:
                self.config.display.primary_currency = Currency(currency.upper())
            except ValueError:
                pass
        
        # Refresh interval override
        refresh_interval = os.getenv("DASHBOARD_REFRESH_INTERVAL")
        if refresh_interval:
            try:
                interval_value = int(refresh_interval)
                # Find matching enum value
                for interval in RefreshInterval:
                    if interval.value == interval_value:
                        # Update default refresh interval for widgets
                        for layout in self.config.layouts:
                            for widget in layout.widgets:
                                widget.refresh_interval = interval
                        break
            except ValueError:
                pass
        
        # Notifications toggle
        notifications = os.getenv("DASHBOARD_NOTIFICATIONS")
        if notifications:
            enabled = notifications.lower() in ("true", "1", "yes")
            self.config.display.desktop_notifications = enabled
            self.config.alerts.push_notifications = enabled
        
        # API base URL override
        api_base = os.getenv("DASHBOARD_API_BASE_URL")
        if api_base:
            # Update all API endpoints with new base URL
            for key, endpoint in self.config.api_endpoints.items():
                self.config.api_endpoints[key] = f"{api_base.rstrip('/')}{endpoint}"
    
    def get_config(self) -> DashboardConfig:
        """Get the current dashboard configuration"""
        return self.config
    
    def get_layout(self, layout_id: str) -> Optional[DashboardLayout]:
        """Get a specific dashboard layout"""
        for layout in self.config.layouts:
            if layout.layout_id == layout_id:
                return layout
        return None
    
    def get_default_layout(self) -> Optional[DashboardLayout]:
        """Get the default dashboard layout"""
        for layout in self.config.layouts:
            if layout.is_default:
                return layout
        return self.config.layouts[0] if self.config.layouts else None
    
    def add_layout(self, layout: DashboardLayout):
        """Add a new dashboard layout"""
        # Ensure layout ID is unique
        existing_ids = [l.layout_id for l in self.config.layouts]
        if layout.layout_id in existing_ids:
            counter = 1
            base_id = layout.layout_id
            while f"{base_id}_{counter}" in existing_ids:
                counter += 1
            layout.layout_id = f"{base_id}_{counter}"
        
        layout.created_at = datetime.now().isoformat()
        layout.updated_at = datetime.now().isoformat()
        self.config.layouts.append(layout)
    
    def remove_layout(self, layout_id: str):
        """Remove a dashboard layout"""
        self.config.layouts = [l for l in self.config.layouts if l.layout_id != layout_id]
    
    def update_widget(self, layout_id: str, widget_id: str, widget_config: WidgetConfig):
        """Update a widget in a specific layout"""
        layout = self.get_layout(layout_id)
        if layout:
            for i, widget in enumerate(layout.widgets):
                if widget.widget_id == widget_id:
                    layout.widgets[i] = widget_config
                    layout.updated_at = datetime.now().isoformat()
                    break
    
    def add_widget(self, layout_id: str, widget: WidgetConfig):
        """Add a widget to a specific layout"""
        layout = self.get_layout(layout_id)
        if layout:
            # Ensure widget ID is unique within the layout
            existing_ids = [w.widget_id for w in layout.widgets]
            if widget.widget_id in existing_ids:
                counter = 1
                base_id = widget.widget_id
                while f"{base_id}_{counter}" in existing_ids:
                    counter += 1
                widget.widget_id = f"{base_id}_{counter}"
            
            layout.widgets.append(widget)
            layout.updated_at = datetime.now().isoformat()
    
    def remove_widget(self, layout_id: str, widget_id: str):
        """Remove a widget from a specific layout"""
        layout = self.get_layout(layout_id)
        if layout:
            layout.widgets = [w for w in layout.widgets if w.widget_id != widget_id]
            layout.updated_at = datetime.now().isoformat()
    
    def update_localization(self, language: Language, settings: Dict[str, Any]):
        """Update localization settings"""
        self.config.localization.default_language = language
        for key, value in settings.items():
            if hasattr(self.config.localization, key):
                setattr(self.config.localization, key, value)
    
    def update_display_settings(self, settings: Dict[str, Any]):
        """Update display settings"""
        for key, value in settings.items():
            if hasattr(self.config.display, key):
                setattr(self.config.display, key, value)
    
    def toggle_feature_flag(self, flag_name: str, enabled: bool):
        """Toggle a feature flag"""
        self.config.feature_flags[flag_name] = enabled
    
    def get_user_preferences(self, user_id: str) -> Dict[str, Any]:
        """Get user-specific preferences (placeholder for user customization)"""
        # This would typically load from a database
        return {
            "theme": self.config.display.theme.value,
            "language": self.config.localization.default_language.value,
            "currency": self.config.display.primary_currency.value,
            "layout_id": self.get_default_layout().layout_id if self.get_default_layout() else None,
            "notifications": self.config.display.desktop_notifications
        }
    
    def save_user_preferences(self, user_id: str, preferences: Dict[str, Any]):
        """Save user-specific preferences (placeholder for user customization)"""
        # This would typically save to a database
        pass
    
    def get_dashboard_summary(self) -> Dict[str, Any]:
        """Get a summary of dashboard configuration"""
        return {
            "version": self.config.version,
            "theme": self.config.display.theme.value,
            "language": self.config.localization.default_language.value,
            "currency": self.config.display.primary_currency.value,
            "total_layouts": len(self.config.layouts),
            "total_widgets": sum(len(layout.widgets) for layout in self.config.layouts),
            "feature_flags": self.config.feature_flags,
            "api_endpoints": len(self.config.api_endpoints),
            "supported_languages": [lang.value for lang in self.config.localization.supported_languages],
            "notifications_enabled": self.config.alerts.push_notifications,
            "performance_optimizations": {
                "lazy_loading": self.config.performance.lazy_loading,
                "virtual_scrolling": self.config.performance.virtual_scrolling,
                "caching": self.config.performance.cache_duration > 0
            }
        }
    
    def to_dict(self) -> Dict:
        """Convert configuration to dictionary"""
        def convert_enum(obj):
            if isinstance(obj, Enum):
                return obj.value
            elif isinstance(obj, dict):
                return {k: convert_enum(v) for k, v in obj.items()}
            elif isinstance(obj, list):
                return [convert_enum(item) for item in obj]
            elif hasattr(obj, '__dict__'):
                return {k: convert_enum(v) for k, v in obj.__dict__.items()}
            return obj
        
        return convert_enum(self.config.__dict__)
    
    def to_json(self, indent: int = 2) -> str:
        """Convert configuration to JSON string"""
        return json.dumps(self.to_dict(), indent=indent)
    
    def save_to_file(self, filepath: str):
        """Save configuration to JSON file"""
        with open(filepath, 'w') as f:
            f.write(self.to_json())
    
    def validate_config(self) -> List[str]:
        """Validate the current configuration and return any errors"""
        errors = []
        
        # Validate layouts
        if not self.config.layouts:
            errors.append("At least one dashboard layout must be configured")
        
        layout_ids = [layout.layout_id for layout in self.config.layouts]
        if len(layout_ids) != len(set(layout_ids)):
            errors.append("Dashboard layout IDs must be unique")
        
        default_layouts = [layout for layout in self.config.layouts if layout.is_default]
        if len(default_layouts) != 1:
            errors.append("Exactly one default layout must be configured")
        
        # Validate widgets
        for layout in self.config.layouts:
            widget_ids = [widget.widget_id for widget in layout.widgets]
            if len(widget_ids) != len(set(widget_ids)):
                errors.append(f"Widget IDs must be unique within layout '{layout.layout_id}'")
            
            for widget in layout.widgets:
                if not widget.data_source and widget.type in ["chart", "metric", "table"]:
                    errors.append(f"Widget '{widget.widget_id}' requires a data source")
        
        # Validate API endpoints
        required_endpoints = ["portfolio_api", "price_api", "transaction_api"]
        for endpoint in required_endpoints:
            if endpoint not in self.config.api_endpoints:
                errors.append(f"Required API endpoint '{endpoint}' is missing")
        
        # Validate display settings
        if self.config.display.font_size not in ["small", "medium", "large"]:
            errors.append("Font size must be 'small', 'medium', or 'large'")
        
        # Validate localization
        if self.config.localization.default_language not in self.config.localization.supported_languages:
            errors.append("Default language must be in supported languages list")
        
        return errors

# Utility functions
def create_dashboard_manager() -> DashboardConfigManager:
    """Create a new dashboard configuration manager"""
    return DashboardConfigManager()

def load_dashboard_config_from_env() -> DashboardConfigManager:
    """Load dashboard configuration from environment variables"""
    return DashboardConfigManager()

# Example usage and testing
if __name__ == "__main__":
    from datetime import datetime
    
    # Create dashboard configuration manager
    dashboard_manager = create_dashboard_manager()
    
    # Print dashboard summary
    print("Dashboard Configuration Summary:")
    summary = dashboard_manager.get_dashboard_summary()
    print(json.dumps(summary, indent=2))
    
    # Example: Add custom widget
    custom_widget = WidgetConfig(
        widget_id="custom_alerts",
        title="Custom Alerts",
        type="alert",
        position={"x": 0, "y": 11, "w": 12, "h": 2},
        refresh_interval=RefreshInterval.FAST,
        data_source="alerts_api",
        settings={
            "alert_types": ["price", "portfolio", "system"],
            "max_alerts": 5
        }
    )
    
    dashboard_manager.add_widget("default", custom_widget)
    print(f"\nAdded custom widget: {custom_widget.widget_id}")
    
    # Example: Update display settings
    dashboard_manager.update_display_settings({
        "theme": Theme.DARK,
        "compact_mode": True,
        "animations_enabled": False
    })
    print("Updated display settings")
    
    # Validate configuration
    errors = dashboard_manager.validate_config()
    if errors:
        print("\nConfiguration Errors:")
        for error in errors:
            print(f"  - {error}")
    else:
        print("\nDashboard configuration is valid!")
    
    # Save configuration to file
    dashboard_manager.save_to_file("dashboard_config.json")
    print("\nConfiguration saved to dashboard_config.json")