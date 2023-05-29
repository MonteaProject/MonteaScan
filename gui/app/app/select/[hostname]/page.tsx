import Info from "../../components/info";
import { Pkg } from "../../types/pkgTypes";
import { Box } from "../../common/components";
import { notFound } from "next/navigation";


const getServerInfo = async (hostname: string) => {
    const res = await fetch(
        `http://localhost:3000/api/serverInfo/${hostname}`, {cache: "no-store"}
    );

    if (res.status === 404) {
        notFound();
    }

    if (!res.ok) {
        throw new Error("Failed to fetch server infomation...");
    }

    const data = await res.json();
    return data.pkg as Pkg[];
};

// const getHogeHoge = async (hostname: string) => {
//     const res = await fetch(
//         `http://localhost:3000/api/serverinfo/${hostname}`, {cache: "no-store"}
//     );

//     if (res.status === 404) {
//         notFound();
//     }

//     if (!res.ok) {
//         throw new Error("Failed to fetch server infomation...");
//     }

//     const data = await res.json();
//     return data as Pkg[];
// };


export default async function ArticleDetail({
    params,
}: {
    params: { hostname: string };
}) {
    const infoPromise = getServerInfo(params.hostname);
    // const hogehogePromise = getHogeHoge(params.hostname);

    const info = await infoPromise;

    return (
        <Box>
            {/* @ts-expect-error 現状は jsx が Promise を返すと TypeScript が型エラーを報告するが、将来的には解決される */}
            <Info infoPromises={info} />
        </Box>
    );
}