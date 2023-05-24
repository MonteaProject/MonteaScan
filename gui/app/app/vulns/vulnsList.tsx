import { Impact } from "../types/impactTypes";
import NextLink from "next/link";
import { CircularProgress, CircularProgressLabel } from "../common/components";
import {
  Stat,
  StatLabel,
  StatNumber,
  StatHelpText,
  StatArrow,
  StatGroup,
} from "../common/components";

async function getVulnsList() {
  const res = await fetch("http://localhost:3000/api/vulnsList/", {cache: "no-store"});

  if (!res.ok) {
    throw new Error("Failed to fetch vulns list...");
  }

  const data = await res.json();
  return data as Impact[];
}

type decrease = any;

export default async function VulnsList() {
  const v = await getVulnsList();

  let a: number = 30;
  let c: decrease = "increase";

  return (
    <div>
      {v.map((d) => (
        <div>
          <StatGroup mb={4}>
            <Stat>
              <StatLabel>Total</StatLabel>
              <CircularProgress value={a} size='140px' thickness='10px' color='green.400'>
                <CircularProgressLabel>{d.total}</CircularProgressLabel>
              </CircularProgress>
              <StatHelpText>
                <StatArrow type={c} />
                前回差異: 23件
              </StatHelpText>
            </Stat>
            
            <Stat>
              <StatLabel>Critical</StatLabel>
              <CircularProgress value={a} size='140px' thickness='10px' color='red.700'>
                <CircularProgressLabel>{d.critical}</CircularProgressLabel>
              </CircularProgress>
              <StatHelpText>
                <StatArrow type='decrease' />
                前回差異: 23件
              </StatHelpText>
            </Stat>
          
            <Stat>
              <StatLabel>High</StatLabel>
              <CircularProgress value={a} size='140px' thickness='10px' color='orange.400'>
                <CircularProgressLabel>{d.important}</CircularProgressLabel>
              </CircularProgress>
              <StatHelpText>
                <StatArrow type='increase' />
                前回差異: -23件
              </StatHelpText>
            </Stat>

            <Stat>
              <StatLabel>Medium</StatLabel>
              <CircularProgress value={a} size='140px' thickness='10px' color='yellow.400'>
                <CircularProgressLabel>{d.moderate}</CircularProgressLabel>
              </CircularProgress>
              <StatHelpText>
                <StatArrow type='increase' />
                前回差異: -23件
              </StatHelpText>
            </Stat>

            <Stat>
              <StatLabel>Low</StatLabel>
              <CircularProgress value={a} size='140px' thickness='10px' color='blue.400'>
                <CircularProgressLabel>{d.low}</CircularProgressLabel>
              </CircularProgress>
              <StatHelpText>
                <StatArrow type='increase' />
                前回差異: -23件
              </StatHelpText>
            </Stat>
          </StatGroup>

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
        </div>
      ))}
    </div>
  );
}