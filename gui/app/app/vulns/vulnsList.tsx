// import './serverList.scss'
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

type decrease = any;

async function getVulnsList() {
  const res = await fetch("http://localhost:3000/api/vulnsList/", {cache: "no-store"});

  if (!res.ok) {
    throw new Error("Failed to fetch vulns list...");
  }

  const data = await res.json();
  return data as Impact[];
}

export default async function VulnsList() {
  const v = await getVulnsList();

  let a: number = 30;
  let b: string = "5";
  let c: decrease = "increase";
  
  return (
    <div>
      {v.map((d) => (
        <div>
          <p>合計: {d.total}</p>
          <p>重要: {d.critical}</p>
          <p>高: {d.important}</p>
          <p>中: {d.moderate}</p>
          <p>小: {d.low}</p>
        </div>
      ))}
      <StatGroup>
        <Stat>
          <StatLabel>Total</StatLabel>
          <StatNumber>35</StatNumber>
          <StatHelpText>
            <StatArrow type={c} />
            前回差異: 23件
          </StatHelpText>
        </Stat>
        
        <Stat>
          <StatLabel>Critical</StatLabel>
          <StatNumber>35</StatNumber>
          <StatHelpText>
            <StatArrow type='decrease' />
            前回差異: 23件
          </StatHelpText>
        </Stat>
      
        <Stat>
          <StatLabel>High</StatLabel>
          <StatNumber>35</StatNumber>
          <StatHelpText>
            <StatArrow type='increase' />
            前回差異: -23件
          </StatHelpText>
        </Stat>

        <Stat>
          <StatLabel>Medium</StatLabel>
          <StatNumber>35</StatNumber>
          <StatHelpText>
            <StatArrow type='increase' />
            前回差異: -23件
          </StatHelpText>
        </Stat>

        <Stat>
          <StatLabel>Low</StatLabel>
          <StatNumber>35</StatNumber>
          <StatHelpText>
            <StatArrow type='increase' />
            前回差異: -23件
          </StatHelpText>
        </Stat>
      </StatGroup>

      <StatGroup>
        <Stat>
          <StatLabel>Total</StatLabel>
          <CircularProgress value={a} size='140px' thickness='10px' color='green.400'>
            <CircularProgressLabel>{b}</CircularProgressLabel>
          </CircularProgress>
          <StatHelpText>
            <StatArrow type={c} />
            前回差異: 23件
          </StatHelpText>
        </Stat>
        
        <Stat>
          <StatLabel>Critical</StatLabel>
          <CircularProgress value={a} size='140px' thickness='10px' color='red.700'>
            <CircularProgressLabel>{b}</CircularProgressLabel>
          </CircularProgress>
          <StatHelpText>
            <StatArrow type='decrease' />
            前回差異: 23件
          </StatHelpText>
        </Stat>
      
        <Stat>
          <StatLabel>High</StatLabel>
          <CircularProgress value={a} size='140px' thickness='10px' color='orange.400'>
            <CircularProgressLabel>{b}</CircularProgressLabel>
          </CircularProgress>
          <StatHelpText>
            <StatArrow type='increase' />
            前回差異: -23件
          </StatHelpText>
        </Stat>

        <Stat>
          <StatLabel>Medium</StatLabel>
          <CircularProgress value={a} size='140px' thickness='10px' color='yellow.400'>
            <CircularProgressLabel>{b}</CircularProgressLabel>
          </CircularProgress>
          <StatHelpText>
            <StatArrow type='increase' />
            前回差異: -23件
          </StatHelpText>
        </Stat>

        <Stat>
          <StatLabel>Low</StatLabel>
          <CircularProgress value={a} size='140px' thickness='10px' color='blue.400'>
            <CircularProgressLabel>{b}</CircularProgressLabel>
          </CircularProgress>
          <StatHelpText>
            <StatArrow type='increase' />
            前回差異: -23件
          </StatHelpText>
        </Stat>
      </StatGroup>
    </div>
  );
}