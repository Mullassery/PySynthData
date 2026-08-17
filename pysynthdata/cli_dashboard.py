"""Synthetic Data Generation Dashboard"""
import sys, platform, json
from datetime import datetime
from typing import Optional, Dict, Any
from dataclasses import dataclass

@dataclass
class DashboardMetrics:
    timestamp: str
    title: str
    metrics: Dict[str, Any]
    alerts: list
    recommendations: list

def get_dashboard_impl(product_name: str):
    try:
        from rich.console import Console
        return RichDashboard(product_name)
    except ImportError:
        return SimpleDashboard(product_name)

class SimpleDashboard:
    def __init__(self, product_name: str):
        self.product_name = product_name

    def render(self, data: DashboardMetrics) -> None:
        print(f"\n{'='*80}\n✓ {data.title}\n  {data.timestamp}\n{'='*80}\n")
        for key, value in data.metrics.items():
            print(f"  {key}: {value}")

class RichDashboard:
    def __init__(self, product_name: str):
        self.product_name = product_name
        from rich.console import Console
        self.console = Console()

    def render(self, data: DashboardMetrics) -> None:
        from rich.table import Table
        self.console.print(f"\n[bold cyan]✓ {data.title}[/bold cyan]")
        table = Table()
        table.add_column("Metric", style="cyan")
        table.add_column("Value", style="green")
        for key, value in data.metrics.items():
            table.add_row(key, str(value))
        self.console.print(table)

class PySynthDataDashboard:
    def __init__(self, config_path: Optional[str] = None):
        self.dashboard = get_dashboard_impl("PySynthData v0.1")

    def get_mock_metrics(self) -> DashboardMetrics:
        return DashboardMetrics(
            datetime.now().isoformat(),
            "PySynthData Generation Dashboard",
            {
                "Status": "🟢 Generating",
                "Datasets": "12 active",
                "Records Generated": "2.3M total",
                "Generation Rate": "185K records/min",
                "Quality Score": "97.2%",
                "Data Types": "8 supported",
                "Memory Usage": "1.2 GB",
            },
            [],
            []
        )

    def run_dashboard(self, interactive: bool = True) -> None:
        # NOTE: `interactive` currently has no distinct behavior — both
        # `SimpleDashboard` and `RichDashboard` only implement `render()`,
        # there is no separate live/interactive mode to call into. Calling
        # a nonexistent `.run()` here used to crash unconditionally; render
        # once instead of pretending an interactive mode exists.
        metrics = self.get_mock_metrics()
        self.dashboard.render(metrics)

    def export_json(self, output_file: str) -> None:
        metrics = self.get_mock_metrics()
        with open(output_file, 'w') as f:
            json.dump({"metrics": metrics.metrics, "timestamp": metrics.timestamp}, f)
