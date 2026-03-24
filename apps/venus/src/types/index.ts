/**
 * Recursively convert `null` in union types to `undefined`.
 * Generated OpenAPI types use `T | null` for optional fields,
 * but the frontend codebase expects `T | undefined`.
 */
export type StripNull<T> = T extends null
  ? undefined
  : T extends (infer U)[]
    ? StripNull<U>[]
    : T extends Record<string, unknown>
      ? { [K in keyof T]: StripNull<T[K]> }
      : T

export interface DefaultSearchParams {
  page?: number
  limit?: number
  search?: string
  sortBy?: string
  sortOrder?: string
}

export interface DefaultResponse {
  success: boolean
  message: string
  error?: string
  errors?: string[]
}

export interface ResponseError {
  error: {
    status: number
    data: {
      success: boolean
      error: string
      errors?: string[]
    }
  }
  isUnhandledError: false
}

export interface ImageMetadata {
  filename: string
  url: string
  bucket: string
  size: number
  contentType: string
}
