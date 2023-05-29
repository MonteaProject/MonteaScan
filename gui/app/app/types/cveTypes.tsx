export type Detects = {
    detect: Detect[]
};

export type Detect = {
    time    : string,
    hostname: string,
    ip      : string[],
    os      : string,
    kernel  : string,
    oval    : Oval[]
};

export type Oval = {
    '@id'   : string,
    '@class': string,
    metadata: Metadata,
    criteria: Criteria
};

export type Metadata = {
    title      : string,
    affected   : Affected,
    reference  : Reference[],
    description: string,
    advisory   : Advisory
};

export type Affected = {
    '@family': string,
    platform : string[]
};

export type Reference = {
    '@ref_id' : string,
    '@ref_url': string,
    '@source' : string
};

export type Advisory = {
    "@from"          : string,
    severity         : string,
    rights           : string,
    issued           : Issued,
    updated          : Updated,
    cve              : Cve[],
    bugzilla         : Bugzilla[],
    affected_cpe_list: Cpe
};

export type Issued = {
    "@date": string
};

export type Updated = {
    "@date": string
};

export type Cve = {
    "@cvss2" : string,
    "@cvss3" : string,
    "@cwe"   : string,
    "@href"  : string,
    "@impact": string,
    "@public": string,
    "$value" : string
};

export type Bugzilla = {
    "@href" : string,
    "@id"   : string,
    "$value": string
};

export type Cpe = {
    cpe: string[]
};

export type Criteria = {
    "@operator": string,
    criterion  : Criterion[],
    criteria   : Criteria2[]
};

export type Criterion = {
    "@comment" : string,
    "@test_ref": string
};

export type Criteria2 = {
    "@operator": string,
    criterion  : Criterion2[]
};

export type Criterion2 = {
    "@comment" : string,
    "@test_ref": string
};