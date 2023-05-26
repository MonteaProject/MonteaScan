"use client";
import { useEffect } from "react";
import { Heading, Button, Box } from "./common/components";

export default function Error ({
    error,
    reset,
}: {
    error: Error;
    reset: () => void;
}) {
    useEffect(() => {
        console.error(error);
    }, [error]);

    return (
        <Box>
            <Heading mb={4}>情報の取得に失敗しました</Heading>
            <Button onClick={() => reset()}>Try again</Button>
        </Box>
    );
}