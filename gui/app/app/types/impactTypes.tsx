export type VulnsList = {
    impact: Impact[],
    sum   : Sum[]
};

export type Impact = {
    hostname : string,
    total    : number,
    critical : number,
    important: number,
    moderate : number,
    low      : number
};

export type Sum = {
    totalTotal    : number,
    criticalTotal : number,
    importantTotal: number,
    moderateTotal : number,
    lowTotal      : number
};