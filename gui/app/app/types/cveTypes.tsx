export type Vulns = {
  time        : string,
  hostname    : string,
  ip          : string[],
  os          : string,
  kernel      : string,
  cveid       : string,
  impact      : string,
  cvssv3_oval : string,
  cwe_oval    : string,
  issued      : string,
  updated     : string,
  pkgname     : string,
  pkgver      : string,
  pkgrelease  : string,
  update_flag : string,
  upver       : string,
  uprelease   : string,
  pkgarch     : string,
  cwe_name    : string,
  cwe_url     : string[],
  oval        : Oval,
}

export type Oval = {
  title:       string,
  family:      string,
  platform:    string[],
  description: string,
  reference:   Reference[],
  cpe:         string[],
  cve:         Cve,
  cvss:        Cvss,
  advisory:    Advisory,
  bugzilla:    Bugzilla[],
}

export type Reference = {
  ref_id:  string,
  ref_url: string,
  source:  string,
}

export type Cve = {
  score:  string,
  cwe:    string,
  href:   string,
  impact: string,
  public: string,
}

export type Cvss = {
  score:  string,
  vector: string,
}

export type Advisory = {
  from:     string,
  severity: string,
  rights:   string,
  issued:   string,
  updated:  string,
}

export type Bugzilla = {
  href:        string,
  id:          string,
  description: string,
}