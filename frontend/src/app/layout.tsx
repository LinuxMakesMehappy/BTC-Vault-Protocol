import type { Metadata } from 'next';
import { Inter, JetBrains_Mono } from 'next/font/google';
import './globals.css';
import { VaultWalletProvider } from '@/components/WalletProvider';
import { I18nProvider } from '@/components/I18nProvider';
import { ToastProvider } from '@/components/ToastProvider';
import { Navigation } from '@/components/Navigation';

const inter = Inter({ subsets: ['latin'], variable: '--font-inter' });
const jetbrainsMono = JetBrains_Mono({
  subsets: ['latin'],
  variable: '--font-jetbrains-mono'
});

export const metadata: Metadata = {
  title: 'Vault Protocol - Non-Custodial Bitcoin Staking',
  description: 'A security-focused, non-custodial Bitcoin protocol that allows users to commit BTC without transferring custody while earning rewards from protocol-owned staking activities.',
  keywords: ['Bitcoin', 'DeFi', 'Staking', 'Non-Custodial', 'Solana', 'Multi-Chain'],
  authors: [{ name: 'Vault Protocol Team' }],
  viewport: 'width=device-width, initial-scale=1',
  themeColor: '#1a1a1a',
  openGraph: {
    title: 'Vault Protocol - Non-Custodial Bitcoin Staking',
    description: 'Earn staking rewards on your Bitcoin without giving up custody',
    type: 'website',
    locale: 'en_US',
  },
  twitter: {
    card: 'summary_large_image',
    title: 'Vault Protocol',
    description: 'Non-Custodial Bitcoin Staking Protocol',
  },
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en" className={`${inter.variable} ${jetbrainsMono.variable}`}>
      <body className="min-h-screen bg-gradient-to-br from-vault-primary to-vault-secondary text-white antialiased">
        <I18nProvider>
          <VaultWalletProvider>
            <ToastProvider>
              <div className="flex flex-col min-h-screen">
                <Navigation />
                <main className="flex-1">
                  {children}
                </main>
                <footer className="border-t border-gray-800 py-8">
                  <div className="container mx-auto px-4 text-center text-gray-400">
                    <p>&copy; 2024 Vault Protocol. All rights reserved.</p>
                    <p className="text-sm mt-2">
                      Non-custodial Bitcoin staking with enterprise-grade security
                    </p>
                  </div>
                </footer>
              </div>
            </ToastProvider>
          </VaultWalletProvider>
        </I18nProvider>
      </body>
    </html>
  );
}