"use client";
import { FormLabel, Input, Button, VStack, FormControl, FormErrorMessage } from "../common/components";
import { useState } from "react";
 
export default async function ServerList() {
    const [name, setName] = useState('');
    const [password, setPassword] = useState('');

    const isNameError = name === '';
    const isPasswdError = password === '';

    const handleClick = () => {
        console.log({ name, password });
        setName('');
        setPassword('');
    }

    return (
        <VStack>
            <VStack w="30vw">
                {/* <FormControl isInvalid={isNameError}> */}
                    <FormLabel htmlFor="name">First name</FormLabel>
                    <Input
                        id="name"
                        placeholder="name"
                        value={name}
                        onChange={(e) => setName(e.target.value)}
                    />
                    {/* <FormErrorMessage>Email is required.</FormErrorMessage> */}
                {/* </FormControl> */}

                {/* <FormControl isInvalid={isPasswdError}> */}
                    <FormLabel htmlFor="password">Password</FormLabel>
                    <Input
                        id="password"
                        placeholder="password"
                        type="password"
                        value={password}
                        onChange={(e) => setPassword(e.target.value)}
                    />
                    {/* <FormErrorMessage>Passwd is required.</FormErrorMessage>
                </FormControl> */}

                <Button mt={4} colorScheme="teal" onClick={handleClick}>
                    Submit
                </Button>
            </VStack>
        </VStack>
    )
}