import { VulnsList } from "../types/impactTypes";
import NextLink from "next/link";
import {
  Stat,
  StatLabel,
  StatNumber,
  StatGroup,
  CircularProgress,
  CircularProgressLabel,
  Box
} from "../common/components";

async function getVulnsList() {
  const res = await fetch("http://localhost:3000/api/vulnsList/", {cache: "no-store"});

  if (!res.ok) {
    throw new Error("Failed to fetch vulns list...");
  }

  const data = await res.json();
  return data as VulnsList;
}

export default async function VulnsList() {
  const v       = await getVulnsList();
  const vImpact = v.impact;
  const vSum    = v.sum;

  let total_per     = vSum[0].totalTotal     / vSum[0].totalTotal;
  let critical_per  = vSum[0].criticalTotal  / vSum[0].totalTotal;
  let important_per = vSum[0].importantTotal / vSum[0].totalTotal;
  let moderate_per  = vSum[0].moderateTotal  / vSum[0].totalTotal;
  let low_per       = vSum[0].lowTotal       / vSum[0].totalTotal;

  return (
    <Box>
      {vSum.map((s) => (
      <StatGroup mb={4}>
        <Stat>
          <StatLabel>Total</StatLabel>
          <CircularProgress max={total_per} value={total_per} size='140px' thickness='10px' color='green.400'>
          <NextLink href="/components/">
            <CircularProgressLabel _hover={{color: 'green.400'}}>{s.totalTotal}</CircularProgressLabel>
          </NextLink>
          </CircularProgress>
        </Stat>
        
        <Stat>
          <StatLabel>Critical</StatLabel>
          <CircularProgress max={total_per} value={critical_per} size='140px' thickness='10px' color='red.700'>
          <NextLink href="/components/">
            <CircularProgressLabel _hover={{color: 'green.400'}}>{s.criticalTotal}</CircularProgressLabel>
          </NextLink>
          </CircularProgress>
        </Stat>
      
        <Stat>
          <StatLabel>High</StatLabel>
          <CircularProgress max={total_per} value={important_per} size='140px' thickness='10px' color='orange.400'>
          <NextLink href="/components/">
            <CircularProgressLabel _hover={{color: 'green.400'}}>{s.importantTotal}</CircularProgressLabel>
          </NextLink>
          </CircularProgress>
        </Stat>

        <Stat>
          <StatLabel>Medium</StatLabel>
          <CircularProgress max={total_per} value={moderate_per} size='140px' thickness='10px' color='yellow.400'>
          <NextLink href="/components/">
            <CircularProgressLabel _hover={{color: 'green.400'}}>{s.moderateTotal}</CircularProgressLabel>
          </NextLink>
          </CircularProgress>
        </Stat>

        <Stat>
          <StatLabel>Low</StatLabel>
          <CircularProgress max={total_per} value={low_per} size='140px' thickness='10px' color='blue.400'>
          <NextLink href="/components/">
            <CircularProgressLabel _hover={{color: 'green.400'}}>{s.lowTotal}</CircularProgressLabel>
          </NextLink>
          </CircularProgress>
        </Stat>
      </StatGroup>
      ))}
      
      {vImpact.map((d) => (
      <StatGroup>
        <Stat>
          <StatLabel>{d.hostname}</StatLabel>
          <NextLink href={`/select/${d.hostname}`}>
            <StatNumber _hover={{color: 'green.400'}}>{d.total}</StatNumber>
          </NextLink>
        </Stat>
        
        <Stat>
          <StatLabel>{d.hostname}</StatLabel>
          <NextLink href={`/select/${d.hostname}`}>
            <StatNumber _hover={{color: 'green.400'}}>{d.critical}</StatNumber>
          </NextLink>
        </Stat>
      
        <Stat>
          <StatLabel>{d.hostname}</StatLabel>
          <NextLink href={`/select/${d.hostname}`}>
            <StatNumber _hover={{color: 'green.400'}}>{d.important}</StatNumber>
          </NextLink>
        </Stat>

        <Stat>
          <StatLabel>{d.hostname}</StatLabel>
          <NextLink href={`/select/${d.hostname}`}>
            <StatNumber _hover={{color: 'green.400'}}>{d.moderate}</StatNumber>
          </NextLink>
        </Stat>

        <Stat>
          <StatLabel>{d.hostname}</StatLabel>
          <NextLink href={`/select/${d.hostname}`}>
            <StatNumber _hover={{color: 'green.400'}}>{d.low}</StatNumber>
          </NextLink>
        </Stat>
      </StatGroup>
      ))}
    </Box>
  );
}