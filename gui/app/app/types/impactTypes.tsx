export type VulnsList = {
    impact: Impact[],
    sum: Sum[],
    last: Diff[]
};

export type Impact = {
    hostname: string,
    total: number,
    critical: number,
    important: number,
    moderate: number,
    low: number,
    hostDiff: HostDiff[]
};

export type Sum = {
    total_sum: number,
    critical_sum: number,
    important_sum: number,
    moderate_sum: number,
    low_sum: number
};

export type Diff = {
    total_diff: number,
    critical_diff: number,
    important_diff: number,
    moderate_diff: number,
    low_diff: number
};

export type HostDiff = {
    totalHostDiff: number,
    criticalHostDiff: number,
    importantHostDiff: number,
    moderateHostDiff: number,
    lowHostDiff: number
}