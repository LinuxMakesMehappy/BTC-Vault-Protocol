import type { Metadata } from 'next';
import { Inter } from 'next/font/google';
import './globals.css';
import { VaultWalletProvider } from '../components/WalletProvider';
import '../i18n/config';

const inter = Inter({ subsets: ['latin'] });

export const metadata: Metadata = {
  title: 'Vault Protocol',
  description: 'Non-custodial Bitcoin commitment protocol with staking rewards',
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body className={inter.className}>
        <VaultWalletProvider>
          {children}
        </VaultWalletProvider>
      </body>
    </html>
  );
}