'use client';

import { motion } from 'framer-motion';
import { LucideIcon } from 'lucide-react';

interface FeatureCardProps {
    icon: LucideIcon;
    title: string;
    description: string;
    color: string;
    delay?: number;
}

export function FeatureCard({ icon: Icon, title, description, color, delay = 0 }: FeatureCardProps) {
    return (
        <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.5, delay }}
            className="card group hover:scale-105 transition-all duration-300 hover:border-vault-accent/30"
        >
            <div className="flex items-center space-x-3 mb-4">
                <div className={`p-2 rounded-lg bg-vault-secondary/50 group-hover:bg-vault-accent/10 transition-colors`}>
                    <Icon className={`w-6 h-6 ${color} group-hover:text-vault-accent transition-colors`} />
                </div>
                <h3 className="text-xl font-semibold text-white group-hover:text-vault-accent transition-colors">
                    {title}
                </h3>
            </div>

            <p className="text-gray-300 leading-relaxed group-hover:text-gray-200 transition-colors">
                {description}
            </p>
        </motion.div>
    );
}