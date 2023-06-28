export type Config = {
    server: EachConfig[]
};

export type EachConfig = {
    user: string,
    host: string,
    port: string,
    key : string,
    os  : string,
};