import { LineChart }           from "./lineChart";
import { LineChart2 }          from "./lineChart2";
import { HorizontalBarChart }  from "./horizontalBarChart";
import { DoughnutChart }       from "./doughnutChart";
import { BubbleChart }         from "./bubbleChart";
import { MultiTypeChart }      from "./multiTypeChart";
import { ScatterChart }        from "./scatterChart";
import { PolarChart }          from "./polarChart";
import { RadarChart }          from "./radarChart";
import { MultiLineChart }      from "./multiLineChart";
import { AreaChart }           from "./areaChart";
import { VerticalBarChart }    from "./verticalBarChart";
import { HorizontalBarChart2 } from "./horizontalBarChart2";
import { StackedBarChart }     from "./stackedBarChart";
import { GroupedBarChart }     from "./groupedBarChart";

import { VulnsList } from "../types/impactTypes";
import NextLink from "next/link";
import {
  Stat,
  StatLabel,
  StatNumber,
  StatGroup,
  CircularProgress,
  CircularProgressLabel,
  Box,
  Flex,
  Tooltip,
  InfoIcon,
  InfoOutlineIcon,
  ViewIcon,
  Center
} from "../common/components";

async function getVulnsList() {
  const res = await fetch("http://localhost:3000/api/vulnsList/", {cache: "no-store"});

  if (!res.ok) {
    throw new Error("Failed to fetch vulns list...");
  }

  const data = await res.json();
  return data as VulnsList;
}

