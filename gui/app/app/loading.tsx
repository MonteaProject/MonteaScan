import { Box, Spinner } from "./common/components";

export default function Loading() {
  return (
    <Box justifyContent="center" display="flex">
      <Spinner color="green.300" size="lg" />
    </Box>
  );
}