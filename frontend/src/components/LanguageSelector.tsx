'use client';

import { Fragment } from 'react';
import { useTranslation } from 'react-i18next';
import { Listbox, Transition } from '@headlessui/react';
import { Globe, ChevronDown, Check } from 'lucide-react';
import { clsx } from 'clsx';

const languages = [
    { code: 'en', name: 'English', flag: '🇺🇸' },
    { code: 'es', name: 'Español', flag: '🇪🇸' },
    { code: 'zh', name: '中文', flag: '🇨🇳' },
    { code: 'ja', name: '日本語', flag: '🇯🇵' },
];

export function LanguageSelector() {
    const { i18n } = useTranslation();

    const currentLanguage = languages.find(lang => lang.code === i18n.language) || languages[0];

    const changeLanguage = (languageCode: string) => {
        i18n.changeLanguage(languageCode);
    };

    return (
        <Listbox value={currentLanguage.code} onChange={changeLanguage}>
            <div className="relative">
                <Listbox.Button className="relative flex items-center space-x-2 bg-vault-secondary hover:bg-gray-700 border border-gray-600 rounded-lg px-3 py-2 text-sm transition-colors">
                    <Globe className="w-4 h-4 text-gray-400" />
                    <span className="hidden sm:block">{currentLanguage.name}</span>
                    <span className="sm:hidden">{currentLanguage.flag}</span>
                    <ChevronDown className="w-4 h-4 text-gray-400" />
                </Listbox.Button>

                <Transition
                    as={Fragment}
                    leave="transition ease-in duration-100"
                    leaveFrom="opacity-100"
                    leaveTo="opacity-0"
                >
                    <Listbox.Options className="absolute right-0 mt-2 w-48 bg-vault-secondary border border-gray-600 rounded-lg shadow-lg z-50">
                        <div className="py-1">
                            {languages.map((language) => (
                                <Listbox.Option
                                    key={language.code}
                                    value={language.code}
                                    className={({ active }) =>
                                        clsx(
                                            'relative cursor-pointer select-none py-2 px-3 flex items-center space-x-3',
                                            active ? 'bg-vault-accent/10 text-vault-accent' : 'text-gray-300'
                                        )
                                    }
                                >
                                    {({ selected }) => (
                                        <>
                                            <span className="text-lg">{language.flag}</span>
                                            <span className="flex-1">{language.name}</span>
                                            {selected && (
                                                <Check className="w-4 h-4 text-vault-accent" />
                                            )}
                                        </>
                                    )}
                                </Listbox.Option>
                            ))}
                        </div>
                    </Listbox.Options>
                </Transition>
            </div>
        </Listbox>
    );
}