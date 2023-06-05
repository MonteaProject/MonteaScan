export type Server = {
    hostname : string,
    os       : string,
    kernel   : string,
    time     : string,
    total    : number,
    critical : number,
    important: number,
    moderate : number,
    low      : number
};