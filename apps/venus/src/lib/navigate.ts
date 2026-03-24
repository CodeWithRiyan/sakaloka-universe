export const navigateReplaceSearchParams = (searchParams: {
  [key: string]: string
}) => {
  return [
    {
      ...searchParams,
    },
    {
      replace: true,
    },
  ]
}
