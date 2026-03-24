const BRANCH_KEY = "branch"
interface LocalBranch {
  id: string
  name: string
  address: string
  type: string
}

export function setBranch(organization: LocalBranch): void {
  localStorage.setItem(BRANCH_KEY, JSON.stringify(organization))
}

export function getBranch(): LocalBranch {
  const branch = localStorage.getItem(BRANCH_KEY)
  return JSON.parse(branch || "{}")
}

export function clearBranch(): void {
  localStorage.removeItem(BRANCH_KEY)
}
