export interface RequestTask {
    title: string;
    description: string;
    difficulty: number;
}

export interface ResponseTask {
    id: number;
    title: string;
    description: string;
    difficulty: number;
    finished: boolean;
}
