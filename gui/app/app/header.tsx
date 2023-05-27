import { Box, Flex, Heading, Button, Grid } from "./common/components";
import NextLink from "next/link";
import Image from 'next/image';

export default function Header() {
  return (
    <Box as="header">
    <Flex align="center" minH={"60px"} color="green.300" bg="white" borderBottom={1} borderStyle="solid" borderColor="gray.200">
      <Box pl={10}>
        <NextLink href="/">
          <Image src="/icon.png" width={64} height={64} alt="icon" />
        </NextLink>
      </Box>
      <Box>
        <Heading fontSize='12px'>
          <NextLink href="/components/">
            サーバ一覧
          </NextLink>
        </Heading>
      </Box>
    </Flex>
    </Box>
  );
}