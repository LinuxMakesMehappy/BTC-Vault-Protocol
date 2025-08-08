#!/usr/bin/env python3
"""
Comprehensive Integration Test Runner
Executes all integration tests including end-to-end user journeys,
cross-chain operations, stress testing, and security integration tests.

Addresses Task 26: Create comprehensive integration tests
Requirements: FR7 - Testing and Development Infrastructure
"""

import os
import sys
import time
import asyncio
import subprocess
from typing import Dict, Any, List
import psutil

# Add project root to path
sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

def main():
    """Main function to run comprehensive integration tests"""
    print("🧪 VAULT PROTOCOL - COMPREHENSIVE INTEGRATION TESTS")
    print("Addresses Task 26: Create comprehensive integration tests")
    print("Requirements: FR7 - Testing and Development Infrastructure")
    print("=" * 80)
    
    # System information
    memory_gb = psutil.virtual_memory().total / (1024**3)
    cpu_count = os.cpu_count()
    
    print(f"💻 System Information:")
    print(f"   Total Memory: {memory_gb:.1f} GB")
    print(f"   CPU Cores: {cpu_count}")
    print(f"   Available Memory: {psutil.virtual_memory().available / (1024**3):.1f} GB")
    
    # Optimization for low-resource systems
    if memory_gb <= 8:
        print(f"   🎯 Low-resource system detected - optimizing for 8GB RAM")
        max_workers = min(4, cpu_count)
    else:
        max_workers = min(8, cpu_count)
    
    print(f"   🔧 Using {max_workers} concurrent workers")
    print("=" * 80)
    
    # Run comprehensive integration tests
    start_time = time.time()
    
    try:
        # Execute the comprehensive integration test
        result = subprocess.run([
            sys.executable, '-m', 'pytest', 
            'tests/test_comprehensive_integration.py::TestComprehensiveIntegrationRunner::test_run_all_comprehensive_integration_tests',
            '-v', '--tb=short', '--no-header'
        ], capture_output=True, text=True, timeout=600)  # 10 minute timeout
        
        execution_time = time.time() - start_time
        
        # Print results
        print(f"\n📊 INTEGRATION TEST RESULTS:")
        print(f"   Execution Time: {execution_time:.2f}s")
        print(f"   Return Code: {result.returncode}")
        
        if result.returncode == 0:
            print(f"   Status: ✅ ALL TESTS PASSED")
        else:
            print(f"   Status: ❌ SOME TESTS FAILED")
        
        # Print output
        if result.stdout:
            print(f"\n📝 Test Output:")
            print(result.stdout)
        
        if result.stderr:
            print(f"\n⚠️  Errors/Warnings:")
            print(result.stderr)
        
        # Performance assessment
        print(f"\n🎯 PERFORMANCE ASSESSMENT:")
        if execution_time < 300:  # Under 5 minutes
            print(f"   ✅ Execution time excellent ({execution_time:.2f}s < 300s)")
        elif execution_time < 600:  # Under 10 minutes
            print(f"   ⚠️  Execution time acceptable ({execution_time:.2f}s < 600s)")
        else:
            print(f"   ❌ Execution time too long ({execution_time:.2f}s >= 600s)")
        
        # Memory assessment
        final_memory = psutil.virtual_memory().used / (1024**2)
        if final_memory < 4096:  # Under 4GB
            print(f"   ✅ Memory usage excellent ({final_memory:.1f}MB < 4096MB)")
        elif final_memory < 6144:  # Under 6GB
            print(f"   ⚠️  Memory usage acceptable ({final_memory:.1f}MB < 6144MB)")
        else:
            print(f"   ❌ Memory usage too high ({final_memory:.1f}MB >= 6144MB)")
        
        print("=" * 80)
        
        return result.returncode == 0
        
    except subprocess.TimeoutExpired:
        execution_time = time.time() - start_time
        print(f"❌ Integration tests timed out after {execution_time:.2f}s")
        return False
        
    except Exception as e:
        execution_time = time.time() - start_time
        print(f"❌ Integration tests failed with error: {str(e)}")
        return False

if __name__ == "__main__":
    success = main()
    sys.exit(0 if success else 1)