import React, { useState, useEffect, useCallback } from 'react';
import { useConnection, useWallet } from '@solana/wallet-adapter-react';
import { PublicKey, Transaction } from '@solana/web3.js';
import { BN } from '@project-serum/anchor';
import {
  createLiquidityEngineClient,
  LiquidityEngineClient,
  LiquidityEngineState,
  SwapParams,
  StakeParams,
  UnstakeParams,
  CrossChainParams,
} from '../lib/liquidity-engine';
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from './ui/card';
import { Button } from './ui/button';
import { Input } from './ui/input';
import { Label } from './ui/label';
import { Tabs, TabsContent, TabsList, TabsTrigger } from './ui/tabs';
import { Badge } from './ui/badge';
import { Progress } from './ui/progress';
import { Alert, AlertDescription } from './ui/alert';
import { Loader2, TrendingUp, ArrowUpDown, Bridge, Zap } from 'lucide-react';

interface TokenInfo {
  mint: string;
  symbol: string;
  name: string;
  decimals: number;
  logoURI?: string;
}

const POPULAR_TOKENS: TokenInfo[] = [
  {
    mint: 'So11111111111111111111111111111111111111112',
    symbol: 'SOL',
    name: 'Solana',
    decimals: 9,
  },
  {
    mint: '7Q2afV64in6N6SeZsAAB81TJzwDoD6zpqmHkzi9Dcavn',
    symbol: 'JSOL',
    name: 'Jupiter Staked SOL',
    decimals: 9,
  },
  {
    mint: 'EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v',
    symbol: 'USDC',
    name: 'USD Coin',
    decimals: 6,
  },
  {
    mint: 'Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB',
    symbol: 'USDT',
    name: 'Tether USD',
    decimals: 6,
  },
];

const SUPPORTED_CHAINS = [
  { id: 1, name: 'Ethereum', symbol: 'ETH' },
  { id: 56, name: 'BNB Smart Chain', symbol: 'BSC' },
  { id: 137, name: 'Polygon', symbol: 'MATIC' },
  { id: 43114, name: 'Avalanche', symbol: 'AVAX' },
];

