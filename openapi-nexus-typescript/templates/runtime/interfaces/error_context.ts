export interface ErrorContext {
    fetch: FetchAPI;
    url: string;
    init: RequestInit;
    error: any;
    response?: Response;
}
