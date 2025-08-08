# Vault Protocol User Guides

## Table of Contents

1. [Getting Started](#getting-started)
2. [BTC Commitment Guide](#btc-commitment-guide)
3. [Reward Claiming Guide](#reward-claiming-guide)
4. [Security Setup Guide](#security-setup-guide)
5. [KYC Verification Guide](#kyc-verification-guide)
6. [Wallet Integration Guide](#wallet-integration-guide)
7. [Troubleshooting](#troubleshooting)

## Getting Started

### What is Vault Protocol?

Vault Protocol is a non-custodial Bitcoin protocol that allows you to earn rewards on your BTC without transferring custody. The protocol verifies your Bitcoin balance and provides rewards based on protocol staking activities.

### Key Features

- **Non-Custodial**: Your Bitcoin never leaves your wallet
- **Verified Rewards**: Earn 50% of protocol staking profits
- **Multi-Currency Payments**: Receive rewards in BTC or USDC
- **Enterprise Security**: HSM-backed multisig and 2FA protection
- **Global Access**: Multi-language support and hardware wallet integration

### System Requirements

- **Minimum**: 8GB RAM, 256GB storage
- **Supported Wallets**: BlueWallet, Ledger hardware wallets
- **Supported Languages**: English, Spanish, Chinese, Japanese
- **Internet**: Stable connection for oracle verification

## BTC Commitment Guide

### Step 1: Connect Your Wallet

1. Visit the Vault Protocol dashboard
2. Click "Connect Wallet" in the top right
3. Choose your wallet type:
   - **BlueWallet**: One-click connection via QR code
   - **Ledger**: Connect hardware wallet via USB

### Step 2: Verify Your Bitcoin Address

1. Navigate to the "Commit BTC" section
2. Enter your Bitcoin address (must be a valid address you control)
3. The system will generate an ECDSA proof requirement
4. Sign the proof using your wallet's private key

**Important**: Never share your private key. The protocol only needs a cryptographic proof of ownership.

### Step 3: Set Your Commitment Amount

1. Enter the amount of BTC you want to commit
2. **Non-KYC Users**: Maximum 1 BTC commitment
3. **KYC Verified Users**: No commitment limit
4. Review the commitment details:
   - Commitment amount
   - Expected rewards (based on current protocol performance)
   - Verification frequency (every 60 seconds)

### Step 4: Submit Your Commitment

1. Review all details carefully
2. Click "Submit Commitment"
3. Sign the transaction with your wallet
4. Wait for blockchain confirmation (typically 1-2 minutes)

### Step 5: Monitor Your Commitment

- **Dashboard**: View real-time commitment status
- **Balance Verification**: Automatic every 60 seconds via Chainlink oracles
- **Reward Tracking**: See accumulated rewards in real-time

### Commitment Management

#### Updating Your Commitment

1. Go to "Manage Commitment" in your dashboard
2. Enter new commitment amount (can increase or decrease)
3. Provide updated ECDSA proof
4. Submit transaction

#### Removing Your Commitment

1. Set commitment amount to 0
2. Claim any pending rewards first
3. Submit removal transaction
4. Your commitment will be removed after confirmation

## Reward Claiming Guide

### Understanding Rewards

- **Reward Ratio**: 1:2 (your BTC commitment to protocol staked assets)
- **Profit Share**: 50% of protocol staking profits
- **Calculation**: Based on your commitment percentage of total protocol commitments
- **Frequency**: Rewards accumulate continuously, claim anytime

### Step 1: Check Your Rewards

1. Visit your dashboard
2. View "Rewards" section showing:
   - **Pending Rewards**: Unclaimed rewards available
   - **Total Earned**: Lifetime rewards earned
   - **Payment History**: Previous reward payments

### Step 2: Choose Payment Method

#### Option A: Bitcoin (BTC) - Default

1. Click "Claim Rewards"
2. Select "BTC Payment"
3. Enter your Lightning Network invoice (recommended)
4. Or provide on-chain Bitcoin address for larger amounts
5. Confirm payment details

**Lightning Network Benefits**:
- Instant payments
- Lower fees
- Better privacy

#### Option B: USDC Stablecoin

1. Click "Claim Rewards"
2. Select "USDC Payment"
3. Provide your USDC wallet address (Solana SPL token)
4. Confirm payment details

**USDC Benefits**:
- Stable value
- Easy conversion to fiat
- Lower volatility

### Step 3: Configure Auto-Reinvestment (Optional)

1. Go to "Reward Settings"
2. Toggle "Auto-Reinvest" on
3. Set reinvestment percentage (0-100%)
4. Choose reinvestment strategy:
   - **Full Reinvest**: All rewards reinvested
   - **Partial**: Set percentage to reinvest
   - **Threshold**: Only reinvest above certain amount

### Step 4: Process Payment

1. Review payment details
2. Click "Confirm Payment"
3. Wait for processing (typically 1-5 minutes)
4. Receive confirmation notification

### Payment Troubleshooting

#### Lightning Network Issues

If Lightning payment fails:
1. System automatically retries 3 times
2. Falls back to on-chain Bitcoin payment
3. Check your Lightning wallet connectivity
4. Ensure sufficient channel capacity

#### USDC Payment Issues

If USDC payment fails:
1. Verify your USDC address is correct
2. Ensure you have a Solana USDC token account
3. Check network congestion status
4. Contact support if issues persist

## Security Setup Guide

### Two-Factor Authentication (2FA)

#### Step 1: Enable 2FA

1. Go to "Security Settings" in your profile
2. Click "Enable 2FA"
3. Choose your preferred method:
   - **TOTP Authenticator** (Google Authenticator, Authy)
   - **WebAuthn/Passkeys** (Hardware keys, biometrics)

#### Step 2: TOTP Setup

1. Install authenticator app on your phone
2. Scan the QR code displayed
3. Enter the 6-digit code from your app
4. Save backup codes in a secure location
5. Confirm 2FA activation

#### Step 3: Passkey Setup

1. Click "Setup Passkey"
2. Follow browser prompts for:
   - **Hardware Key**: Insert and touch your security key
   - **Biometric**: Use fingerprint or face recognition
   - **Platform**: Use device's built-in authenticator
3. Test passkey functionality
4. Add backup passkey (recommended)

### Session Management

#### Security Features

- **Session Timeout**: 24 hours of inactivity
- **Device Tracking**: Monitor login locations and devices
- **Risk Assessment**: Automatic detection of unusual activity
- **Account Lockout**: Temporary lockout after failed attempts

#### Managing Sessions

1. View "Active Sessions" in security settings
2. See current login locations and devices
3. Revoke suspicious sessions immediately
4. Enable email notifications for new logins

### Wallet Security

#### Best Practices

1. **Hardware Wallets**: Use Ledger for maximum security
2. **Backup Seeds**: Store recovery phrases securely offline
3. **Regular Updates**: Keep wallet software updated
4. **Separate Devices**: Use dedicated device for crypto operations

#### Compromise Detection

If you suspect wallet compromise:
1. **Immediate**: Revoke all active sessions
2. **Change**: Update all passwords and 2FA
3. **Move Funds**: Transfer Bitcoin to new secure wallet
4. **Update**: Change commitment to new wallet address
5. **Monitor**: Watch for unauthorized transactions

### Security Monitoring

#### Automated Alerts

The system monitors for:
- **Unusual Login Patterns**: New locations, devices
- **High-Risk Transactions**: Large amounts, frequent claims
- **Failed Authentication**: Multiple failed 2FA attempts
- **Wallet Changes**: New Bitcoin addresses

#### Response Actions

When alerts trigger:
1. **Email Notification**: Immediate security alert
2. **Account Review**: Temporary restrictions may apply
3. **Manual Verification**: May require identity confirmation
4. **Support Contact**: Reach out if legitimate activity

## KYC Verification Guide

### When KYC is Required

- **Commitment > 1 BTC**: Mandatory KYC verification
- **High-Risk Flags**: Chainalysis risk assessment
- **Regulatory Compliance**: Jurisdiction-specific requirements

### KYC Process Overview

1. **Document Submission**: Upload required documents
2. **Chainalysis Verification**: Automated risk assessment
3. **Manual Review**: Human verification if needed
4. **Approval/Rejection**: Final decision within 48 hours

### Step 1: Prepare Documents

#### Required Documents

**Identity Verification**:
- Government-issued photo ID (passport, driver's license)
- Must be current and clearly readable
- All corners and details visible

**Address Verification**:
- Utility bill (electricity, water, gas)
- Bank statement
- Government correspondence
- Must be dated within last 3 months

#### Document Requirements

- **Format**: PDF, JPG, PNG (max 10MB each)
- **Quality**: High resolution, all text readable
- **Completeness**: Full document visible, no cropping
- **Authenticity**: Original documents only, no screenshots

### Step 2: Submit KYC Application

1. Go to "KYC Verification" in your profile
2. Click "Start KYC Process"
3. Fill out personal information form:
   - Full legal name
   - Date of birth
   - Address
   - Phone number
   - Occupation
4. Upload required documents
5. Review and submit application

### Step 3: Chainalysis Verification

- **Automated Screening**: Risk assessment of your Bitcoin addresses
- **Sanctions Check**: Verification against global sanctions lists
- **PEP Screening**: Politically Exposed Person verification
- **Source of Funds**: Analysis of Bitcoin transaction history

### Step 4: Manual Review (If Required)

Some applications require human review:
- **High-Risk Jurisdictions**: Additional scrutiny
- **Large Commitments**: Enhanced due diligence
- **Unclear Documents**: Document quality issues
- **Risk Flags**: Chainalysis alerts

### Step 5: KYC Decision

#### Approval

- **Notification**: Email confirmation of approval
- **Increased Limits**: Unlimited BTC commitment
- **Enhanced Features**: Access to premium features
- **Compliance Status**: Ongoing monitoring

#### Rejection

- **Reason Provided**: Specific rejection reason
- **Resubmission**: Opportunity to correct issues
- **Appeal Process**: Formal appeal if disagreed
- **Alternative Options**: Remain in non-KYC tier

### KYC Maintenance

#### Ongoing Requirements

- **Annual Review**: Yearly KYC refresh
- **Document Updates**: Submit new documents when expired
- **Address Changes**: Update address within 30 days
- **Risk Monitoring**: Continuous Chainalysis screening

#### Compliance Violations

If compliance issues arise:
1. **Account Freeze**: Temporary restriction of activities
2. **Investigation**: Review of account activity
3. **Remediation**: Opportunity to resolve issues
4. **Termination**: Account closure if unresolved

## Wallet Integration Guide

### BlueWallet Integration

#### Setup Process

1. **Install BlueWallet**: Download from official app store
2. **Create/Import Wallet**: Set up Bitcoin wallet
3. **Enable Lightning**: Set up Lightning Network wallet
4. **Connect to Protocol**: Use QR code connection

#### Features

- **One-Click Connect**: Seamless protocol integration
- **Lightning Support**: Instant reward payments
- **Mobile Friendly**: Optimized for mobile use
- **Backup/Restore**: Easy wallet recovery

#### Troubleshooting

**Connection Issues**:
- Ensure latest BlueWallet version
- Check internet connectivity
- Verify QR code scanning
- Restart app if needed

**Lightning Issues**:
- Ensure Lightning wallet is funded
- Check channel capacity
- Verify node connectivity
- Contact BlueWallet support

### Ledger Hardware Wallet

#### Setup Process

1. **Initialize Ledger**: Set up hardware wallet
2. **Install Bitcoin App**: Use Ledger Live
3. **Connect to Computer**: USB connection
4. **Verify Address**: Confirm address on device
5. **Connect to Protocol**: Authorize connection

#### Security Benefits

- **Private Key Security**: Keys never leave device
- **Transaction Signing**: Secure transaction approval
- **PIN Protection**: Device-level security
- **Recovery Phrase**: Secure backup mechanism

#### Usage Tips

- **Keep Updated**: Regular firmware updates
- **Secure Storage**: Store device safely
- **Backup Recovery**: Secure recovery phrase storage
- **Test Transactions**: Verify with small amounts first

#### Troubleshooting

**Connection Issues**:
- Check USB cable and port
- Update Ledger Live software
- Restart Ledger device
- Try different USB port

**Transaction Issues**:
- Verify transaction details on device
- Check device battery level
- Ensure Bitcoin app is open
- Confirm address matches

### Multi-Wallet Management

#### Best Practices

1. **Primary Wallet**: Use for main commitments
2. **Backup Wallet**: Keep for emergencies
3. **Hardware Priority**: Prefer hardware wallets
4. **Regular Testing**: Test wallet functionality

#### Switching Wallets

1. **Claim Rewards**: Clear pending rewards
2. **Update Commitment**: Change to new wallet address
3. **Verify New Wallet**: Confirm new wallet control
4. **Test Small Amount**: Verify functionality

## Troubleshooting

### Common Issues

#### Commitment Problems

**Issue**: "Invalid ECDSA Proof"
- **Cause**: Incorrect signature or address mismatch
- **Solution**: Regenerate proof with correct wallet
- **Prevention**: Double-check Bitcoin address

**Issue**: "Oracle Verification Failed"
- **Cause**: Chainlink oracle connectivity issues
- **Solution**: Wait 60 seconds for retry
- **Prevention**: Ensure stable internet connection

**Issue**: "Commitment Limit Exceeded"
- **Cause**: Non-KYC user trying to commit > 1 BTC
- **Solution**: Complete KYC verification or reduce amount
- **Prevention**: Check limits before committing

#### Reward Problems

**Issue**: "Payment Processing Failed"
- **Cause**: Lightning Network or USDC payment issues
- **Solution**: Check wallet connectivity, try alternative payment
- **Prevention**: Test payment methods with small amounts

**Issue**: "Insufficient Rewards"
- **Cause**: No rewards accumulated yet
- **Solution**: Wait for staking rewards to accumulate
- **Prevention**: Check reward calculation schedule

#### Security Problems

**Issue**: "2FA Required"
- **Cause**: Security policy requires 2FA for operation
- **Solution**: Set up and verify 2FA
- **Prevention**: Enable 2FA during initial setup

**Issue**: "Account Locked"
- **Cause**: Multiple failed authentication attempts
- **Solution**: Wait for lockout period or contact support
- **Prevention**: Use correct credentials and 2FA

**Issue**: "Session Expired"
- **Cause**: Inactive session timeout
- **Solution**: Log in again with 2FA
- **Prevention**: Stay active or extend session

#### KYC Problems

**Issue**: "KYC Documents Rejected"
- **Cause**: Poor quality or invalid documents
- **Solution**: Resubmit with high-quality, valid documents
- **Prevention**: Follow document requirements carefully

**Issue**: "Chainalysis Risk Flag"
- **Cause**: High-risk Bitcoin address history
- **Solution**: Provide source of funds documentation
- **Prevention**: Use clean Bitcoin addresses

### Getting Help

#### Self-Service Resources

1. **FAQ**: Check frequently asked questions
2. **Documentation**: Review technical documentation
3. **Video Tutorials**: Watch step-by-step guides
4. **Community Forum**: Ask community questions

#### Support Channels

1. **Email Support**: support@vaultprotocol.com
2. **Discord Community**: Real-time chat support
3. **GitHub Issues**: Technical problems and bugs
4. **Emergency Contact**: For security incidents

#### Support Information to Provide

When contacting support, include:
- **User ID**: Your protocol user identifier
- **Transaction Hash**: Relevant transaction IDs
- **Error Messages**: Exact error text
- **Screenshots**: Visual evidence of issues
- **Steps to Reproduce**: How to recreate the problem

### Emergency Procedures

#### Suspected Security Breach

1. **Immediate**: Change all passwords
2. **Revoke**: All active sessions
3. **Enable**: Additional security measures
4. **Contact**: Support immediately
5. **Monitor**: Account activity closely

#### Lost Access

1. **2FA Recovery**: Use backup codes
2. **Wallet Recovery**: Use recovery phrase
3. **Account Recovery**: Contact support with verification
4. **Document**: Keep records of recovery process

#### Protocol Issues

1. **Check Status**: Visit status page
2. **Wait**: For automatic resolution
3. **Monitor**: Official communications
4. **Contact**: Support if issues persist

Remember: The Vault Protocol team will never ask for your private keys, recovery phrases, or passwords. Always verify communications through official channels.