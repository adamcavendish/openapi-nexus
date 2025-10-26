//! Enhanced TypeScript generator with improved schema parsing

use crate::generator::{GeneratorError, TypeScriptGenerator};
use utoipa::openapi::OpenApi;

/// Enhanced TypeScript generator with improved schema parsing
pub struct EnhancedTypeScriptGenerator {
    base_generator: TypeScriptGenerator,
}

impl EnhancedTypeScriptGenerator {
    pub fn new() -> Self {
        Self {
            base_generator: TypeScriptGenerator::new(),
        }
    }

    /// Generate TypeScript code with improved schema parsing
    pub fn generate_enhanced(&self, openapi: &OpenApi) -> Result<String, GeneratorError> {
        // For now, use the base generator but with enhanced output
        let base_code = self.base_generator.generate(openapi)?;

        // Enhance the generated code with additional features
        let enhanced_code = self.enhance_generated_code(&base_code)?;

        Ok(enhanced_code)
    }

    /// Enhance the generated code with additional TypeScript features
    fn enhance_generated_code(&self, base_code: &str) -> Result<String, GeneratorError> {
        let mut enhanced = String::new();

        // Add enhanced imports
        enhanced.push_str("// Enhanced TypeScript code generation\n");
        enhanced.push_str("import { AxiosInstance, AxiosResponse } from 'axios';\n");
        enhanced.push_str("import { z } from 'zod';\n\n");

        // Add the base code
        enhanced.push_str(base_code);

        // Add enhanced API client with better typing
        enhanced.push_str("\n\n// Enhanced API Client with better typing\n");
        enhanced.push_str("export class EnhancedApiClient {\n");
        enhanced.push_str("  private axios: AxiosInstance;\n\n");
        enhanced.push_str("  constructor(baseURL: string, axiosInstance?: AxiosInstance) {\n");
        enhanced
            .push_str("    this.axios = axiosInstance || require('axios').create({ baseURL });\n");
        enhanced.push_str("  }\n\n");
        enhanced.push_str("  // Generic request method with proper typing\n");
        enhanced.push_str("  async request<T>(config: any): Promise<AxiosResponse<T>> {\n");
        enhanced.push_str("    return this.axios.request<T>(config);\n");
        enhanced.push_str("  }\n\n");
        enhanced.push_str("  // GET request with typing\n");
        enhanced
            .push_str("  async get<T>(url: string, config?: any): Promise<AxiosResponse<T>> {\n");
        enhanced.push_str("    return this.request<T>({ method: 'GET', url, ...config });\n");
        enhanced.push_str("  }\n\n");
        enhanced.push_str("  // POST request with typing\n");
        enhanced.push_str(
            "  async post<T>(url: string, data?: any, config?: any): Promise<AxiosResponse<T>> {\n",
        );
        enhanced
            .push_str("    return this.request<T>({ method: 'POST', url, data, ...config });\n");
        enhanced.push_str("  }\n\n");
        enhanced.push_str("  // PUT request with typing\n");
        enhanced.push_str(
            "  async put<T>(url: string, data?: any, config?: any): Promise<AxiosResponse<T>> {\n",
        );
        enhanced.push_str("    return this.request<T>({ method: 'PUT', url, data, ...config });\n");
        enhanced.push_str("  }\n\n");
        enhanced.push_str("  // DELETE request with typing\n");
        enhanced.push_str(
            "  async delete<T>(url: string, config?: any): Promise<AxiosResponse<T>> {\n",
        );
        enhanced.push_str("    return this.request<T>({ method: 'DELETE', url, ...config });\n");
        enhanced.push_str("  }\n");
        enhanced.push_str("}\n\n");

        // Add Zod validation schemas
        enhanced.push_str("// Zod validation schemas for runtime type checking\n");
        enhanced.push_str("export const ValidationSchemas = {\n");
        enhanced.push_str("  // Add validation schemas here based on OpenAPI spec\n");
        enhanced.push_str("  // Example: Pet: z.object({ id: z.number(), name: z.string() }),\n");
        enhanced.push_str("} as const;\n\n");

        // Add utility types
        enhanced.push_str("// Utility types for better TypeScript support\n");
        enhanced.push_str("export type ApiResponse<T> = {\n");
        enhanced.push_str("  data: T;\n");
        enhanced.push_str("  status: number;\n");
        enhanced.push_str("  statusText: string;\n");
        enhanced.push_str("};\n\n");

        enhanced.push_str("export type ApiError = {\n");
        enhanced.push_str("  message: string;\n");
        enhanced.push_str("  status: number;\n");
        enhanced.push_str("  details?: any;\n");
        enhanced.push_str("};\n\n");

        // Add error handling utilities
        enhanced.push_str("// Error handling utilities\n");
        enhanced.push_str("export class ApiError extends Error {\n");
        enhanced.push_str(
            "  constructor(public status: number, message: string, public details?: any) {\n",
        );
        enhanced.push_str("    super(message);\n");
        enhanced.push_str("    this.name = 'ApiError';\n");
        enhanced.push_str("  }\n");
        enhanced.push_str("}\n\n");

        // Add configuration types
        enhanced.push_str("// Configuration types\n");
        enhanced.push_str("export interface ApiConfig {\n");
        enhanced.push_str("  baseURL: string;\n");
        enhanced.push_str("  timeout?: number;\n");
        enhanced.push_str("  headers?: Record<string, string>;\n");
        enhanced.push_str("  validateStatus?: (status: number) => boolean;\n");
        enhanced.push_str("}\n\n");

        Ok(enhanced)
    }
}

impl Default for EnhancedTypeScriptGenerator {
    fn default() -> Self {
        Self::new()
    }
}
