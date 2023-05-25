import { Box, Flex, Heading, Button, Grid } from "./common/components";
import NextLink from "next/link";

export default function Header() {
  return (
    <Box as="header">
      <Flex
        align="center"
        minH={"60px"}
        color="green.400"
        bg="white"
        borderBottom={1}
        borderStyle="solid"
        borderColor="gray.200"
      >
        <Flex flex={1} justify="space-between" maxW="8xl" mx="auto">
          <Heading as='h4' size='md' fontFamily='Menlo'>
            <NextLink href="/">MonteaScan</NextLink>
          </Heading>
        </Flex>
      </Flex>
    </Box>
  );
}