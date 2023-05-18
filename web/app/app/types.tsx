export type Host = {
    hostname: string;
    os: string;
    kernel: string;
    slug: string;
    time: string;
};

export type Comment = {
    id: number;
    body: string;
    articleId: number;
    createdAt: string;
    updatedAt: string;
    author: Author;
};

export type Author = {
    name: string;
    avatarUrl: string;
};