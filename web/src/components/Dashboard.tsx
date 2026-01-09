import React, { useEffect, useState } from 'react';
import { DataTrustBanner } from './DataTrustBanner';
import { Activity, Zap, AlertTriangle, HardDrive, Cpu, TrendingUp } from 'lucide-react';

interface EfficiencyEntry {
    hardware: string;
    encoder: string;
    codec: string;
    res: string;
    speed: number;
    reduction: number;
    samples: number;
}

interface StabilityEntry {
    encoder: string;
    error: string;
    count: number;
}

interface StatsResponse {
    coverage: {
        total_jobs: number;
        unique_hardware: number;
    };
    leaderboard: EfficiencyEntry[];
    stability: StabilityEntry[];
}

export const Dashboard: React.FC = () => {
    const [data, setData] = useState<StatsResponse | null>(null);
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        const fetchStats = async () => {
            try {
                const res = await fetch('/api/v1/stats/insights');
                if (res.ok) {
                    const json = await res.json();
                    setData(json);
                }
            } catch (error) {
                console.error("Failed to fetch stats", error);
            } finally {
                setLoading(false);
            }
        };

        fetchStats();
        // Poll every 30s
        const interval = setInterval(fetchStats, 30000);
        return () => clearInterval(interval);
    }, []);

    if (loading) return <div className="p-12 text-center text-helios-ink-muted">Loading Argument Engine...</div>;
    if (!data) return <div className="p-12 text-center text-status-error">Failed to load telemetry.</div>;

    return (
        <div className="space-y-8">
            <DataTrustBanner />

            <div className="grid grid-cols-1 md:grid-cols-4 gap-8">
                {/* Main Content: Efficiency Leaderboard */}
                <div className="md:col-span-3 space-y-6">
                    <div className="flex items-center justify-between">
                        <h2 className="text-xl font-bold text-helios-ink flex items-center gap-2">
                            <TrendingUp size={20} className="text-status-success" />
                            Efficiency Leaderboard
                        </h2>
                        <span className="text-xs text-helios-ink-muted font-mono bg-helios-panel px-2 py-1 rounded">
                            {data.coverage.total_jobs} SAMPLES / {data.coverage.unique_hardware} CONFIGS
                        </span>
                    </div>

                    <div className="bg-helios-panel border border-helios-border rounded-lg overflow-hidden">
                        <div className="grid grid-cols-12 gap-4 p-4 border-b border-helios-border text-xs font-mono text-helios-ink-muted font-bold uppercase tracking-wider">
                            <div className="col-span-4">Setup (Hardware + Encoder)</div>
                            <div className="col-span-2">Codec / Res</div>
                            <div className="col-span-3">Speed (x Realtime)</div>
                            <div className="col-span-3">Size Reduction</div>
                        </div>
                        <div className="divide-y divide-helios-border/50">
                            {data.leaderboard.length === 0 ? (
                                <div className="p-8 text-center text-helios-ink-muted italic">No efficiency data collected yet.</div>
                            ) : (
                                data.leaderboard.map((entry, idx) => (
                                    <div key={idx} className="grid grid-cols-12 gap-4 p-4 items-center hover:bg-helios-elevated/50 transition-colors">
                                        <div className="col-span-4">
                                            <div className="text-sm font-bold text-helios-ink">{entry.hardware}</div>
                                            <div className="text-xs text-helios-ink-muted font-mono">{entry.encoder}</div>
                                        </div>
                                        <div className="col-span-2 text-xs font-mono text-helios-ink-muted">
                                            <span className="text-helios-solar">{entry.codec.toUpperCase()}</span>
                                            <span className="mx-1">/</span>
                                            {entry.res}
                                        </div>
                                        <div className="col-span-3">
                                            <div className="flex items-center gap-2">
                                                <div className="flex-1 h-2 bg-helios-elevated rounded-full overflow-hidden">
                                                    <div
                                                        className="h-full bg-status-success rounded-full"
                                                        style={{ width: `${Math.min(100, (entry.speed / 10) * 100)}%` }} // Assume 10x is max for vis
                                                    ></div>
                                                </div>
                                                <span className="text-xs font-mono font-bold text-status-success w-12 text-right">{entry.speed.toFixed(2)}x</span>
                                            </div>
                                        </div>
                                        <div className="col-span-3">
                                            <div className="flex items-center gap-2">
                                                <div className="flex-1 h-2 bg-helios-elevated rounded-full overflow-hidden">
                                                    <div
                                                        className="h-full bg-helios-solar rounded-full"
                                                        style={{ width: `${Math.min(100, entry.reduction * 100)}%` }}
                                                    ></div>
                                                </div>
                                                <span className="text-xs font-mono font-bold text-helios-solar w-12 text-right">{(entry.reduction * 100).toFixed(0)}%</span>
                                            </div>
                                        </div>
                                    </div>
                                ))
                            )}
                        </div>
                    </div>
                </div>

                {/* Sidebar: Stability Report */}
                <div className="space-y-6">
                    <h2 className="text-xl font-bold text-helios-ink flex items-center gap-2">
                        <AlertTriangle size={20} className="text-status-warning" />
                        Stability Report
                    </h2>

                    <div className="space-y-4">
                        {data.stability.length === 0 ? (
                            <div className="p-4 rounded border border-helios-border bg-helios-panel/50 text-center text-xs text-helios-ink-muted">
                                No failures reported.
                            </div>
                        ) : (
                            data.stability.map((entry, idx) => (
                                <div key={idx} className="p-3 rounded border border-helios-border bg-helios-panel/50">
                                    <div className="flex justify-between items-start mb-2">
                                        <div className="text-sm font-bold text-helios-ink">{entry.encoder}</div>
                                        <div className="text-xs font-mono font-bold text-status-warning bg-status-warning/10 px-1.5 py-0.5 rounded">
                                            {entry.count}
                                        </div>
                                    </div>
                                    <div className="flex items-center gap-2 text-xs text-status-error">
                                        <AlertTriangle size={12} />
                                        <span className="font-mono uppercase">{entry.error}</span>
                                    </div>
                                </div>
                            ))
                        )}
                    </div>

                    <div className="p-4 rounded-lg bg-helios-panel/30 border border-helios-border/50">
                        <h3 className="text-xs font-bold text-helios-ink-muted uppercase mb-2">About Comparisons</h3>
                        <p className="text-xs text-helios-ink-muted leading-relaxed">
                            Efficiency score is calculated based on size reduction percentage relative to realtime encoding speed.
                            <br /><br />
                            Stability tracks reported engine failures normalized by total attempts.
                        </p>
                    </div>
                </div>
            </div>
        </div>
    );
};
