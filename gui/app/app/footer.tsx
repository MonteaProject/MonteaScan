import { Container, Box, Text, VStack, Flex } from "./common/components";

export default function Footer() {
  return (
    <Box bg="gray.50" color="gray.700" as="footer">
      <Flex
        align="center"
        py={3}
        color="gray.500"
        bg="gray.50"
        borderTop={1}
        borderStyle="solid"
        borderColor="gray.200"
      >
        <Container maxW="5xl" py={4}>
          <VStack justify='center'>
            <Text as="small">© 2023 MonteaProject</Text>
          </VStack>
        </Container>
      </Flex>
    </Box>
  );
}