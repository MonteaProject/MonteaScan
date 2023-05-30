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
            <Heading as='h4' size='md'>情報の取得に失敗しました</Heading>
            <Button onClick={() => reset()}>Try again</Button>
        </Box>
    );
}