"""PySynthData CLI"""
import argparse
from .cli_dashboard import PySynthDataDashboard

def main():
    parser = argparse.ArgumentParser(description='PySynthData Generation')
    subparsers = parser.add_subparsers(dest='command')
    dashboard_parser = subparsers.add_parser('dashboard', help='View dashboard')
    dashboard_parser.add_argument('--static', action='store_true')
    dashboard_parser.add_argument('--export', type=str)
    args = parser.parse_args()
    
    if args.command == 'dashboard':
        dashboard = PySynthDataDashboard()
        if args.export:
            dashboard.export_json(args.export)
        else:
            dashboard.run_dashboard(interactive=not args.static)

if __name__ == '__main__':
    main()
