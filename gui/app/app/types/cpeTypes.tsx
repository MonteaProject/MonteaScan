export type rhelCPE = {
  score                    : string,
  cveId                    : string,
  attackVector_value       : string,
  attackComplexity_value   : string,
  privilegesRequired_value : string,
  userInteraction_value    : string,
  scope_value              : string,
  confidentiality_value    : string,
  integrity_value          : string,
  availability_value       : string,
}

export type cpeVec = {
  cpe     : string,
  kind    : string,
  vendor  : string,
  product : string,
  version : string,
  update  : string,
  edition : string,
  language: string,
}