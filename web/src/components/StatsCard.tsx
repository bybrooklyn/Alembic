import React from 'react';

interface StatsCardProps {
    title: string;
    value: string | number;
    subtext?: string;
    icon?: React.ReactNode;
}

export const StatsCard: React.FC<StatsCardProps> = ({ title, value, subtext, icon }) => {
    return (
        <div className="p-6 rounded-lg bg-helios-panel border border-helios-border">
            <div className="flex justify-between items-start mb-2">
                <h3 className="text-helios-ink-muted text-sm font-medium uppercase tracking-wider">{title}</h3>
                {icon && <div className="text-helios-solar">{icon}</div>}
            </div>
            <div className="text-3xl font-bold text-helios-ink font-mono">{value}</div>
            {subtext && <div className="text-xs text-helios-ink-muted mt-2">{subtext}</div>}
        </div>
    );
};
