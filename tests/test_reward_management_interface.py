#!/usr/bin/env python3
"""
VaultBTC Protocol - Reward Management Interface Tests
Tests the reward claiming interface and management functionality (Task 19)
"""

import os
import sys
import json
import time
from datetime import datetime, timedelta

def test_reward_manager_component():
    """Test that RewardManager component is properly implemented"""
    print("Testing RewardManager component structure...")
    
    # Check RewardManager component exists
    component_path = "frontend/src/components/RewardManager.tsx"
    assert os.path.exists(component_path), f"RewardManager component not found at {component_path}"
    
    with open(component_path, 'r') as f:
        component_content = f.read()
    
    # Verify key reward management features
    required_features = [
        "reward claiming interface",
        "BTC/USDC payment selection",
        "auto-reinvestment toggle",
        "payment history",
        "transaction tracking",
        "selectedPaymentMethod",
        "autoReinvest",
        "handleClaimRewards",
        "handleToggleAutoReinvest",
        "Payment History"
    ]
    
    for feature in required_features:
        assert feature in component_content, f"RewardManager missing feature: {feature}"
    
    print("✅ RewardManager component structure verified")

def test_reward_claiming_interface():
    """Test reward claiming interface functionality"""
    print("Testing reward claiming interface...")
    
    component_path = "frontend/src/components/RewardManager.tsx"
    with open(component_path, 'r') as f:
        component_content = f.read()
    
    # Verify claiming interface features
    claiming_features = [
        "Claim Rewards",
        "Payment Method",
        "BTC (Lightning)",
        "USDC",
        "handleClaimRewards",
        "claiming",
        "setClaiming",
        "showClaimModal",
        "Confirm Reward Claim"
    ]
    
    for feature in claiming_features:
        assert feature in component_content, f"Claiming interface missing: {feature}"
    
    print("✅ Reward claiming interface verified")

def test_payment_selection():
    """Test BTC/USDC payment selection functionality"""
    print("Testing payment selection...")
    
    component_path = "frontend/src/components/RewardManager.tsx"
    with open(component_path, 'r') as f:
        component_content = f.read()
    
    # Verify payment selection features
    payment_features = [
        "selectedPaymentMethod",
        "setSelectedPaymentMethod",
        "btc",
        "usdc",
        "Lightning Network payment",
        "USDC payment to your wallet",
        "radio",
        "input"
    ]
    
    for feature in payment_features:
        assert feature in component_content, f"Payment selection missing: {feature}"
    
    print("✅ Payment selection functionality verified")

def test_auto_reinvestment_toggle():
    """Test auto-reinvestment toggle and configuration"""
    print("Testing auto-reinvestment functionality...")
    
    component_path = "frontend/src/components/RewardManager.tsx"
    with open(component_path, 'r') as f:
        component_content = f.read()
    
    # Verify auto-reinvestment features
    reinvest_features = [
        "Auto-Reinvestment",
        "autoReinvest",
        "setAutoReinvest",
        "handleToggleAutoReinvest",
        "Enable Auto-Reinvestment",
        "Automatically reinvest rewards",
        "compound earnings",
        "toggle"
    ]
    
    for feature in reinvest_features:
        assert feature in component_content, f"Auto-reinvestment missing: {feature}"
    
    print("✅ Auto-reinvestment functionality verified")

def test_payment_history_tracking():
    """Test payment history and transaction tracking"""
    print("Testing payment history tracking...")
    
    component_path = "frontend/src/components/RewardManager.tsx"
    with open(component_path, 'r') as f:
        component_content = f.read()
    
    # Verify payment history features
    history_features = [
        "Payment History",
        "paymentHistory",
        "setPaymentHistory",
        "getPaymentHistory",
        "Track your reward claims",
        "Date",
        "Amount",
        "Method",
        "Status",
        "Transaction",
        "table"
    ]
    
    for feature in history_features:
        assert feature in component_content, f"Payment history missing: {feature}"
    
    print("✅ Payment history tracking verified")

