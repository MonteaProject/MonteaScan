import { VulnsList } from "../types/impactTypes";
import NextLink from "next/link";
import {
  Stat,
  StatLabel,
  StatNumber,
  StatHelpText,
  StatArrow,
  StatGroup,
  CircularProgress,
  CircularProgressLabel
} from "../common/components";

async function getVulnsList() {
  const res = await fetch("http://localhost:3000/api/vulnsList/", {cache: "no-store"});

  if (!res.ok) {
    throw new Error("Failed to fetch vulns list...");
  }

  const data = await res.json();
  return data as VulnsList;
}

type decrease = any;

export default async function VulnsList() {
  const v = await getVulnsList();
  const v1 = v.impact;
  const v2 = v.sum;

  let a: number = 30;
  let c: decrease = "increase";

  return (
    <div>
      <div>
        {v2.map((s) => (
        <StatGroup mb={4}>
          <Stat>
            <StatLabel>Total</StatLabel>
            <CircularProgress value={a} size='140px' thickness='10px' color='green.400'>
              <CircularProgressLabel>{s.total_sum}</CircularProgressLabel>
            </CircularProgress>
            <StatHelpText>
              <StatArrow type={c} />
              前回差異: 23件
            </StatHelpText>
          </Stat>
          
          <Stat>
            <StatLabel>Critical</StatLabel>
            <CircularProgress value={a} size='140px' thickness='10px' color='red.700'>
              <CircularProgressLabel>{s.critical_sum}</CircularProgressLabel>
            </CircularProgress>
            <StatHelpText>
              <StatArrow type='decrease' />
              前回差異: 23件
            </StatHelpText>
          </Stat>
        
          <Stat>
            <StatLabel>High</StatLabel>
            <CircularProgress value={a} size='140px' thickness='10px' color='orange.400'>
              <CircularProgressLabel>{s.important_sum}</CircularProgressLabel>
            </CircularProgress>
            <StatHelpText>
              <StatArrow type='increase' />
              前回差異: -23件
            </StatHelpText>
          </Stat>

          <Stat>
            <StatLabel>Medium</StatLabel>
            <CircularProgress value={a} size='140px' thickness='10px' color='yellow.400'>
              <CircularProgressLabel>{s.moderate_sum}</CircularProgressLabel>
            </CircularProgress>
            <StatHelpText>
              <StatArrow type='increase' />
              前回差異: -23件
            </StatHelpText>
          </Stat>

          <Stat>
            <StatLabel>Low</StatLabel>
            <CircularProgress value={a} size='140px' thickness='10px' color='blue.400'>
              <CircularProgressLabel>{s.low_sum}</CircularProgressLabel>
            </CircularProgress>
            <StatHelpText>
              <StatArrow type='increase' />
              前回差異: -23件
            </StatHelpText>
          </Stat>
        </StatGroup>
        ))}
      </div>
      <div>
        {v1.map((d) => (
        <StatGroup>
          <Stat>
            <StatLabel>{d.hostname}</StatLabel>
            <StatNumber>{d.total}</StatNumber>
            <StatHelpText>
              <StatArrow type={c} />
              前回差異: 23件
            </StatHelpText>
          </Stat>
          
          <Stat>
            <StatLabel>{d.hostname}</StatLabel>
            <StatNumber>{d.critical}</StatNumber>
            <StatHelpText>
              <StatArrow type='decrease' />
              前回差異: 23件
            </StatHelpText>
          </Stat>
        
          <Stat>
            <StatLabel>{d.hostname}</StatLabel>
            <StatNumber>{d.important}</StatNumber>
            <StatHelpText>
              <StatArrow type='increase' />
              前回差異: -23件
            </StatHelpText>
          </Stat>

          <Stat>
            <StatLabel>{d.hostname}</StatLabel>
            <StatNumber>{d.moderate}</StatNumber>
            <StatHelpText>
              <StatArrow type='increase' />
              前回差異: -23件
            </StatHelpText>
          </Stat>

          <Stat>
            <StatLabel>{d.hostname}</StatLabel>
            <StatNumber>{d.low}</StatNumber>
            <StatHelpText>
              <StatArrow type='increase' />
              前回差異: -23件
            </StatHelpText>
          </Stat>
        </StatGroup>
        ))}
      </div>
    </div>
  );
}