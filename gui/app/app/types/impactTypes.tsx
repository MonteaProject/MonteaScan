export type VulnsList = {
    impact: Impact[],
    sum: Sum[]
};

export type Impact = {
    hostname: string,
    total: number,
    critical: number,
    important: number,
    moderate: number,
    low: number
};

export type Sum = {
    total_sum: number,
    critical_sum: number,
    important_sum: number,
    moderate_sum: number,
    low_sum: number
};