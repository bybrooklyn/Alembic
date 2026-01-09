import React from 'react';
import { ShieldCheck, AlertTriangle } from 'lucide-react';

export const DataTrustBanner: React.FC = () => {
    return (
        <div className="mb-8 p-4 rounded-lg border border-helios-border bg-helios-panel/30 backdrop-blur-sm">
            <div className="flex items-start gap-4">
                <div className="p-2 rounded-full bg-helios-solar/10 text-helios-solar">
                    <ShieldCheck size={24} />
                </div>
                <div>
                    <h3 className="text-helios-ink font-bold text-lg mb-1">Data Trust & Transparency</h3>
                    <p className="text-helios-ink-muted text-sm leading-relaxed mb-3">
                        Alembic is a community reference reference. We collect strictly anonymous performance metrics to build this argument engine.
                    </p>
                    <div className="grid grid-cols-1 md:grid-cols-2 gap-4 text-xs">
                        <div className="flex flex-col gap-1">
                            <span className="text-status-success font-mono font-semibold">✓ WHAT WE COLLECT</span>
                            <span className="text-helios-ink-muted">Hardware Model, Encoder Settings, Speed, Size, Duration</span>
                        </div>
                        <div className="flex flex-col gap-1">
                            <span className="text-status-error font-mono font-semibold">✕ WHAT WE DO NOT COLLECT</span>
                            <span className="text-helios-ink-muted">Filenames, File Paths, IP Addresses, User IDs, Media Titles</span>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
};
