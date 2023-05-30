import { ChakraProvider } from "./common/components";

export default function Provider({ children }:{ children: React.ReactNode }) {
    return (
        <ChakraProvider>
            {children}
        </ChakraProvider>
    );
}