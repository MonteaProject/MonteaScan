import { Container } from "./common/components";

export default function Main({ children }: { children: React.ReactNode }) {
  return (
    <Container as="main" maxW="container.xl" my="4" minH="calc(100vh - 115px - 2rem)">
      {children}
    </Container>
  );
}