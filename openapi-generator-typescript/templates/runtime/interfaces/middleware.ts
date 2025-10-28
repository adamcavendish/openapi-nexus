export interface Middleware {
    pre?: (context: RequestContext) => Promise<FetchParams | void>;
    post?: (context: ResponseContext) => Promise<Response | void>;
    onError?: (context: ErrorContext) => Promise<Response | void>;
}
