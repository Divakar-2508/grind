export interface Task {
    id: number;
    title: string;
    repeat: boolean;
    desc: string | null;
    link: string | null;
    [task : string]: unknown;
}