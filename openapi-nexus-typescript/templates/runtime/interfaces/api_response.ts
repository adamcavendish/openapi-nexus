export interface ApiResponse<T> {
    raw: Response;
    value: () => Promise<T>;
}
