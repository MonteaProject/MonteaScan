import NextLink from "next/link";
import Image from 'next/image';
import { 
  Box,
  Flex,
  Heading,
  Stack,
  HStack,
  VStack
} from "./common/components";

export default function Header() {
  return (
    <Flex align="center" minH={"60px"} color="green.300" bg="white" borderBottom={1} borderStyle="solid" borderColor="gray.200">
      <HStack direction={["column", "row"]} spacing="24px">
        <Box pl={10}>
          <NextLink href="/">
            <Image src="/icon.png" width={64} height={64} alt="icon" />
          </NextLink>
        </Box>
        <Box w="60px" h="60px" pt={6}>
          <Heading fontSize='12px'>
            <NextLink href="/components/">サーバ一覧</NextLink>
          </Heading>
        </Box>
        <Box w="60px" h="60px" pt={6}>
          <Heading fontSize="12px">
            <NextLink href="/settings/">設定</NextLink>
          </Heading>
        </Box>
      </HStack>
    </Flex>
  );
}