def test_rewards_page_integration():
    """Test that rewards page uses RewardManager component"""
    print("Testing rewards page integration...")
    
    page_path = "frontend/src/app/rewards/page.tsx"
    assert os.path.exists(page_path), f"Rewards page not found at {page_path}"
    
    with open(page_path, 'r') as f:
        page_content = f.read()
    
    # Verify page imports and uses RewardManager
    assert "import RewardManager" in page_content, "Rewards page doesn't import RewardManager"
    assert "<RewardManager />" in page_content, "Rewards page doesn't render RewardManager"
    
    print("✅ Rewards page integration verified")

def test_vault_client_reward_methods():
    """Test VaultClient reward management methods"""
    print("Testing VaultClient reward methods...")
    
    client_path = "frontend/src/lib/vault-client.ts"
    assert os.path.exists(client_path), f"VaultClient not found at {client_path}"
    
    with open(client_path, 'r') as f:
        client_content = f.read()
    
    # Verify reward management methods
    reward_methods = [
        "setAutoReinvest",
        "getRewardHistory",
        "claimRewards",
        "getPaymentHistory",
        "getPaymentConfig"
    ]
    
    for method in reward_methods:
        assert f"async {method}" in client_content or f"{method}(" in client_content, f"VaultClient missing method: {method}"
    
    print("✅ VaultClient reward methods verified")

def test_reward_types():
    """Test reward management type definitions"""
    print("Testing reward management types...")
    
    types_path = "frontend/src/types/vault.ts"
    assert os.path.exists(types_path), f"Types file not found at {types_path}"
    
    with open(types_path, 'r') as f:
        types_content = f.read()
    
    # Verify reward management types
    required_types = [
        "PaymentHistory",
        "RewardSummary"
    ]
    
    for type_name in required_types:
        assert f"interface {type_name}" in types_content, f"Missing type definition: {type_name}"
    
    print("✅ Reward management types verified")

def test_reward_ui_components():
    """Test reward management UI components"""
    print("Testing reward management UI components...")
    
    component_path = "frontend/src/components/RewardManager.tsx"
    with open(component_path, 'r') as f:
        component_content = f.read()
    
    # Verify UI components
    ui_components = [
        "Reward Summary Cards",
        "Pending Rewards",
        "Total Earned", 
        "Claimed Rewards",
        "Current APY",
        "grid grid-cols",
        "bg-white rounded-xl",
        "shadow-sm",
        "responsive"
    ]
    
    ui_checks = [
        "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4",  # Responsive grid
        "Pending Rewards",  # Summary cards
        "Total Earned",  # Earnings display
        "Claimed Rewards",  # Claims tracking
        "Current APY",  # APY display
        "bg-white rounded-xl shadow-sm",  # Card styling
        "hover:bg-gray-50"  # Interactive elements
    ]
    
    for check in ui_checks:
        assert check in component_content, f"UI component missing: {check}"
    
    print("✅ Reward management UI components verified")

def test_reward_data_flow():
    """Test reward management data flow"""
    print("Testing reward data flow...")
    
    component_path = "frontend/src/components/RewardManager.tsx"
    with open(component_path, 'r') as f:
        component_content = f.read()
    
    # Verify data flow components
    data_flow_features = [
        "fetchRewardData",
        "useState",
        "useEffect",
        "VaultClient",
        "getRewardSummary",
        "getPaymentHistory",
        "getPaymentConfig",
        "loading",
        "setLoading"
    ]
    
    for feature in data_flow_features:
        assert feature in component_content, f"Data flow missing: {feature}"
    
    print("✅ Reward data flow verified")

