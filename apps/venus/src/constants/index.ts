export const INIT_PAGINATION_PARAMS = {
  page: 1,
  limit: 10,
}

export const INIT_PAGINATION_META = {
  total: 0,
  page: INIT_PAGINATION_PARAMS.page,
  limit: INIT_PAGINATION_PARAMS.limit,
  hasNext: false,
  hasPrev: false,
  totalPages: 1,
}

export const BASE_URL = import.meta.env.VITE_BASE_URL || "http://localhost:3000"
export const API_BASE_URL =
  import.meta.env.VITE_API_BASE_URL || "http://localhost:3000/api"

export const DEFAULT_LOGIN_REDIRECT_URL = "/dashboard"

export const CLOSE_INPUT_FORM_WARNING =
  "Data yang kamu masukkan akan hilang, apakah kamu yakin ingin melanjutkan?"