export const LiquidityEngine: React.FC = () => {
  const { connection } = useConnection();
  const { publicKey, sendTransaction } = useWallet();
  
  const [client, setClient] = useState<LiquidityEngineClient | null>(null);
  const [engineState, setEngineState] = useState<LiquidityEngineState | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  
  // Swap state
  const [swapFromToken, setSwapFromToken] = useState<TokenInfo>(POPULAR_TOKENS[0]);
  const [swapToToken, setSwapToToken] = useState<TokenInfo>(POPULAR_TOKENS[1]);
  const [swapAmount, setSwapAmount] = useState('');
  const [swapQuote, setSwapQuote] = useState<any>(null);
  
  // Stake state
  const [stakeAmount, setStakeAmount] = useState('');
  const [jsolExchangeRate, setJsolExchangeRate] = useState(1.05);
  
  // Unstake state
  const [unstakeAmount, setUnstakeAmount] = useState('');
  
  // Cross-chain state
  const [bridgeAmount, setBridgeAmount] = useState('');
  const [targetChain, setTargetChain] = useState(SUPPORTED_CHAINS[0]);
  const [recipientAddress, setRecipientAddress] = useState('');
  const [bridgeEstimate, setBridgeEstimate] = useState<{
    estimatedTime: number;
    bridgeFee: BN;
  } | null>(null);

  // Initialize client
  useEffect(() => {
    const initClient = async () => {
      try {
        const liquidityClient = createLiquidityEngineClient(connection);
        await liquidityClient.initialize();
        setClient(liquidityClient);
        
        // Fetch engine state
        const state = await liquidityClient.getLiquidityEngineState();
        setEngineState(state);
        
        // Fetch JSOL exchange rate
        const rate = await liquidityClient.getJSOLExchangeRate();
        setJsolExchangeRate(rate);
      } catch (err) {
        console.error('Failed to initialize liquidity engine client:', err);
        setError('Failed to initialize liquidity engine');
      }
    };

    if (connection) {
      initClient();
    }
  }, [connection]);

  // Get swap quote
  const getSwapQuote = useCallback(async () => {
    if (!client || !swapAmount || !swapFromToken || !swapToToken) return;
    
    try {
      setLoading(true);
      const amount = new BN(parseFloat(swapAmount) * Math.pow(10, swapFromToken.decimals));
      const routes = await client.getJupiterQuote(
        new PublicKey(swapFromToken.mint),
        new PublicKey(swapToToken.mint),
        amount
      );
      
      if (routes.length > 0) {
        setSwapQuote(routes[0]);
      }
    } catch (err) {
      console.error('Failed to get swap quote:', err);
      setError('Failed to get swap quote');
    } finally {
      setLoading(false);
    }
  }, [client, swapAmount, swapFromToken, swapToToken]);

  // Execute swap
  const executeSwap = async () => {
    if (!client || !publicKey || !swapQuote) return;
    
    try {
      setLoading(true);
      const amount = new BN(parseFloat(swapAmount) * Math.pow(10, swapFromToken.decimals));
      const minAmountOut = swapQuote.outAmount.muln(0.99); // 1% slippage
      
      const swapParams: SwapParams = {
        inputMint: new PublicKey(swapFromToken.mint),
        outputMint: new PublicKey(swapToToken.mint),
        amount,
        slippageBps: 100, // 1%
        userPublicKey: publicKey,
      };
      
      const instruction = await client.swapViaJupiter(swapParams);
      const transaction = await client.buildTransaction([instruction], publicKey);
      
      const signature = await sendTransaction(transaction, connection);
      console.log('Swap transaction signature:', signature);
      
      // Reset form
      setSwapAmount('');
      setSwapQuote(null);
      setError(null);
    } catch (err) {
      console.error('Swap failed:', err);
      setError('Swap transaction failed');
    } finally {
      setLoading(false);
    }
  };

  // Execute stake
  const executeStake = async () => {
    if (!client || !publicKey || !stakeAmount) return;
    
    try {
      setLoading(true);
      const amount = new BN(parseFloat(stakeAmount) * Math.pow(10, 9)); // SOL has 9 decimals
      
      const stakeParams: StakeParams = {
        amount,
        userPublicKey: publicKey,
      };
      
      const instruction = await client.stakeToJSOL(stakeParams);
      const transaction = await client.buildTransaction([instruction], publicKey);
      
      const signature = await sendTransaction(transaction, connection);
      console.log('Stake transaction signature:', signature);
      
      setStakeAmount('');
      setError(null);
    } catch (err) {
      console.error('Stake failed:', err);
      setError('Stake transaction failed');
    } finally {
      setLoading(false);
    }
  };

  // Execute unstake
  const executeUnstake = async () => {
    if (!client || !publicKey || !unstakeAmount) return;
    
    try {
      setLoading(true);
      const amount = new BN(parseFloat(unstakeAmount) * Math.pow(10, 9)); // JSOL has 9 decimals
      
      const unstakeParams: UnstakeParams = {
        jsolAmount: amount,
        userPublicKey: publicKey,
      };
      
      const instruction = await client.instantUnstakeJSOL(unstakeParams);
      const transaction = await client.buildTransaction([instruction], publicKey);
      
      const signature = await sendTransaction(transaction, connection);
      console.log('Unstake transaction signature:', signature);
      
      setUnstakeAmount('');
      setError(null);
    } catch (err) {
      console.error('Unstake failed:', err);
      setError('Unstake transaction failed');
    } finally {
      setLoading(false);
    }
  };

  // Get bridge estimate
  const getBridgeEstimate = useCallback(async () => {
    if (!client || !bridgeAmount) return;
    
    try {
      const amount = new BN(parseFloat(bridgeAmount) * Math.pow(10, 9));
      const estimate = await client.estimateCrossChainTransfer(targetChain.id, amount);
      setBridgeEstimate(estimate);
    } catch (err) {
      console.error('Failed to get bridge estimate:', err);
    }
  }, [client, bridgeAmount, targetChain]);

  // Execute bridge
  const executeBridge = async () => {
    if (!client || !publicKey || !bridgeAmount || !recipientAddress) return;
    
    try {
      setLoading(true);
      const amount = new BN(parseFloat(bridgeAmount) * Math.pow(10, 9));
      
      // Convert recipient address to bytes (simplified)
      const recipient = new Uint8Array(32);
      const recipientBuffer = Buffer.from(recipientAddress.replace('0x', ''), 'hex');
      recipient.set(recipientBuffer.slice(0, 32));
      
      const bridgeParams: CrossChainParams = {
        amount,
        targetChain: targetChain.id,
        recipient,
        userPublicKey: publicKey,
      };
      
      const instruction = await client.crossChainBridge(bridgeParams);
      const transaction = await client.buildTransaction([instruction], publicKey);
      
      const signature = await sendTransaction(transaction, connection);
      console.log('Bridge transaction signature:', signature);
      
      setBridgeAmount('');
      setRecipientAddress('');
      setBridgeEstimate(null);
      setError(null);
    } catch (err) {
      console.error('Bridge failed:', err);
      setError('Bridge transaction failed');
    } finally {
      setLoading(false);
    }
  };

  // Debounced effects
  useEffect(() => {
    const timer = setTimeout(() => {
      if (swapAmount && swapFromToken && swapToToken) {
        getSwapQuote();
      }
    }, 500);
    return () => clearTimeout(timer);
  }, [swapAmount, swapFromToken, swapToToken, getSwapQuote]);

  useEffect(() => {
    const timer = setTimeout(() => {
      if (bridgeAmount && targetChain) {
        getBridgeEstimate();
      }
    }, 500);
    return () => clearTimeout(timer);
  }, [bridgeAmount, targetChain, getBridgeEstimate]);

  if (!publicKey) {
    return (
      <Card className="w-full max-w-2xl mx-auto">
        <CardContent className="p-6 text-center">
          <p>Please connect your wallet to use the Liquidity Engine</p>
        </CardContent>
      </Card>
    );
  }

  return (
    <div className="w-full max-w-4xl mx-auto space-y-6">
      {/* Header */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Zap className="h-6 w-6" />
            Primitive Liquidity Engine
          </CardTitle>
          <CardDescription>
            Cross-chain liquidity with Jupiter DEX integration and instant JSOL unstaking
          </CardDescription>
        </CardHeader>
        {engineState && (
          <CardContent>
            <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
              <div>
                <Label className="text-sm text-muted-foreground">Total Volume</Label>
                <p className="text-lg font-semibold">
                  ${(engineState.totalVolume.toNumber() / 1e9).toLocaleString()}
                </p>
              </div>
              <div>
                <Label className="text-sm text-muted-foreground">Total Fees</Label>
                <p className="text-lg font-semibold">
                  ${(engineState.totalFees.toNumber() / 1e9).toLocaleString()}
                </p>
              </div>
              <div>
                <Label className="text-sm text-muted-foreground">Fee Rate</Label>
                <p className="text-lg font-semibold">{engineState.feeRate / 100}%</p>
              </div>
              <div>
                <Label className="text-sm text-muted-foreground">Status</Label>
                <Badge variant={engineState.isPaused ? "destructive" : "default"}>
                  {engineState.isPaused ? "Paused" : "Active"}
                </Badge>
              </div>
            </div>
          </CardContent>
        )}
      </Card>

      {/* Error Alert */}
      {error && (
        <Alert variant="destructive">
          <AlertDescription>{error}</AlertDescription>
        </Alert>
      )}

      {/* Main Interface */}
      <Tabs defaultValue="swap" className="w-full">
        <TabsList className="grid w-full grid-cols-4">
          <TabsTrigger value="swap" className="flex items-center gap-2">
            <ArrowUpDown className="h-4 w-4" />
            Swap
          </TabsTrigger>
          <TabsTrigger value="stake" className="flex items-center gap-2">
            <TrendingUp className="h-4 w-4" />
            Stake
          </TabsTrigger>
          <TabsTrigger value="unstake">Unstake</TabsTrigger>
          <TabsTrigger value="bridge" className="flex items-center gap-2">
            <Bridge className="h-4 w-4" />
            Bridge
          </TabsTrigger>
        </TabsList>

        {/* Swap Tab */}
        <TabsContent value="swap">
          <Card>
            <CardHeader>
              <CardTitle>Token Swap via Jupiter</CardTitle>
              <CardDescription>
                Swap tokens with optimal routing and minimal slippage
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="space-y-2">
                <Label>From Token</Label>
                <select
                  value={swapFromToken.mint}
                  onChange={(e) => {
                    const token = POPULAR_TOKENS.find(t => t.mint === e.target.value);
                    if (token) setSwapFromToken(token);
                  }}
                  className="w-full p-2 border rounded"
                >
                  {POPULAR_TOKENS.map(token => (
                    <option key={token.mint} value={token.mint}>
                      {token.symbol} - {token.name}
                    </option>
                  ))}
                </select>
              </div>
              
              <div className="space-y-2">
                <Label>Amount</Label>
                <Input
                  type="number"
                  placeholder="0.0"
                  value={swapAmount}
                  onChange={(e) => setSwapAmount(e.target.value)}
                />
              </div>

              <div className="space-y-2">
                <Label>To Token</Label>
                <select
                  value={swapToToken.mint}
                  onChange={(e) => {
                    const token = POPULAR_TOKENS.find(t => t.mint === e.target.value);
                    if (token) setSwapToToken(token);
                  }}
                  className="w-full p-2 border rounded"
                >
                  {POPULAR_TOKENS.map(token => (
                    <option key={token.mint} value={token.mint}>
                      {token.symbol} - {token.name}
                    </option>
                  ))}
                </select>
              </div>

              {swapQuote && (
                <div className="p-4 bg-muted rounded-lg">
                  <p className="text-sm">
                    You will receive approximately{' '}
                    <span className="font-semibold">
                      {(swapQuote.outAmount.toNumber() / Math.pow(10, swapToToken.decimals)).toFixed(6)}
                    </span>{' '}
                    {swapToToken.symbol}
                  </p>
                  <p className="text-xs text-muted-foreground">
                    Price impact: {swapQuote.priceImpactPct?.toFixed(2)}%
                  </p>
                </div>
              )}

              <Button
                onClick={executeSwap}
                disabled={loading || !swapAmount || !swapQuote}
                className="w-full"
              >
                {loading ? <Loader2 className="h-4 w-4 animate-spin" /> : 'Swap Tokens'}
              </Button>
            </CardContent>
          </Card>
        </TabsContent>

        {/* Stake Tab */}
        <TabsContent value="stake">
          <Card>
            <CardHeader>
              <CardTitle>Stake SOL for JSOL</CardTitle>
              <CardDescription>
                Stake SOL to receive liquid staking tokens with instant liquidity
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="space-y-2">
                <Label>SOL Amount</Label>
                <Input
                  type="number"
                  placeholder="0.0"
                  value={stakeAmount}
                  onChange={(e) => setStakeAmount(e.target.value)}
                />
              </div>

              {stakeAmount && (
                <div className="p-4 bg-muted rounded-lg">
                  <p className="text-sm">
                    You will receive approximately{' '}
                    <span className="font-semibold">
                      {(parseFloat(stakeAmount) / jsolExchangeRate).toFixed(6)}
                    </span>{' '}
                    JSOL
                  </p>
                  <p className="text-xs text-muted-foreground">
                    Exchange rate: 1 JSOL = {jsolExchangeRate.toFixed(3)} SOL
                  </p>
                </div>
              )}

              <Button
                onClick={executeStake}
                disabled={loading || !stakeAmount}
                className="w-full"
              >
                {loading ? <Loader2 className="h-4 w-4 animate-spin" /> : 'Stake SOL'}
              </Button>
            </CardContent>
          </Card>
        </TabsContent>

        {/* Unstake Tab */}
        <TabsContent value="unstake">
          <Card>
            <CardHeader>
              <CardTitle>Instant Unstake JSOL</CardTitle>
              <CardDescription>
                Instantly convert JSOL back to SOL without waiting periods
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="space-y-2">
                <Label>JSOL Amount</Label>
                <Input
                  type="number"
                  placeholder="0.0"
                  value={unstakeAmount}
                  onChange={(e) => setUnstakeAmount(e.target.value)}
                />
              </div>

              {unstakeAmount && (
                <div className="p-4 bg-muted rounded-lg">
                  <p className="text-sm">
                    You will receive approximately{' '}
                    <span className="font-semibold">
                      {(parseFloat(unstakeAmount) * jsolExchangeRate).toFixed(6)}
                    </span>{' '}
                    SOL
                  </p>
                  <p className="text-xs text-muted-foreground">
                    Instant unstaking with current exchange rate
                  </p>
                </div>
              )}

              <Button
                onClick={executeUnstake}
                disabled={loading || !unstakeAmount}
                className="w-full"
              >
                {loading ? <Loader2 className="h-4 w-4 animate-spin" /> : 'Instant Unstake'}
              </Button>
            </CardContent>
          </Card>
        </TabsContent>

        {/* Bridge Tab */}
        <TabsContent value="bridge">
          <Card>
            <CardHeader>
              <CardTitle>Cross-Chain Bridge</CardTitle>
              <CardDescription>
                Transfer assets to other blockchain networks
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="space-y-2">
                <Label>Amount (SOL)</Label>
                <Input
                  type="number"
                  placeholder="0.0"
                  value={bridgeAmount}
                  onChange={(e) => setBridgeAmount(e.target.value)}
                />
              </div>

              <div className="space-y-2">
                <Label>Target Chain</Label>
                <select
                  value={targetChain.id}
                  onChange={(e) => {
                    const chain = SUPPORTED_CHAINS.find(c => c.id === parseInt(e.target.value));
                    if (chain) setTargetChain(chain);
                  }}
                  className="w-full p-2 border rounded"
                >
                  {SUPPORTED_CHAINS.map(chain => (
                    <option key={chain.id} value={chain.id}>
                      {chain.name} ({chain.symbol})
                    </option>
                  ))}
                </select>
              </div>

              <div className="space-y-2">
                <Label>Recipient Address</Label>
                <Input
                  placeholder="0x..."
                  value={recipientAddress}
                  onChange={(e) => setRecipientAddress(e.target.value)}
                />
              </div>

              {bridgeEstimate && (
                <div className="p-4 bg-muted rounded-lg space-y-2">
                  <p className="text-sm">
                    Bridge Fee:{' '}
                    <span className="font-semibold">
                      {(bridgeEstimate.bridgeFee.toNumber() / 1e9).toFixed(6)} SOL
                    </span>
                  </p>
                  <p className="text-sm">
                    Estimated Time:{' '}
                    <span className="font-semibold">
                      {Math.round(bridgeEstimate.estimatedTime / 60)} minutes
                    </span>
                  </p>
                  <Progress 
                    value={100 - (bridgeEstimate.estimatedTime / 900) * 100} 
                    className="w-full" 
                  />
                </div>
              )}

              <Button
                onClick={executeBridge}
                disabled={loading || !bridgeAmount || !recipientAddress}
                className="w-full"
              >
                {loading ? <Loader2 className="h-4 w-4 animate-spin" /> : 'Bridge Assets'}
              </Button>
            </CardContent>
          </Card>
        </TabsContent>
      </Tabs>
    </div>
  );
};