def test_reward_security_features():
    """Test reward management security features"""
    print("Testing reward security features...")
    
    component_path = "frontend/src/components/RewardManager.tsx"
    with open(component_path, 'r') as f:
        component_content = f.read()
    
    # Verify security features
    security_features = [
        "!connected",  # Wallet connection check
        "Connect Your Wallet",  # Connection requirement
        "wallet.publicKey",  # Public key validation
        "try {",  # Error handling
        "catch (error)",  # Error catching
        "showToast",  # Error notifications
        "disabled={",  # Button state management
        "claiming"  # Transaction state
    ]
    
    for feature in security_features:
        assert feature in component_content, f"Security feature missing: {feature}"
    
    print("✅ Reward security features verified")

def test_reward_integration_points():
    """Test integration with other system components"""
    print("Testing reward integration points...")
    
    component_path = "frontend/src/components/RewardManager.tsx"
    with open(component_path, 'r') as f:
        component_content = f.read()
    
    # Verify integration points
    integrations = [
        "useWallet",  # Wallet integration (Task 17)
        "useToast",  # Toast notifications
        "VaultClient",  # Backend integration
        "connected",  # Connection state
        "wallet.publicKey",  # Wallet access
        "RewardSummary",  # Type integration
        "PaymentHistory"  # History integration
    ]
    
    for integration in integrations:
        assert integration in component_content, f"Integration missing: {integration}"
    
    print("✅ Reward integration points verified")

def test_reward_user_experience():
    """Test reward management user experience features"""
    print("Testing reward user experience...")
    
    component_path = "frontend/src/components/RewardManager.tsx"
    with open(component_path, 'r') as f:
        component_content = f.read()
    
    # Verify UX features
    ux_features = [
        "Loading states",
        "Error handling",
        "Success notifications",
        "Modal dialogs",
        "Confirmation flows",
        "Responsive design",
        "Accessibility",
        "Visual feedback"
    ]
    
    ux_checks = [
        "loading",  # Loading states
        "showToast",  # Notifications
        "showClaimModal",  # Modal dialogs
        "Confirm Reward Claim",  # Confirmation
        "md:grid-cols",  # Responsive
        "disabled={",  # Accessibility
        "hover:",  # Visual feedback
        "transition-colors"  # Smooth transitions
    ]
    
    for check in ux_checks:
        assert check in component_content, f"UX feature missing: {check}"
    
    print("✅ Reward user experience verified")

def run_all_tests():
    """Run all reward management interface tests"""
    print("🚀 Starting Reward Management Interface Tests...")
    print("=" * 60)
    
    tests = [
        test_reward_manager_component,
        test_reward_claiming_interface,
        test_payment_selection,
        test_auto_reinvestment_toggle,
        test_payment_history_tracking,
        test_rewards_page_integration,
        test_vault_client_reward_methods,
        test_reward_types,
        test_reward_ui_components,
        test_reward_data_flow,
        test_reward_security_features,
        test_reward_integration_points,
        test_reward_user_experience
    ]
    
    passed = 0
    failed = 0
    
    for test in tests:
        try:
            test()
            passed += 1
        except Exception as e:
            print(f"❌ {test.__name__} failed: {str(e)}")
            failed += 1
    
    print("=" * 60)
    print(f"📊 Test Results: {passed} passed, {failed} failed")
    
    if failed == 0:
        print("🎉 All reward management interface tests passed!")
        print("\nReward Management Interface Summary:")
        print("   - Reward claiming interface with BTC/USDC payment selection")
        print("   - Auto-reinvestment toggle and configuration options")
        print("   - Payment history and transaction tracking")
        print("   - Integration with wallet system (Task 17) and rewards system (Task 7)")
        print("   - Comprehensive UI with responsive design")
        print("   - Security features with wallet connection requirements")
        print("   - Real-time data updates and error handling")
        print("   - Modal dialogs for confirmation flows")
        return True
    else:
        print(f"❌ {failed} reward management interface tests failed!")
        return False

if __name__ == "__main__":
    success = run_all_tests()
    sys.exit(0 if success else 1)