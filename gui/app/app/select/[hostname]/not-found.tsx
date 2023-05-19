import { Heading, Button } from "../../common/components";
import NextLink from "next/link";

export default function NotFound() {
  return (
    <div>
      <Heading mb={4}>404 Not Found</Heading>
      <Button as={NextLink} href="/">
        トップへ戻る
      </Button>
    </div>
  );
}