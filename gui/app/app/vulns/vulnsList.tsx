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
  const v = await getVulnsList();
  const v1 = v.impact;
  const v2 = v.sum;
  const v3 = v.last;

  let total_per = v2[0].total_sum / v2[0].total_sum;
  let critical_per = v2[0].critical_sum / v2[0].total_sum;
  let important_per = v2[0].important_sum / v2[0].total_sum;
  let moderate_per = v2[0].moderate_sum / v2[0].total_sum;
  let low_per = v2[0].low_sum / v2[0].total_sum;

  type increase = any;
  type decrease = any;

  let increase: increase = "increase";
  let decrease: decrease = "decrease";
  let increase_color = "red.400";
  let decrease_color = "green.400";

  let stay = "";
  let up = <StatArrow type={increase} color={increase_color} />;
  let down = <StatArrow type={decrease} color={decrease_color} />;

  let icon1: JSX.Element;
  let icon2: JSX.Element;
  let icon3: JSX.Element;
  let icon4: JSX.Element;
  let icon5: JSX.Element;

  if (Math.sign(v3[0].total_diff) === 1) {
    let icon1 = up;
  } else if (Math.sign(v3[0].total_diff) === -1) {
    let icon1 = down;
  } else {
    let icon1 = stay;
  }

  if (Math.sign(v3[0].critical_diff) === 1) {
    let icon2 = up;
  } else if (Math.sign(v3[0].critical_diff) === -1) {
    let icon2 = down;
  } else {
    let icon2 = stay;
  }

  if (Math.sign(v3[0].important_diff) === 1) {
    let icon3 = up;
  } else if (Math.sign(v3[0].important_diff) === -1) {
    let icon3 = down;
  } else {
    let icon3 = stay;
  }

  if (Math.sign(v3[0].moderate_diff) === 1) {
    let icon4 = up;
  } else if (Math.sign(v3[0].moderate_diff) === -1) {
    let icon4 = down;
  } else {
    let icon4 = stay;
  }

  if (Math.sign(v3[0].low_diff) === 1) {
    let icon5 = up;
  } else if (Math.sign(v3[0].low_diff) === -1) {
    let icon5 = down;
  } else {
    let icon5 = stay;
  }

  const Judg = (isNew: number) => {
    if (Math.sign(isNew) === 1) {
      return up
    } else if (Math.sign(isNew) === -1) {
      return down
    } else {
      return stay
    }
  }

  return (
    <Box>
      {v2.map((s) => (
      <StatGroup mb={4}>
        <Stat>
          <StatLabel>Total</StatLabel>
          <CircularProgress max={total_per} value={total_per} size='140px' thickness='10px' color='green.400'>
            <CircularProgressLabel>{s.total_sum}</CircularProgressLabel>
          </CircularProgress>
          <StatHelpText>
            {icon1}
            前回差異: {v3[0].total_diff}件
          </StatHelpText>
        </Stat>
        
        <Stat>
          <StatLabel>Critical</StatLabel>
          <CircularProgress max={total_per} value={critical_per} size='140px' thickness='10px' color='red.700'>
            <CircularProgressLabel>{s.critical_sum}</CircularProgressLabel>
          </CircularProgress>
          <StatHelpText>
            {icon2}
            前回差異: {v3[0].critical_diff}件
          </StatHelpText>
        </Stat>
      
        <Stat>
          <StatLabel>High</StatLabel>
          <CircularProgress max={total_per} value={important_per} size='140px' thickness='10px' color='orange.400'>
            <CircularProgressLabel>{s.important_sum}</CircularProgressLabel>
          </CircularProgress>
          <StatHelpText>
            {icon3}
            前回差異: {v3[0].important_diff}件
          </StatHelpText>
        </Stat>

        <Stat>
          <StatLabel>Medium</StatLabel>
          <CircularProgress max={total_per} value={moderate_per} size='140px' thickness='10px' color='yellow.400'>
            <CircularProgressLabel>{s.moderate_sum}</CircularProgressLabel>
          </CircularProgress>
          <StatHelpText>
            {icon4}
            前回差異: {v3[0].moderate_diff}件
          </StatHelpText>
        </Stat>

        <Stat>
          <StatLabel>Low</StatLabel>
          <CircularProgress max={total_per} value={low_per} size='140px' thickness='10px' color='blue.400'>
            <CircularProgressLabel>{s.low_sum}</CircularProgressLabel>
          </CircularProgress>
          <StatHelpText>
            {icon5}
            前回差異: {v3[0].low_diff}件
          </StatHelpText>
        </Stat>
      </StatGroup>
      ))}
      
      {v1.map((d) => (
      <StatGroup>
        <Stat>
          <StatLabel>{d.hostname}</StatLabel>
          <StatNumber>{d.total}</StatNumber>
          <StatHelpText>
            <Judg isNew={d.hostDiff[0].totalHostDiff} />
            前回差異: {d.hostDiff[0].totalHostDiff}件
          </StatHelpText>
        </Stat>
        
        <Stat>
          <StatLabel>{d.hostname}</StatLabel>
          <StatNumber>{d.critical}</StatNumber>
          <StatHelpText>
            <Judg isNew={d.hostDiff[0].criticalHostDiff} />
            前回差異: {d.hostDiff[0].criticalHostDiff}件
          </StatHelpText>
        </Stat>
      
        <Stat>
          <StatLabel>{d.hostname}</StatLabel>
          <StatNumber>{d.important}</StatNumber>
          <StatHelpText>
            <Judg isNew={d.hostDiff[0].importantHostDiff} />
            前回差異: {d.hostDiff[0].importantHostDiff}件
          </StatHelpText>
        </Stat>

        <Stat>
          <StatLabel>{d.hostname}</StatLabel>
          <StatNumber>{d.moderate}</StatNumber>
          <StatHelpText>
            <Judg isNew={d.hostDiff[0].moderateHostDiff} />
            前回差異: {d.hostDiff[0].moderateHostDiff}件
          </StatHelpText>
        </Stat>

        <Stat>
          <StatLabel>{d.hostname}</StatLabel>
          <StatNumber>{d.low}</StatNumber>
          <StatHelpText>
            <Judg isNew={d.hostDiff[0].lowHostDiff} />
            前回差異: {d.hostDiff[0].lowHostDiff}件
          </StatHelpText>
        </Stat>
      </StatGroup>
      ))}
    </Box>
  );
}