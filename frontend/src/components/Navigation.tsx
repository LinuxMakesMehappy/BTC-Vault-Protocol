'use client';

import { useState } from 'react';
import Link from 'next/link';
import { usePathname } from 'next/navigation';
import { useTranslation } from 'react-i18next';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import { Menu, X, Globe, Bitcoin, Shield, Gift, Settings } from 'lucide-react';
import { LanguageSelector } from './LanguageSelector';

export function Navigation() {
    const [isOpen, setIsOpen] = useState(false);
    const pathname = usePathname();
    const { t } = useTranslation();

    const navigation = [
        { name: 'Home', href: '/', icon: Bitcoin },
        { name: 'Dashboard', href: '/dashboard', icon: Settings },
        { name: t('rewards.title'), href: '/rewards', icon: Gift },
        { name: t('kyc.title'), href: '/kyc', icon: Shield },
        { name: t('security.title'), href: '/security', icon: Settings },
    ];

    return (
        <nav className="bg-vault-primary/95 backdrop-blur-sm border-b border-gray-800 sticky top-0 z-50">
            <div className="container mx-auto px-4">
                <div className="flex justify-between items-center h-16">
                    {/* Logo */}
                    <Link href="/" className="flex items-center space-x-2">
                        <div className="w-8 h-8 bg-gradient-to-br from-vault-accent to-orange-400 rounded-lg flex items-center justify-center">
                            <Bitcoin className="w-5 h-5 text-white" />
                        </div>
                        <span className="text-xl font-bold text-gradient">
                            Vault Protocol
                        </span>
                    </Link>

                    {/* Desktop Navigation */}
                    <div className="hidden md:flex items-center space-x-8">
                        {navigation.map((item) => {
                            const Icon = item.icon;
                            const isActive = pathname === item.href;

                            return (
                                <Link
                                    key={item.name}
                                    href={item.href}
                                    className={`flex items-center space-x-2 ${isActive ? 'nav-link-active' : 'nav-link'
                                        }`}
                                >
                                    <Icon className="w-4 h-4" />
                                    <span>{item.name}</span>
                                </Link>
                            );
                        })}
                    </div>

                    {/* Desktop Actions */}
                    <div className="hidden md:flex items-center space-x-4">
                        <LanguageSelector />
                        <WalletMultiButton className="!bg-vault-accent hover:!bg-orange-600 !rounded-lg !font-medium !transition-colors" />
                    </div>

                    {/* Mobile menu button */}
                    <div className="md:hidden flex items-center space-x-2">
                        <LanguageSelector />
                        <button
                            onClick={() => setIsOpen(!isOpen)}
                            className="text-gray-300 hover:text-white p-2"
                            aria-label={isOpen ? "Close menu" : "Open menu"}
                            aria-expanded={isOpen}
                        >
                            {isOpen ? <X className="w-6 h-6" /> : <Menu className="w-6 h-6" />}
                        </button>
                    </div>
                </div>

                {/* Mobile Navigation */}
                {isOpen && (
                    <div className="md:hidden py-4 border-t border-gray-800">
                        <div className="flex flex-col space-y-2">
                            {navigation.map((item) => {
                                const Icon = item.icon;
                                const isActive = pathname === item.href;

                                return (
                                    <Link
                                        key={item.name}
                                        href={item.href}
                                        className={`flex items-center space-x-3 px-3 py-2 rounded-md ${isActive
                                            ? 'text-vault-accent bg-vault-accent/10'
                                            : 'text-gray-300 hover:text-white hover:bg-vault-secondary/50'
                                            }`}
                                        onClick={() => setIsOpen(false)}
                                    >
                                        <Icon className="w-5 h-5" />
                                        <span>{item.name}</span>
                                    </Link>
                                );
                            })}

                            <div className="pt-4 border-t border-gray-800">
                                <WalletMultiButton className="!w-full !bg-vault-accent hover:!bg-orange-600 !rounded-lg !font-medium !transition-colors" />
                            </div>
                        </div>
                    </div>
                )}
            </div>
        </nav>
    );
}