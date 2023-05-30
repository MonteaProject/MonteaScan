// Server Component の制限事項
//   1. 状態を持たないため、useState/Context が使えない
//   2. ライフサイクルフック、useEffect が使えない
//   3. ブラウザAPI、localStorage が使えない
//   4. イベントハンドラー、onClick/onChange が使えない

// ChakraProvider は内部で useState を利用しているため、Client Component として扱う必要があるため、
// "use client"を宣言してラップします。
// ChakraUI を利用するたびに、"use client"を宣言する手間を省くため、
// ChakraUI のコンポーネントをここでexportする。

"use client";
export * from "@chakra-ui/react";
export * from "@chakra-ui/icons";