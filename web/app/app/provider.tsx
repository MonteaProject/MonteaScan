// Server Component の制限
//　・状態を持たない useState/Context
//　・ライフサイクルフックが使えない useEffect
//　・ブラウザのみ利用可能なAPIが使えない localStorage
//　・イベントハンドラーが使えない onClick/onChange

// ChakraProviderは内部でuseStateを利用しているため、Client Componentとして扱うために、"use client"を宣言したファイルでラップします。

// "use client";
// import { ChakraProvider } from "@chakra-ui/react";
// import React from "react";
import { ChakraProvider } from "./common/components";

export default function Provider({ children }:{ children: React.ReactNode }) {
    return <ChakraProvider>{children}</ChakraProvider>
}