import { Box, Flex, Heading, Button, Grid } from "./common/components";
import NextLink from "next/link";
import Image from 'next/image';

export default function Header() {
  return (
    <Box as="header">
      <Flex
        align="center"
        minH={"60px"}
        color="green.300"
        bg="white"
        borderBottom={1}
        borderStyle="solid"
        borderColor="gray.200"
      >
        {/* <Flex flex={1} justify="space-between" maxW="8xl" mx="auto"> */}
          <Box pl={10}>
            <NextLink href="/">
              <Image src="/icon.png" width={64} height={64} alt="icon" />
            </NextLink>
          </Box>
          <Box pl={2}>
            <Heading size='lg' fontSize='24px'>
              <NextLink href="/">
                MonteaScan
              </NextLink>
            </Heading>
          </Box>
        {/* </Flex> */}
      </Flex>
    </Box>
  );
}