#!/usr/bin/env python3
"""
Comprehensive Test Suite Runner
Executes all test modules with concurrent support using ThreadPoolExecutor
Addresses FR7: Testing and Development Infrastructure requirements
"""

import os
import sys
import time
import subprocess
import threading
from concurrent.futures import ThreadPoolExecutor, as_completed
from typing import List, Dict, Any, Tuple
import psutil

# Add project root to path
sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

class ComprehensiveTestRunner:
    """
    Manages execution of all test modules with resource monitoring
    Optimized for low-resource systems (8GB RAM, 256GB storage)
    """
    
    def __init__(self, max_workers: int = None):
        # Optimize for low-resource systems
        if max_workers is None:
            cpu_count = os.cpu_count() or 4
            memory_gb = psutil.virtual_memory().total / (1024**3)
            
            if memory_gb <= 8:
                max_workers = min(4, cpu_count)  # Conservative for 8GB RAM
            else:
                max_workers = min(8, cpu_count)
        
        self.max_workers = max_workers
        self.results = {}
        self.start_time = None
        self.end_time = None
        
    def run_test_module(self, module_path: str, module_name: str) -> Dict[str, Any]:
        """Run a single test module and return results"""
        start_time = time.time()
        
        try:
            # Run pytest on the module
            result = subprocess.run([
                sys.executable, '-m', 'pytest', module_path, '-v', '--tb=short'
            ], capture_output=True, text=True, timeout=300)  # 5 minute timeout
            
            execution_time = time.time() - start_time
            
            # Parse pytest output
            output_lines = result.stdout.split('\n')
            passed = 0
            failed = 0
            errors = []
            
            for line in output_lines:
                if '::' in line and 'PASSED' in line:
                    passed += 1
                elif '::' in line and 'FAILED' in line:
                    failed += 1
                    errors.append(line.strip())
                elif '::' in line and 'ERROR' in line:
                    failed += 1
                    errors.append(line.strip())
            
            return {
                'module_name': module_name,
                'passed': passed,
                'failed': failed,
                'total': passed + failed,
                'execution_time': execution_time,
                'return_code': result.returncode,
                'errors': errors,
                'stdout': result.stdout,
                'stderr': result.stderr
            }
            
        except subprocess.TimeoutExpired:
            return {
                'module_name': module_name,
                'passed': 0,
                'failed': 1,
                'total': 1,
                'execution_time': time.time() - start_time,
                'return_code': -1,
                'errors': ['Test module timed out after 5 minutes'],
                'stdout': '',
                'stderr': 'Timeout'
            }
        except Exception as e:
            return {
                'module_name': module_name,
                'passed': 0,
                'failed': 1,
                'total': 1,
                'execution_time': time.time() - start_time,
                'return_code': -1,
                'errors': [str(e)],
                'stdout': '',
                'stderr': str(e)
            }
    
    def run_all_tests(self) -> Dict[str, Any]:
        """Execute all test modules concurrently"""
        self.start_time = time.time()
        
        # Define test modules
        test_modules = [
            ('tests/test_btc_commitment.py', 'BTC_Commitment'),
            ('tests/test_staking.py', 'Staking_Pool'),
            ('tests/test_rewards.py', 'Reward_System'),
            ('tests/test_multisig_security.py', 'Multisig_Security'),
            ('tests/test_kyc_compliance.py', 'KYC_Compliance'),
            ('tests/test_treasury_management.py', 'Treasury_Management'),
        ]
        
        # Filter to only existing modules
        existing_modules = []
        for module_path, module_name in test_modules:
            if os.path.exists(module_path):
                existing_modules.append((module_path, module_name))
            else:
                print(f"⚠️  Warning: {module_path} not found, skipping...")
        
        print(f"🚀 Starting comprehensive test execution with {self.max_workers} workers...")
        print(f"📊 Total test modules: {len(existing_modules)}")
        print(f"💾 Available memory: {psutil.virtual_memory().available / (1024**3):.1f} GB")
        print(f"🔧 CPU cores: {os.cpu_count()}")
        print("-" * 80)
        
        # Execute test modules concurrently
        with ThreadPoolExecutor(max_workers=self.max_workers) as executor:
            # Submit all test modules
            future_to_module = {
                executor.submit(self.run_test_module, module_path, module_name): module_name
                for module_path, module_name in existing_modules
            }
            
            # Collect results as they complete
            for future in as_completed(future_to_module):
                module_name = future_to_module[future]
                try:
                    result = future.result()
                    self.results[module_name] = result
                    
                    # Print progress
                    if result['return_code'] == 0 and result['failed'] == 0:
                        status = "✅"
                    elif result['failed'] > 0:
                        status = "❌"
                    else:
                        status = "⚠️"
                    
                    print(f"{status} {module_name}: {result['passed']}/{result['total']} passed "
                          f"({result['execution_time']:.2f}s)")
                    
                    if result['errors']:
                        for error in result['errors'][:3]:  # Show first 3 errors
                            print(f"   • {error}")
                        if len(result['errors']) > 3:
                            print(f"   • ... and {len(result['errors']) - 3} more errors")
                    
                except Exception as e:
                    self.results[module_name] = {
                        'module_name': module_name,
                        'passed': 0,
                        'failed': 1,
                        'total': 1,
                        'execution_time': 0,
                        'return_code': -1,
                        'errors': [str(e)],
                        'stdout': '',
                        'stderr': str(e)
                    }
                    print(f"❌ {module_name}: Module execution failed - {str(e)}")
        
        self.end_time = time.time()
        return self.generate_summary()
    
    def generate_summary(self) -> Dict[str, Any]:
        """Generate comprehensive test execution summary"""
        total_tests = sum(result['total'] for result in self.results.values())
        total_passed = sum(result['passed'] for result in self.results.values())
        total_failed = sum(result['failed'] for result in self.results.values())
        total_time = self.end_time - self.start_time if self.end_time and self.start_time else 0
        
        # Performance metrics
        fastest_module = min(self.results.values(), key=lambda x: x['execution_time']) if self.results else None
        slowest_module = max(self.results.values(), key=lambda x: x['execution_time']) if self.results else None
        
        # Success metrics
        successful_modules = [r for r in self.results.values() if r['return_code'] == 0 and r['failed'] == 0]
        failed_modules = [r for r in self.results.values() if r['return_code'] != 0 or r['failed'] > 0]
        
        summary = {
            'execution_summary': {
                'total_modules': len(self.results),
                'successful_modules': len(successful_modules),
                'failed_modules': len(failed_modules),
                'total_tests': total_tests,
                'total_passed': total_passed,
                'total_failed': total_failed,
                'success_rate': (total_passed / total_tests * 100) if total_tests > 0 else 0,
                'total_execution_time': total_time,
                'concurrent_workers': self.max_workers
            },
            'performance_metrics': {
                'fastest_module': {
                    'name': fastest_module['module_name'] if fastest_module else None,
                    'time': fastest_module['execution_time'] if fastest_module else 0
                },
                'slowest_module': {
                    'name': slowest_module['module_name'] if slowest_module else None,
                    'time': slowest_module['execution_time'] if slowest_module else 0
                },
                'avg_module_time': sum(r['execution_time'] for r in self.results.values()) / len(self.results) if self.results else 0
            },
            'module_results': self.results,
            'resource_optimization': {
                'time_efficient': total_time < 300,  # Under 5 minutes
                'low_resource_compatible': total_time < 180  # Under 3 minutes for low-resource systems
            }
        }
        
        return summary
    
    def print_detailed_report(self, summary: Dict[str, Any]):
        """Print detailed test execution report"""
        print("\n" + "="*80)
        print("🧪 COMPREHENSIVE TEST EXECUTION REPORT")
        print("="*80)
        
        # Execution Summary
        exec_summary = summary['execution_summary']
        print(f"\n📊 EXECUTION SUMMARY:")
        print(f"   Total Test Modules: {exec_summary['total_modules']}")
        print(f"   ✅ Successful Modules: {exec_summary['successful_modules']}")
        print(f"   ❌ Failed Modules: {exec_summary['failed_modules']}")
        print(f"   Total Tests: {exec_summary['total_tests']}")
        print(f"   ✅ Passed: {exec_summary['total_passed']}")
        print(f"   ❌ Failed: {exec_summary['total_failed']}")
        print(f"   📈 Success Rate: {exec_summary['success_rate']:.1f}%")
        print(f"   ⏱️  Total Time: {exec_summary['total_execution_time']:.2f}s")
        print(f"   🔧 Workers Used: {exec_summary['concurrent_workers']}")
        
        # Performance Metrics
        perf_metrics = summary['performance_metrics']
        print(f"\n⚡ PERFORMANCE METRICS:")
        print(f"   Fastest Module: {perf_metrics['fastest_module']['name']} ({perf_metrics['fastest_module']['time']:.2f}s)")
        print(f"   Slowest Module: {perf_metrics['slowest_module']['name']} ({perf_metrics['slowest_module']['time']:.2f}s)")
        print(f"   Average Module Time: {perf_metrics['avg_module_time']:.2f}s")
        
        # Resource Optimization
        resource_opt = summary['resource_optimization']
        print(f"\n🎯 RESOURCE OPTIMIZATION:")
        print(f"   Time Efficient: {'✅' if resource_opt['time_efficient'] else '❌'}")
        print(f"   Low-Resource Compatible: {'✅' if resource_opt['low_resource_compatible'] else '❌'}")
        
        # Module Details
        print(f"\n📋 MODULE DETAILS:")
        for module_name, result in summary['module_results'].items():
            if result['return_code'] == 0 and result['failed'] == 0:
                status = "✅"
            elif result['failed'] > 0:
                status = "❌"
            else:
                status = "⚠️"
            
            print(f"   {status} {module_name}:")
            print(f"      Tests: {result['passed']}/{result['total']} passed")
            print(f"      Time: {result['execution_time']:.2f}s")
            print(f"      Return Code: {result['return_code']}")
            if result['errors']:
                print(f"      Errors: {len(result['errors'])}")
        
        # Recommendations
        print(f"\n💡 RECOMMENDATIONS:")
        if not resource_opt['time_efficient']:
            print("   • Consider optimizing slow test modules or increasing parallelization")
        if exec_summary['failed_modules'] > 0:
            print("   • Review failed modules and fix underlying issues")
        if resource_opt['low_resource_compatible']:
            print("   • ✅ Tests are optimized for low-resource systems (8GB RAM, 256GB storage)")
        
        print("\n" + "="*80)

def main():
    """Main function to run comprehensive tests"""
    print("🧪 VAULT PROTOCOL - COMPREHENSIVE TEST SUITE")
    print("Addresses FR7: Testing and Development Infrastructure")
    print("Optimized for low-resource systems (8GB RAM, 256GB storage)")
    print("-" * 80)
    
    # Initialize test runner
    runner = ComprehensiveTestRunner()
    
    # Execute all tests
    summary = runner.run_all_tests()
    
    # Print detailed report
    runner.print_detailed_report(summary)
    
    # Return success status
    success = summary['execution_summary']['failed_modules'] == 0
    return success

if __name__ == "__main__":
    success = main()
    sys.exit(0 if success else 1)