export default async function Dashboard() {
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
      <Box>
        {vSum.map((s, i) => (
        <StatGroup key={i} mb={5}>
          <Stat>
            <StatLabel fontSize="md">Total</StatLabel>
            <Center>
              <CircularProgress max={total_per} value={total_per} size='180px' thickness='16px' color='green.400'>
              <NextLink href="/server/">
                <CircularProgressLabel _hover={{color: 'green.400'}}>{s.totalTotal}</CircularProgressLabel>
              </NextLink>
              </CircularProgress>
            </Center>
          </Stat>
          
          <Stat>
            <StatLabel fontSize="md">Critical</StatLabel>
            <Center>
              <CircularProgress max={total_per} value={critical_per} size='180px' thickness='16px' color='red.700'>
              <NextLink href="/server/">
                <CircularProgressLabel _hover={{color: 'green.400'}}>{s.criticalTotal}</CircularProgressLabel>
              </NextLink>
              </CircularProgress>
            </Center>
          </Stat>
        
          <Stat>
            <StatLabel fontSize="md">High</StatLabel>
            <Center>
              <CircularProgress max={total_per} value={important_per} size='180px' thickness='16px' color='orange.400'>
              <NextLink href="/server/">
                <CircularProgressLabel _hover={{color: 'green.400'}}>{s.importantTotal}</CircularProgressLabel>
              </NextLink>
              </CircularProgress>
            </Center>
          </Stat>

          <Stat>
            <StatLabel fontSize="md">Medium</StatLabel>
            <Center>
              <CircularProgress max={total_per} value={moderate_per} size='180px' thickness='16px' color='yellow.400'>
              <NextLink href="/server/">
                <CircularProgressLabel _hover={{color: 'green.400'}}>{s.moderateTotal}</CircularProgressLabel>
              </NextLink>
              </CircularProgress>
            </Center>
          </Stat>

          <Stat>
            <StatLabel fontSize="md">Low</StatLabel>
            <Center>
              <CircularProgress max={total_per} value={low_per} size='180px' thickness='16px' color='blue.400'>
              <NextLink href="/server/">
                <CircularProgressLabel _hover={{color: 'green.400'}}>{s.lowTotal}</CircularProgressLabel>
              </NextLink>
              </CircularProgress>
            </Center>
          </Stat>
        </StatGroup>
        ))}
        
        {vImpact.map((d, i) => (
        <StatGroup key={i} mb={5}>
          <Stat>
            <StatLabel fontSize="lg">{d.hostname}</StatLabel>
            <NextLink href={`/info/${d.hostname}`}>
              <StatNumber as='abbr' _hover={{color: 'green.400'}}>{d.total}</StatNumber>
            </NextLink>
          </Stat>
          
          <Stat>
            <StatLabel fontSize="lg">{d.hostname}</StatLabel>
            <NextLink href={`/info/${d.hostname}`}>
              <StatNumber as='abbr' _hover={{color: 'green.400'}}>{d.critical}</StatNumber>
            </NextLink>
          </Stat>
        
          <Stat>
            <StatLabel fontSize="lg">{d.hostname}</StatLabel>
            <NextLink href={`/info/${d.hostname}`}>
              <StatNumber as='abbr' _hover={{color: 'green.400'}}>{d.important}</StatNumber>
            </NextLink>
          </Stat>

          <Stat>
            <StatLabel fontSize="lg">{d.hostname}</StatLabel>
            <NextLink href={`/info/${d.hostname}`}>
              <StatNumber as='abbr' _hover={{color: 'green.400'}}>{d.moderate}</StatNumber>
            </NextLink>
          </Stat>

          <Stat>
            <StatLabel fontSize="lg">{d.hostname}</StatLabel>
            <NextLink href={`/info/${d.hostname}`}>
              <StatNumber as='abbr' _hover={{color: 'green.400'}}>{d.low}</StatNumber>
            </NextLink>
          </Stat>
        </StatGroup>
        ))}
      </Box>
      
      <Flex mt="30px" mb="30px">
        <Box w="100%">
          <Tooltip label='test' fontSize='md'><InfoOutlineIcon mb="1" mr="1" /></Tooltip>
          <LineChart />
        </Box>
        <Box w="100%">
          <Tooltip label='test' fontSize='md'><InfoOutlineIcon mb="1" mr="1" /></Tooltip>
          <HorizontalBarChart />
        </Box>
      </Flex>
      <Flex mt="30px" mb="30px">
        <Box w="100%">
          <Tooltip label='test' fontSize='md'><InfoOutlineIcon mb="1" mr="1" /></Tooltip>
          <LineChart2 />
        </Box>
        <Box w="100%">
          <Tooltip label='test' fontSize='md'><InfoOutlineIcon mb="1" mr="1" /></Tooltip>
          <MultiTypeChart />
        </Box>
      </Flex>
      <Flex mt="30px" mb="30px">
        <Box w="100%">
          <Tooltip label='test' fontSize='md'><InfoOutlineIcon mb="1" mr="1" /></Tooltip>
          <DoughnutChart />
        </Box>
        <Box w="100%">
          <Tooltip label='test' fontSize='md'><InfoOutlineIcon mb="1" mr="1" /></Tooltip>
          <PolarChart />
        </Box>
        <Box w="100%">
          <Tooltip label='test' fontSize='md'><InfoOutlineIcon mb="1" mr="1" /></Tooltip>
          <RadarChart />
        </Box>
      </Flex>
      <Flex mt="30px" mb="30px">
        <Box w="100%">
          <Tooltip label='test' fontSize='md'><InfoOutlineIcon mb="1" mr="1" /></Tooltip>
          <MultiLineChart />
        </Box>
        <Box w="100%">
          <Tooltip label='test' fontSize='md'><InfoOutlineIcon mb="1" mr="1" /></Tooltip>
          <AreaChart />
        </Box>
      </Flex>
      <Flex mt="30px" mb="30px">
        <Box w="100%">
          <Tooltip label='test' fontSize='md'><InfoOutlineIcon mb="1" mr="1" /></Tooltip>
          <BubbleChart />
        </Box>
        <Box w="100%">
          <Tooltip label='test' fontSize='md'><InfoOutlineIcon mb="1" mr="1" /></Tooltip>
          <ScatterChart />
        </Box>
      </Flex>
      <Flex mt="30px" mb="30px">
        <Box w="100%">
          <Tooltip label='test' fontSize='md'><InfoOutlineIcon mb="1" mr="1" /></Tooltip>
          <VerticalBarChart />
        </Box>
        <Box w="100%">
          <Tooltip label='test' fontSize='md'><InfoOutlineIcon mb="1" mr="1" /></Tooltip>
          <HorizontalBarChart2 />
        </Box>
      </Flex>
      <Flex mt="30px" mb="30px">
        <Box w="100%">
          <Tooltip label='test' fontSize='md'><InfoOutlineIcon mb="1" mr="1" /></Tooltip>
          <StackedBarChart />
        </Box>
        <Box w="100%">
          <Tooltip label='test' fontSize='md'><InfoOutlineIcon mb="1" mr="1" /></Tooltip>
          <GroupedBarChart />
        </Box>
      </Flex>
    </Box>
  );
}